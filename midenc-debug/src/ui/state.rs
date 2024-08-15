use std::{rc::Rc, sync::Arc};

use miden_assembly::Library;
use miden_core::{utils::Deserializable, FieldElement};
use miden_processor::{Felt, Program, StackInputs};
use midenc_session::{
    diagnostics::{IntoDiagnostic, Report, SourceSpan, Span, WrapErr},
    InputType, Session,
};

use crate::{
    Breakpoint, BreakpointType, DebugExecutor, ExecutionTrace, ProgramInputs, ReadMemoryExpr,
};

pub struct State {
    pub program: Arc<Program>,
    pub inputs: ProgramInputs,
    pub executor: DebugExecutor,
    pub execution_trace: ExecutionTrace,
    pub execution_failed: Option<miden_processor::ExecutionError>,
    pub session: Rc<Session>,
    pub input_mode: InputMode,
    pub breakpoints: Vec<Breakpoint>,
    pub breakpoints_hit: Vec<Breakpoint>,
    pub next_breakpoint_id: u8,
    pub stopped: bool,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum InputMode {
    #[default]
    Normal,
    Insert,
    Command,
}

impl State {
    pub fn from_inputs(
        inputs: Option<ProgramInputs>,
        mut args: Vec<Felt>,
        session: Rc<Session>,
    ) -> Result<Self, Report> {
        let mut inputs = inputs.unwrap_or_default();
        if !args.is_empty() {
            args.reverse();
            inputs.inputs = StackInputs::new(args).into_diagnostic()?;
        }
        let args = inputs.inputs.values().iter().copied().rev().collect::<Vec<_>>();
        let program = load_program(&session)?;

        let mut executor = crate::Executor::new(args.clone());
        executor.with_advice_inputs(inputs.advice_inputs.clone());
        for link_library in session.options.link_libraries.iter() {
            let lib = link_library.load(&session)?;
            executor.with_library(&lib);
        }

        let executor = executor.into_debug(&program, &session);

        // Execute the program until it terminates to capture a full trace for use during debugging
        let mut trace_executor = crate::Executor::new(args);
        trace_executor.with_advice_inputs(inputs.advice_inputs.clone());
        for link_library in session.options.link_libraries.iter() {
            let lib = link_library.load(&session)?;
            trace_executor.with_library(&lib);
        }

        let execution_trace = trace_executor.capture_trace(&program, &session);

        Ok(Self {
            program,
            inputs,
            executor,
            execution_trace,
            execution_failed: None,
            session,
            input_mode: InputMode::Normal,
            breakpoints: vec![],
            breakpoints_hit: vec![],
            next_breakpoint_id: 0,
            stopped: true,
        })
    }

    pub fn reload(&mut self) -> Result<(), Report> {
        log::debug!("reloading program");
        let program = load_program(&self.session)?;
        let args = self.inputs.inputs.values().iter().copied().rev().collect::<Vec<_>>();

        let mut executor = crate::Executor::new(args.clone());
        executor.with_advice_inputs(self.inputs.advice_inputs.clone());
        for link_library in self.session.options.link_libraries.iter() {
            let lib = link_library.load(&self.session)?;
            executor.with_library(&lib);
        }
        let executor = executor.into_debug(&self.program, &self.session);

        // Execute the program until it terminates to capture a full trace for use during debugging
        let mut trace_executor = crate::Executor::new(args);
        trace_executor.with_advice_inputs(self.inputs.advice_inputs.clone());
        for link_library in self.session.options.link_libraries.iter() {
            let lib = link_library.load(&self.session)?;
            trace_executor.with_library(&lib);
        }
        let execution_trace = trace_executor.capture_trace(&program, &self.session);

        self.program = program;
        self.executor = executor;
        self.execution_trace = execution_trace;
        self.execution_failed = None;
        self.breakpoints_hit.clear();
        let breakpoints = core::mem::take(&mut self.breakpoints);
        self.breakpoints.reserve(breakpoints.len());
        self.next_breakpoint_id = 0;
        self.stopped = true;
        for bp in breakpoints {
            self.create_breakpoint(bp.ty);
        }
        Ok(())
    }

    pub fn create_breakpoint(&mut self, ty: BreakpointType) {
        let id = self.next_breakpoint_id();
        let creation_cycle = self.executor.cycle;
        log::trace!("created breakpoint with id {id} at cycle {creation_cycle}");
        self.breakpoints.push(Breakpoint {
            id,
            creation_cycle,
            ty,
        });
    }

    fn next_breakpoint_id(&mut self) -> u8 {
        let mut candidate = self.next_breakpoint_id;
        let mut initial = candidate;
        let mut next = candidate.wrapping_add(1);
        loop {
            assert_ne!(initial, next, "unable to allocate a breakpoint id: too many breakpoints");
            if self
                .breakpoints
                .iter()
                .chain(self.breakpoints_hit.iter())
                .any(|bp| bp.id == candidate)
            {
                candidate = next;
                continue;
            }
            self.next_breakpoint_id = next;
            break candidate;
        }
    }
}

macro_rules! write_with_format_type {
    ($out:ident, $read_expr:ident, $value:expr) => {
        match $read_expr.format {
            crate::FormatType::Decimal => write!(&mut $out, "{}", $value).unwrap(),
            crate::FormatType::Hex => write!(&mut $out, "{:0x}", $value).unwrap(),
            crate::FormatType::Binary => write!(&mut $out, "{:0b}", $value).unwrap(),
        }
    };
}

impl State {
    pub fn read_memory(&self, expr: &ReadMemoryExpr) -> Result<String, String> {
        use core::fmt::Write;

        use midenc_hir::Type;

        let cycle = miden_processor::RowIndex::from(self.executor.cycle);
        let context = self.executor.current_context;
        let mut output = String::new();
        if expr.count > 1 {
            return Err("-count with value > 1 is not yet implemented".into());
        } else if matches!(expr.ty, Type::Felt) {
            if !expr.addr.is_element_aligned() {
                return Err(
                    "read failed: type 'felt' must be aligned to an element boundary".into()
                );
            }
            let felt = self
                .execution_trace
                .read_memory_element_in_context(expr.addr.waddr, expr.addr.index, context, cycle)
                .unwrap_or(Felt::ZERO);
            write_with_format_type!(output, expr, felt.as_int());
        } else if matches!(expr.ty, Type::Array(ref elem_ty, 4) if elem_ty.as_ref() == &Type::Felt)
        {
            if !expr.addr.is_word_aligned() {
                return Err("read failed: type 'word' must be aligned to a word boundary".into());
            }
            let word = self.execution_trace.read_memory_word(expr.addr.waddr).unwrap_or_default();
            output.push('[');
            for (i, elem) in word.iter().enumerate() {
                if i > 0 {
                    output.push_str(", ");
                }
                write_with_format_type!(output, expr, elem.as_int());
            }
            output.push(']');
        } else {
            let bytes = self
                .execution_trace
                .read_bytes_for_type(expr.addr, &expr.ty, context, cycle)
                .map_err(|err| format!("invalid read: {err}"))?;
            match &expr.ty {
                Type::I1 => match expr.format {
                    crate::FormatType::Decimal => write!(&mut output, "{}", bytes[0] != 0).unwrap(),
                    crate::FormatType::Hex => {
                        write!(&mut output, "{:#0x}", (bytes[0] != 0) as u8).unwrap()
                    }
                    crate::FormatType::Binary => {
                        write!(&mut output, "{:#0b}", (bytes[0] != 0) as u8).unwrap()
                    }
                },
                Type::I8 => write_with_format_type!(output, expr, bytes[0] as i8),
                Type::U8 => write_with_format_type!(output, expr, bytes[0]),
                Type::I16 => {
                    write_with_format_type!(output, expr, i16::from_be_bytes([bytes[0], bytes[1]]))
                }
                Type::U16 => {
                    write_with_format_type!(output, expr, u16::from_be_bytes([bytes[0], bytes[1]]))
                }
                Type::I32 => write_with_format_type!(
                    output,
                    expr,
                    i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
                ),
                Type::U32 => write_with_format_type!(
                    output,
                    expr,
                    u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
                ),
                ty @ (Type::I64 | Type::U64) => {
                    let mut hi =
                        u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as u64;
                    let mut lo =
                        u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]) as u64;
                    let val = (hi * 2u64.pow(32)) + lo;
                    if matches!(ty, Type::I64) {
                        write_with_format_type!(output, expr, val as i64)
                    } else {
                        write_with_format_type!(output, expr, val)
                    }
                }
                ty => {
                    return Err(format!("support for reads of type '{ty}' are not implemented yet"))
                }
            }
        }

        Ok(output)
    }
}

fn load_program(session: &Session) -> Result<Arc<Program>, Report> {
    if let Some(entry) = session.options.entrypoint.as_ref() {
        // Input must be a library, not a program
        let id = entry
            .parse::<midenc_hir::FunctionIdent>()
            .map_err(|_| Report::msg(format!("invalid function identifier: '{entry}'")))?;
        let library = match session.inputs[0].file {
            InputType::Real(ref path) => read_library(path)?,
            InputType::Stdin { ref input, .. } => Library::read_from_bytes(input)
                .map_err(|err| Report::msg(format!("failed to read library from stdin: {err}")))?,
        };
        let module = library
            .module_infos()
            .find(|info| info.path().path() == id.module.as_str())
            .ok_or_else(|| {
            Report::msg(format!(
                "invalid entrypoint: library does not contain a module named '{}'",
                id.module.as_str()
            ))
        })?;
        let name = miden_assembly::ast::ProcedureName::new_unchecked(
            miden_assembly::ast::Ident::new_unchecked(Span::new(
                SourceSpan::UNKNOWN,
                Arc::from(id.function.as_str()),
            )),
        );
        if let Some(digest) = module.get_procedure_digest_by_name(&name) {
            let node_id = library.mast_forest().find_procedure_root(digest).ok_or_else(|| {
                Report::msg(
                    "invalid entrypoint: malformed library - procedure exported, but digest has \
                     no node in the forest",
                )
            })?;
            Ok(Arc::new(Program::new(library.mast_forest().clone(), node_id)))
        } else {
            Err(Report::msg(format!(
                "invalid entrypoint: library does not export '{}'",
                id.display()
            )))
        }
    } else {
        match session.inputs[0].file {
            InputType::Real(ref path) => read_program(path),
            InputType::Stdin { ref input, .. } => Program::read_from_bytes(input)
                .map(Arc::new)
                .map_err(|err| Report::msg(format!("failed to read program from stdin: {err}",))),
        }
    }
}

fn read_program(path: &std::path::Path) -> Result<Arc<Program>, Report> {
    use miden_core::utils::ReadAdapter;
    let bytes = std::fs::read(path)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to open program file '{}'", path.display()))?;
    let mut reader = miden_core::utils::SliceReader::new(bytes.as_slice());
    Program::read_from(&mut reader).map(Arc::new).map_err(|err| {
        Report::msg(format!("failed to read program from '{}': {err}", path.display()))
    })
}

fn read_library(path: &std::path::Path) -> Result<Library, Report> {
    Library::deserialize_from_file(path).map_err(|err| {
        Report::msg(format!("failed to read library from '{}': {err}", path.display()))
    })
}
