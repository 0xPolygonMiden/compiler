use std::fmt::{self, Write};

use super::*;

pub fn write_function(w: &mut dyn Write, func: &Function) -> fmt::Result {
    let is_public = func.signature.visibility.is_public();
    if is_public {
        write!(w, "pub ")?;
    }
    write!(w, "fn ")?;
    write_spec(w, func)?;
    if func.signature.visibility.is_externally_defined() {
        return Ok(());
    }
    writeln!(w, " {{")?;
    let mut any = false;
    for (block, block_data) in func.dfg.blocks() {
        if any {
            writeln!(w)?;
        }

        write_block_header(w, func, block, 4)?;
        for inst in block_data.insts() {
            write_instruction(w, func, inst, 4)?;
        }

        any = true;
    }
    writeln!(w, "}}")
}

fn write_spec(w: &mut dyn Write, func: &Function) -> fmt::Result {
    write!(w, "{}(", &func.signature.name)?;
    let args = func
        .signature
        .params()
        .iter()
        .map(|t| t.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    let results = func
        .signature
        .results()
        .iter()
        .map(|t| t.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    write!(w, "{}) -> {} ", &args, &results)
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
    write_operands(w, &func.dfg, inst)?;

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

fn write_operands(w: &mut dyn Write, dfg: &DataFlowGraph, inst: Inst) -> fmt::Result {
    let pool = &dfg.value_lists;
    match dfg[inst].as_ref() {
        Instruction::BinaryOp(BinaryOp { args, .. }) => write!(w, " {}, {}", args[0], args[1]),
        Instruction::BinaryOpImm(BinaryOpImm { arg, imm, .. }) => write!(w, " {}, {}", arg, imm),
        Instruction::UnaryOp(UnaryOp { arg, .. }) => write!(w, " {}", arg),
        Instruction::UnaryOpImm(UnaryOpImm { imm, .. }) => write!(w, " {}", imm),
        Instruction::Ret(Ret { args, .. }) => write!(w, " {}", DisplayValues(args.as_slice(pool))),
        Instruction::Call(Call { args, .. }) => {
            let func_data = dfg.call_signature(inst).unwrap();
            write!(
                w,
                " {}({})",
                &func_data.name,
                DisplayValues(args.as_slice(pool))
            )
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
        Instruction::MemCpy(MemCpy {
            args: [src, dst, count],
            ty,
            ..
        }) => {
            write!(w, ".{} {}, {}, {}", ty, src, dst, count)
        }
        Instruction::InlineAsm(InlineAsm { ref body, args, .. }) => {
            write!(w, " \"")?;
            for (i, ix) in body.iter().enumerate() {
                if i == 0 {
                    write!(w, "{}", &ix.name)?;
                } else {
                    write!(w, " {}", &ix.name)?;
                }
            }
            let args = args.as_slice(pool);
            if args.is_empty() {
                write!(w, "\"")
            } else {
                write!(w, "\", {}", DisplayValues(args))
            }
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

struct DisplayValues<'a>(&'a [Value]);
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

struct DisplayValuesWithImmediate<'a>(&'a [Value], Immediate);
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
