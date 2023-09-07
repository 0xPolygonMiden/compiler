use std::fmt::{self, Write};

use super::*;

pub fn write_function(w: &mut dyn Write, func: &Function) -> fmt::Result {
    write_signature(w, None, func.id.function, &func.signature)?;
    writeln!(w, " {{")?;
    for (i, (block, block_data)) in func.dfg.blocks().enumerate() {
        if i > 0 {
            writeln!(w)?;
        }

        write_block_header(w, func, block, 4)?;
        for inst in block_data.insts() {
            write_instruction(w, func, inst, 4)?;
        }
    }
    writeln!(w, "}}")
}

pub fn write_external_function(
    w: &mut dyn Write,
    name: &FunctionIdent,
    signature: &Signature,
) -> fmt::Result {
    write_signature(w, Some(name.module), name.function, signature)?;
    writeln!(w, ";")
}

fn write_signature(
    w: &mut dyn Write,
    module: Option<Ident>,
    name: Ident,
    signature: &Signature,
) -> fmt::Result {
    if signature.is_public() {
        write!(w, "pub ")?;
    }
    match signature.cc {
        CallConv::Fast => w.write_str("cc(fast) fn ")?,
        CallConv::SystemV => w.write_str("fn ")?,
        CallConv::Kernel => w.write_str("cc(kernel) fn ")?,
    }
    match module {
        None => write!(w, "{}(", name)?,
        Some(module) => write!(
            w,
            "{}(",
            &FunctionIdent {
                module,
                function: name
            }
        )?,
    }
    for (i, param) in signature.params().iter().enumerate() {
        if i > 0 {
            w.write_str(", ")?;
        }
        match param.purpose {
            ArgumentPurpose::Default => (),
            purpose => write!(w, "{} ", purpose)?,
        }
        match param.extension {
            ArgumentExtension::None => (),
            ext => write!(w, "{} ", ext)?,
        }
        write!(w, "{}", &param.ty)?;
    }
    w.write_str(")")?;
    let results = signature.results();
    if !results.is_empty() {
        w.write_str(" -> ")?;
        for (i, result) in results.iter().enumerate() {
            if i > 0 {
                w.write_str(", ")?;
            }
            match result.extension {
                ArgumentExtension::None => (),
                ArgumentExtension::Zext => w.write_str("zext ")?,
                ArgumentExtension::Sext => w.write_str("sext ")?,
            }
            write!(w, "{}", &result.ty)?;
        }
    }

    Ok(())
}

fn write_arg(w: &mut dyn Write, func: &Function, arg: Value) -> fmt::Result {
    write!(w, "{}: {}", arg, func.dfg.value_type(arg))
}

pub fn write_block_header(
    w: &mut dyn Write,
    func: &Function,
    block: Block,
    indent: usize,
) -> fmt::Result {
    // The indent is for instructions, block header is 4 spaces outdented
    write!(w, "{1:0$}{2}", indent - 4, "", block)?;

    let mut args = func.dfg.block_params(block).iter().cloned();
    match args.next() {
        None => return writeln!(w, ":"),
        Some(arg) => {
            write!(w, "(")?;
            write_arg(w, func, arg)?;
        }
    }
    for arg in args {
        write!(w, ", ")?;
        write_arg(w, func, arg)?;
    }
    writeln!(w, "):")
}

fn write_instruction(w: &mut dyn Write, func: &Function, inst: Inst, indent: usize) -> fmt::Result {
    let s = String::with_capacity(16);

    write!(w, "{1:0$}", indent, s)?;

    let mut has_results = false;
    for r in func.dfg.inst_results(inst) {
        if !has_results {
            has_results = true;
            write!(w, "{}", r)?;
        } else {
            write!(w, ", {}", r)?;
        }
    }
    if has_results {
        write!(w, " = ")?
    }

    let opcode = func.dfg[inst].opcode();
    write!(w, "{}", opcode)?;
    write_operands(w, &func.dfg, inst, indent)?;

    if has_results {
        write!(w, "  : ")?;
        for (i, v) in func.dfg.inst_results(inst).iter().enumerate() {
            let t = func.dfg.value_type(*v).to_string();
            if i > 0 {
                write!(w, ", {}", t)?;
            } else {
                write!(w, "{}", t)?;
            }
        }
    }

    writeln!(w)?;

    Ok(())
}

fn write_operands(
    w: &mut dyn Write,
    dfg: &DataFlowGraph,
    inst: Inst,
    indent: usize,
) -> fmt::Result {
    let pool = &dfg.value_lists;
    match dfg[inst].as_ref() {
        Instruction::BinaryOp(BinaryOp { args, .. }) => write!(w, " {}, {}", args[0], args[1]),
        Instruction::BinaryOpImm(BinaryOpImm { arg, imm, .. }) => write!(w, " {}, {}", arg, imm),
        Instruction::UnaryOp(UnaryOp { arg, .. }) => write!(w, " {}", arg),
        Instruction::UnaryOpImm(UnaryOpImm { imm, .. }) => write!(w, " {}", imm),
        Instruction::Ret(Ret { args, .. }) => write!(w, " {}", DisplayValues(args.as_slice(pool))),
        Instruction::RetImm(RetImm { arg, .. }) => write!(w, " {arg}"),
        Instruction::Call(Call { callee, args, .. }) => {
            write!(w, " {}({})", callee, DisplayValues(args.as_slice(pool)))
        }
        Instruction::CondBr(CondBr {
            cond,
            then_dest,
            else_dest,
            ..
        }) => {
            write!(w, " {}, ", cond)?;
            write!(w, "{}", then_dest.0)?;
            write_block_args(w, then_dest.1.as_slice(pool))?;
            write!(w, ", {}", else_dest.0)?;
            write_block_args(w, else_dest.1.as_slice(pool))
        }
        Instruction::Br(Br {
            op,
            destination,
            args,
            ..
        }) if *op == Opcode::Br => {
            write!(w, " {}", destination)?;
            write_block_args(w, args.as_slice(pool))
        }
        Instruction::Br(Br {
            destination, args, ..
        }) => {
            let args = args.as_slice(pool);
            write!(w, " {}, {}", args[0], destination)?;
            write_block_args(w, &args[1..])
        }
        Instruction::Switch(Switch {
            arg, arms, default, ..
        }) => {
            write!(w, " {}", arg)?;
            for (value, dest) in arms.iter() {
                write!(w, ", {} => {}", value, dest)?;
            }
            write!(w, ", {}", default)
        }
        Instruction::Test(Test { arg, ref ty, .. }) => {
            write!(w, ".{} {}", ty, arg)
        }
        Instruction::PrimOp(PrimOp { args, .. }) => {
            write!(w, " {}", DisplayValues(args.as_slice(pool)))
        }
        Instruction::PrimOpImm(PrimOpImm { imm, args, .. }) => {
            write!(w, " {}, {}", imm, DisplayValues(args.as_slice(pool)))
        }
        Instruction::Load(LoadOp { addr, .. }) => {
            write!(w, " {}", addr)
        }
        Instruction::MemCpy(MemCpy {
            args: [src, dst, count],
            ty,
            ..
        }) => {
            write!(w, ".{} {}, {}, {}", ty, src, dst, count)
        }
        Instruction::InlineAsm(ref asm) => {
            write!(w, " {}", asm.display(dfg, indent))
        }
        Instruction::GlobalValue(GlobalValueOp { global, .. }) => {
            write_global_value(w, dfg, *global, false)
        }
    }
}

fn write_global_value(
    w: &mut dyn Write,
    dfg: &DataFlowGraph,
    gv: GlobalValue,
    nested: bool,
) -> fmt::Result {
    match dfg.global_value(gv) {
        GlobalValueData::Symbol { name, offset } => {
            if !nested {
                w.write_str(".symbol ")?;
            }
            let offset = DisplayOffset::from(*offset);
            write!(w, "@{}{}", name, offset)
        }
        GlobalValueData::Load { base, offset, ty } if nested => {
            let pointer_ty = dfg.global_type(*base);
            let is_cast = pointer_ty.pointee().unwrap() != ty;
            let has_offset = *offset != 0;
            let offset = DisplayOffset::from(*offset);
            if is_cast || has_offset {
                write!(w, "*(")?;
            } else {
                w.write_char('*')?;
            }
            write_global_value(w, dfg, *base, true)?;
            if is_cast {
                write!(w, "){} as {}", offset, pointer_ty)
            } else if has_offset {
                write!(w, "){}", offset)
            } else {
                Ok(())
            }
        }
        GlobalValueData::Load { base, offset, ty } => {
            let pointer_ty = dfg.global_type(*base);
            let is_cast = pointer_ty.pointee().unwrap() != ty;
            let has_offset = *offset != 0;
            let offset = DisplayOffset::from(*offset);
            w.write_str(".load ")?;
            if is_cast || has_offset {
                w.write_char('(')?;
            }
            write_global_value(w, dfg, *base, true)?;
            if is_cast {
                write!(w, "){} as {}", offset, pointer_ty)
            } else if has_offset {
                write!(w, "){}", offset)
            } else {
                Ok(())
            }
        }
        GlobalValueData::IAddImm { base, offset, ty } => {
            if !nested {
                w.write_char('.')?;
            }
            write!(w, "iadd.{}.{} ", offset, ty)?;
            write_global_value(w, dfg, *base, true)
        }
    }
}

fn write_block_args(w: &mut dyn Write, args: &[Value]) -> fmt::Result {
    if args.is_empty() {
        Ok(())
    } else {
        write!(w, "({})", DisplayValues(args))
    }
}

#[derive(Copy, Clone)]
struct DisplayOffset(i32);
impl From<i32> for DisplayOffset {
    fn from(offset: i32) -> Self {
        Self(offset)
    }
}
impl fmt::Display for DisplayOffset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0 == 0 {
            Ok(())
        } else {
            if self.0 >= 0 {
                write!(f, "+{}", self.0)
            } else {
                write!(f, "{}", self.0)
            }
        }
    }
}

pub struct DisplayIndent(pub usize);
impl fmt::Display for DisplayIndent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const INDENT: &'static str = "  ";
        for _ in 0..self.0 {
            f.write_str(INDENT)?;
        }
        Ok(())
    }
}

pub struct DisplayValues<'a>(pub &'a [Value]);
impl<'a> fmt::Display for DisplayValues<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, val) in self.0.iter().enumerate() {
            if i == 0 {
                write!(f, "{}", val)?;
            } else {
                write!(f, ", {}", val)?;
            }
        }
        Ok(())
    }
}

pub struct DisplayValuesWithImmediate<'a>(&'a [Value], Immediate);
impl<'a> fmt::Display for DisplayValuesWithImmediate<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, val) in self.0.iter().enumerate() {
            if i == 0 {
                write!(f, "{}", val)?;
            } else {
                write!(f, ", {}", val)?;
            }
        }
        if self.0.is_empty() {
            write!(f, "{}", &self.1)
        } else {
            write!(f, ", {}", &self.1)
        }
    }
}
