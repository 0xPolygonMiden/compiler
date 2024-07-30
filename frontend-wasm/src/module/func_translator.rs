//! Stand-alone WebAssembly to Miden IR translator.
//!
//! This module defines the `FuncTranslator` type which can translate a single WebAssembly
//! function to Miden IR guided by a `FuncEnvironment` which provides information about the
//! WebAssembly module and the runtime environment.
//!
//! Based on Cranelift's Wasm -> CLIF translator v11.0.0

use miden_diagnostics::{CodeMap, DiagnosticsHandler, SourceSpan};
use midenc_hir::{cranelift_entity::EntityRef, Block, InstBuilder, ModuleFunctionBuilder};
use wasmparser::{FuncValidator, FunctionBody, WasmModuleResources};

use super::{module_env::ParsedModule, module_translation_state::ModuleTranslationState};
use crate::{
    code_translator::translate_operator,
    error::WasmResult,
    module::{
        func_translation_state::FuncTranslationState,
        function_builder_ext::{FunctionBuilderContext, FunctionBuilderExt},
        module_env::DwarfReader,
        types::{convert_valtype, ir_type, ModuleTypes},
    },
    ssa::Variable,
    translation_utils::emit_zero,
};

/// WebAssembly to Miden IR function translator.
///
/// A `FuncTranslator` is used to translate a binary WebAssembly function into Miden IR guided
/// by a `FuncEnvironment` object. A single translator instance can be reused to translate multiple
/// functions which will reduce heap allocation traffic.
pub struct FuncTranslator {
    func_ctx: FunctionBuilderContext,
    state: FuncTranslationState,
}

impl FuncTranslator {
    /// Create a new translator.
    pub fn new() -> Self {
        Self {
            func_ctx: FunctionBuilderContext::new(),
            state: FuncTranslationState::new(),
        }
    }

    /// Translate a binary WebAssembly function from a `FunctionBody`.
    #[allow(clippy::too_many_arguments)]
    pub fn translate_body(
        &mut self,
        body: &FunctionBody<'_>,
        mod_func_builder: &mut ModuleFunctionBuilder,
        module_state: &mut ModuleTranslationState,
        module: &ParsedModule<'_>,
        mod_types: &ModuleTypes,
        addr2line: &addr2line::Context<DwarfReader<'_>>,
        codemap: &CodeMap,
        diagnostics: &DiagnosticsHandler,
        func_validator: &mut FuncValidator<impl WasmModuleResources>,
    ) -> WasmResult<()> {
        let mut builder = FunctionBuilderExt::new(mod_func_builder, &mut self.func_ctx);
        let entry_block = builder.current_block();
        builder.seal_block(entry_block); // Declare all predecessors known.

        let num_params = declare_parameters(&mut builder, entry_block);

        // Set up the translation state with a single pushed control block representing the whole
        // function and its return values.
        let exit_block = builder.create_block();
        builder.append_block_params_for_function_returns(exit_block);
        self.state.initialize(builder.signature(), exit_block);

        let mut reader = body.get_locals_reader()?;

        parse_local_decls(&mut reader, &mut builder, num_params, func_validator)?;

        let mut reader = body.get_operators_reader()?;
        parse_function_body(
            &mut reader,
            &mut builder,
            &mut self.state,
            module_state,
            module,
            mod_types,
            addr2line,
            codemap,
            diagnostics,
            func_validator,
        )?;

        builder.finalize();
        Ok(())
    }
}

/// Declare local variables for the signature parameters that correspond to WebAssembly locals.
///
/// Return the number of local variables declared.
fn declare_parameters(builder: &mut FunctionBuilderExt, entry_block: Block) -> usize {
    let sig_len = builder.signature().params().len();
    let mut next_local = 0;
    for i in 0..sig_len {
        let abi_param = &builder.signature().params()[i];
        let local = Variable::new(next_local);
        builder.declare_var(local, abi_param.ty.clone());
        next_local += 1;

        let param_value = builder.block_params(entry_block)[i];
        builder.def_var(local, param_value);
    }
    next_local
}

/// Parse the local variable declarations that precede the function body.
///
/// Declare local variables, starting from `num_params`.
fn parse_local_decls(
    reader: &mut wasmparser::LocalsReader<'_>,
    builder: &mut FunctionBuilderExt,
    num_params: usize,
    validator: &mut FuncValidator<impl WasmModuleResources>,
) -> WasmResult<()> {
    let mut next_local = num_params;
    let local_count = reader.get_count();

    for _ in 0..local_count {
        let pos = reader.original_position();
        let (count, ty) = reader.read()?;
        validator.define_locals(pos, count, ty)?;
        declare_locals(builder, count, ty, &mut next_local)?;
    }

    Ok(())
}

/// Declare `count` local variables of the same type, starting from `next_local`.
///
/// Fail if too many locals are declared in the function, or if the type is not valid for a local.
fn declare_locals(
    builder: &mut FunctionBuilderExt,
    count: u32,
    wasm_type: wasmparser::ValType,
    next_local: &mut usize,
) -> WasmResult<()> {
    let ty = ir_type(convert_valtype(wasm_type))?;
    // All locals are initialized to 0.
    let init = emit_zero(&ty, builder)?;
    for _ in 0..count {
        let local = Variable::new(*next_local);
        builder.declare_var(local, ty.clone());
        builder.def_var(local, init);
        *next_local += 1;
    }
    Ok(())
}

/// Parse the function body in `reader`.
///
/// This assumes that the local variable declarations have already been parsed and function
/// arguments and locals are declared in the builder.
#[allow(clippy::too_many_arguments)]
fn parse_function_body(
    reader: &mut wasmparser::OperatorsReader<'_>,
    builder: &mut FunctionBuilderExt,
    state: &mut FuncTranslationState,
    module_state: &mut ModuleTranslationState,
    module: &ParsedModule<'_>,
    mod_types: &ModuleTypes,
    addr2line: &addr2line::Context<DwarfReader<'_>>,
    codemap: &CodeMap,
    diagnostics: &DiagnosticsHandler,
    func_validator: &mut FuncValidator<impl WasmModuleResources>,
) -> WasmResult<()> {
    // The control stack is initialized with a single block representing the whole function.
    debug_assert_eq!(state.control_stack.len(), 1, "State not initialized");

    let mut end_span = SourceSpan::default();
    while !reader.eof() {
        let pos = reader.original_position();
        let (op, offset) = reader.read_with_offset()?;
        func_validator.op(pos, &op)?;

        let offset = (offset as u64)
            .checked_sub(module.wasm_file.code_section_offset)
            .expect("offset occurs before start of code section");
        let mut span = SourceSpan::default();
        if let Some(loc) = addr2line
            .find_location(offset)
            .map_err(|err| crate::WasmError::Unexpected(err.to_string()))?
        {
            if let Some(file) = loc.file {
                let path = std::path::Path::new(file);
                if path.exists() {
                    let source_id = codemap.add_file(path)?;
                    let line = loc.line.and_then(|line| line.checked_sub(1)).unwrap_or(0);
                    let column = loc.column.and_then(|col| col.checked_sub(1)).unwrap_or(0);
                    span = codemap
                        .line_column_to_span(source_id, line, column)
                        .ok()
                        .unwrap_or_default();
                }
            }
        }

        // Track the span of every END we observe, so we have a span to assign to the return we
        // place in the final exit block
        if let wasmparser::Operator::End = op {
            end_span = span;
        }

        translate_operator(
            &op,
            builder,
            state,
            module_state,
            &module.module,
            mod_types,
            diagnostics,
            span,
        )?;
    }
    let pos = reader.original_position();
    func_validator.finish(pos)?;

    // The final `End` operator left us in the exit block where we need to manually add a return
    // instruction.
    //
    // If the exit block is unreachable, it may not have the correct arguments, so we would
    // generate a return instruction that doesn't match the signature.
    if state.reachable && !builder.is_unreachable() {
        builder.ins().ret(state.stack.first().cloned(), end_span);
    }

    // Discard any remaining values on the stack. Either we just returned them,
    // or the end of the function is unreachable.
    state.stack.clear();

    Ok(())
}
