use std::fmt;

use super::*;
use crate::{write::DisplayIndent, DataFlowGraph};

pub struct DisplayInlineAsm<'a> {
    asm: &'a InlineAsm,
    dfg: &'a DataFlowGraph,
    indent: usize,
}
impl<'a> DisplayInlineAsm<'a> {
    pub fn new(asm: &'a InlineAsm, dfg: &'a DataFlowGraph, indent: usize) -> Self {
        Self { asm, dfg, indent }
    }
}
impl<'a> fmt::Display for DisplayInlineAsm<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::write::DisplayValues;

        {
            let args = self.asm.args.as_slice(&self.dfg.value_lists);
            writeln!(f, "({}) {{", DisplayValues(args))?;
        }

        let indent = self.indent;
        let block = self.asm.body;
        writeln!(
            f,
            "{}",
            DisplayBlock {
                asm: self.asm,
                block,
                indent: indent + 1,
            }
        )?;

        writeln!(f, "{}}}", DisplayIndent(indent))
    }
}

struct DisplayBlock<'a> {
    asm: &'a InlineAsm,
    block: MasmBlockId,
    indent: usize,
}
impl<'a> fmt::Display for DisplayBlock<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let block = &self.asm.blocks[self.block];
        let indent = self.indent;
        for op in block.ops.iter() {
            writeln!(
                f,
                "{}",
                DisplayOp {
                    asm: self.asm,
                    op,
                    indent
                }
            )?;
        }
        Ok(())
    }
}

struct DisplayOp<'a> {
    asm: &'a InlineAsm,
    op: &'a MasmOp,
    indent: usize,
}
impl<'a> fmt::Display for DisplayOp<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", DisplayIndent(self.indent))?;
        match self.op {
            MasmOp::Padw => f.write_str("padw"),
            MasmOp::Push(imm) => write!(f, "push.{}", imm),
            MasmOp::Pushw(word) => write!(
                f,
                "push.{}.{}.{}.{}",
                &word[0], &word[1], &word[2], &word[3]
            ),
            MasmOp::PushU8(imm) => write!(f, "push.{:#0x}", imm),
            MasmOp::PushU16(imm) => write!(f, "push.{:#0x}", imm),
            MasmOp::PushU32(imm) => write!(f, "push.{:#0x}", imm),
            MasmOp::Drop => f.write_str("drop"),
            MasmOp::Dropw => f.write_str("dropw"),
            MasmOp::Dup(idx) => write!(f, "dup.{idx}"),
            MasmOp::Dupw(idx) => write!(f, "dupw.{idx}"),
            MasmOp::Swap(idx) => write!(f, "swap.{idx}"),
            MasmOp::Swapw(idx) => write!(f, "swapw.{idx}"),
            MasmOp::Movup(idx) => write!(f, "movup.{idx}"),
            MasmOp::Movupw(idx) => write!(f, "movupw.{idx}"),
            MasmOp::Movdn(idx) => write!(f, "movdn.{idx}"),
            MasmOp::Movdnw(idx) => write!(f, "movdnw.{idx}"),
            MasmOp::Cswap => f.write_str("cswap"),
            MasmOp::Cswapw => f.write_str("cswapw"),
            MasmOp::Cdrop => f.write_str("cdrop"),
            MasmOp::Cdropw => f.write_str("cdropw"),
            MasmOp::Assert => f.write_str("assert"),
            MasmOp::Assertz => f.write_str("assertz"),
            MasmOp::AssertEq => f.write_str("assert_eq"),
            MasmOp::AssertEqw => f.write_str("assert_eqw"),
            MasmOp::LocAddr(id) => write!(f, "locaddr.{}", id.as_usize()),
            MasmOp::MemLoad | MasmOp::MemLoadOffset => write!(f, "mem_load"),
            MasmOp::MemLoadImm(addr) => write!(f, "mem_load.{:#0x}", addr),
            MasmOp::MemLoadOffsetImm(addr, offset) => write!(f, "mem_load.{:#0x}.{}", addr, offset),
            MasmOp::MemLoadw => write!(f, "mem_loadw"),
            MasmOp::MemLoadwImm(addr) => write!(f, "mem_loadw.{:#0x}", addr),
            MasmOp::MemStore | MasmOp::MemStoreOffset => write!(f, "mem_store"),
            MasmOp::MemStoreImm(addr) => write!(f, "mem_store.{:#0x}", addr),
            MasmOp::MemStoreOffsetImm(addr, offset) => {
                write!(f, "mem_store.{:#0x}.{}", addr, offset)
            }
            MasmOp::MemStorew => write!(f, "mem_storew"),
            MasmOp::MemStorewImm(addr) => write!(f, "mem_storew.{:#0x}", addr),
            MasmOp::If(then_blk, else_blk) => {
                f.write_str("if.true\n")?;
                {
                    let then_block = &self.asm.blocks[*then_blk];
                    let indent = self.indent + 1;
                    for op in then_block.ops.iter() {
                        writeln!(
                            f,
                            "{}",
                            DisplayOp {
                                asm: self.asm,
                                op,
                                indent
                            }
                        )?;
                    }
                }
                writeln!(f, "{}else", DisplayIndent(self.indent))?;
                {
                    let else_block = &self.asm.blocks[*else_blk];
                    let indent = self.indent + 1;
                    for op in else_block.ops.iter() {
                        writeln!(
                            f,
                            "{}",
                            DisplayOp {
                                asm: self.asm,
                                op,
                                indent
                            }
                        )?;
                    }
                }
                write!(f, "{}end", DisplayIndent(self.indent))
            }
            MasmOp::While(blk) => {
                f.write_str("while.true\n")?;
                {
                    let body = &self.asm.blocks[*blk];
                    let indent = self.indent + 1;
                    for op in body.ops.iter() {
                        writeln!(
                            f,
                            "{}",
                            DisplayOp {
                                asm: self.asm,
                                op,
                                indent
                            }
                        )?;
                    }
                }
                write!(f, "{}end", DisplayIndent(self.indent))
            }
            MasmOp::Repeat(n, blk) => {
                writeln!(f, "repeat.{}", n)?;
                {
                    let body = &self.asm.blocks[*blk];
                    let indent = self.indent + 1;
                    for op in body.ops.iter() {
                        writeln!(
                            f,
                            "{}",
                            DisplayOp {
                                asm: self.asm,
                                op,
                                indent
                            }
                        )?;
                    }
                }
                write!(f, "{}end", DisplayIndent(self.indent))
            }
            MasmOp::Exec(id) => write!(f, "exec.{}", id),
            MasmOp::Syscall(id) => write!(f, "syscall.{}", id),
            MasmOp::Add => f.write_str("add"),
            MasmOp::AddImm(imm) => write!(f, "add.{}", imm),
            MasmOp::Sub => f.write_str("sub"),
            MasmOp::SubImm(imm) => write!(f, "sub.{}", imm),
            MasmOp::Mul => f.write_str("mul"),
            MasmOp::MulImm(imm) => write!(f, "mul.{}", imm),
            MasmOp::Div => f.write_str("div"),
            MasmOp::DivImm(imm) => write!(f, "div.{}", imm),
            MasmOp::Neg => f.write_str("neg"),
            MasmOp::Inv => f.write_str("inv"),
            MasmOp::Incr => f.write_str("incr"),
            MasmOp::Pow2 => f.write_str("pow2"),
            MasmOp::Exp => f.write_str("exp.u64"),
            MasmOp::ExpImm(imm) => write!(f, "exp.{}", imm),
            MasmOp::Not => f.write_str("not"),
            MasmOp::And => f.write_str("and"),
            MasmOp::AndImm(imm) => write!(f, "and.{}", imm),
            MasmOp::Or => f.write_str("or"),
            MasmOp::OrImm(imm) => write!(f, "or.{}", imm),
            MasmOp::Xor => f.write_str("xor"),
            MasmOp::XorImm(imm) => write!(f, "xor.{}", imm),
            MasmOp::Eq => f.write_str("eq"),
            MasmOp::EqImm(imm) => write!(f, "eq.{}", imm),
            MasmOp::Neq => f.write_str("neq"),
            MasmOp::NeqImm(imm) => write!(f, "neq.{}", imm),
            MasmOp::Gt => f.write_str("gt"),
            MasmOp::GtImm(imm) => write!(f, "gt.{}", imm),
            MasmOp::Gte => f.write_str("gte"),
            MasmOp::GteImm(imm) => write!(f, "gte.{}", imm),
            MasmOp::Lt => f.write_str("lt"),
            MasmOp::LtImm(imm) => write!(f, "lt.{}", imm),
            MasmOp::Lte => f.write_str("lte"),
            MasmOp::LteImm(imm) => write!(f, "lte.{}", imm),
            MasmOp::IsOdd => f.write_str("is_odd"),
            MasmOp::Eqw => f.write_str("eqw"),
            MasmOp::Clk => f.write_str("clk"),
            MasmOp::U32Test => f.write_str("u32.test"),
            MasmOp::U32Testw => f.write_str("u32.testw"),
            MasmOp::U32Assert => f.write_str("u32.assert"),
            MasmOp::U32Assert2 => f.write_str("u32.assert2"),
            MasmOp::U32Assertw => f.write_str("u32.assertw"),
            MasmOp::U32Cast => f.write_str("u23.cast"),
            MasmOp::U32Split => f.write_str("u32.split"),
            MasmOp::U32CheckedAdd => f.write_str("u32.add.checked"),
            MasmOp::U32CheckedAddImm(imm) => write!(f, "u32.add.checked.{:#0x}", imm),
            MasmOp::U32OverflowingAdd => f.write_str("u32.add.overflowing"),
            MasmOp::U32OverflowingAddImm(imm) => write!(f, "u32.add.overflowing.{:#0x}", imm),
            MasmOp::U32WrappingAdd => f.write_str("u32.add.wrapping"),
            MasmOp::U32WrappingAddImm(imm) => write!(f, "u32.add.wrapping.{:#0x}", imm),
            MasmOp::U32OverflowingAdd3 => f.write_str("u32.add3.overflowing"),
            MasmOp::U32WrappingAdd3 => f.write_str("u32.add3.wrapping"),
            MasmOp::U32CheckedSub => f.write_str("u32.sub.checked"),
            MasmOp::U32CheckedSubImm(imm) => write!(f, "u32.sub.checked.{:#0x}", imm),
            MasmOp::U32OverflowingSub => f.write_str("u32.sub.overflowing"),
            MasmOp::U32OverflowingSubImm(imm) => write!(f, "u32.sub.overflowing.{:#0x}", imm),
            MasmOp::U32WrappingSub => f.write_str("u32.sub.wrapping"),
            MasmOp::U32WrappingSubImm(imm) => write!(f, "u32.sub.wrapping.{:#0x}", imm),
            MasmOp::U32CheckedMul => f.write_str("u32.mul.checked"),
            MasmOp::U32CheckedMulImm(imm) => write!(f, "u32.mul.checked.{:#0x}", imm),
            MasmOp::U32OverflowingMul => f.write_str("u32.mul.overflowing"),
            MasmOp::U32OverflowingMulImm(imm) => write!(f, "u32.mul.overflowing.{:#0x}", imm),
            MasmOp::U32WrappingMul => f.write_str("u32.mul.wrapping"),
            MasmOp::U32WrappingMulImm(imm) => write!(f, "u32.mul.wrapping.{:#0x}", imm),
            MasmOp::U32OverflowingMadd => f.write_str("u32.madd.overflowing"),
            MasmOp::U32WrappingMadd => f.write_str("u32.madd.wrapping"),
            MasmOp::U32CheckedDiv => f.write_str("u32.div.checked"),
            MasmOp::U32CheckedDivImm(imm) => write!(f, "u32.div.checked.{:#0x}", imm),
            MasmOp::U32UncheckedDiv => f.write_str("u32.div.unchecked"),
            MasmOp::U32UncheckedDivImm(imm) => write!(f, "u32.div.unchecked.{:#0x}", imm),
            MasmOp::U32CheckedMod => f.write_str("u32.mod.checked"),
            MasmOp::U32CheckedModImm(imm) => write!(f, "u32.mod.unchecked.{:#0x}", imm),
            MasmOp::U32UncheckedMod => f.write_str("u32.mod.unchecked"),
            MasmOp::U32UncheckedModImm(imm) => write!(f, "u32.mod.unchecked.{:#0x}", imm),
            MasmOp::U32CheckedDivMod => f.write_str("u32.divmod.checked"),
            MasmOp::U32CheckedDivModImm(imm) => write!(f, "u32.divmod.checked.{:#0x}", imm),
            MasmOp::U32UncheckedDivMod => f.write_str("u32.divmod.unchecked"),
            MasmOp::U32UncheckedDivModImm(imm) => write!(f, "u32.divmod.unchecked.{:#0x}", imm),
            MasmOp::U32And => f.write_str("u32.and"),
            MasmOp::U32Or => f.write_str("u32.or"),
            MasmOp::U32Xor => f.write_str("u32.xor"),
            MasmOp::U32Not => f.write_str("u32.not"),
            MasmOp::U32CheckedShl => f.write_str("u32.shl.checked"),
            MasmOp::U32CheckedShlImm(imm) => write!(f, "u32.shl.checked.{}", imm),
            MasmOp::U32UncheckedShl => f.write_str("u32.shl.unchecked"),
            MasmOp::U32UncheckedShlImm(imm) => write!(f, "u32.shl.unchecked.{}", imm),
            MasmOp::U32CheckedShr => f.write_str("u32.shr.checked"),
            MasmOp::U32CheckedShrImm(imm) => write!(f, "u32.shr.checked.{}", imm),
            MasmOp::U32UncheckedShr => f.write_str("u32.shr.unchecked"),
            MasmOp::U32UncheckedShrImm(imm) => write!(f, "u32.shr.unchecked.{}", imm),
            MasmOp::U32CheckedRotl => f.write_str("u32.rotl.checked"),
            MasmOp::U32CheckedRotlImm(imm) => write!(f, "u32.rotl.checked.{}", imm),
            MasmOp::U32UncheckedRotl => f.write_str("u32.rotl.unchecked"),
            MasmOp::U32UncheckedRotlImm(imm) => write!(f, "u32.rotl.unchecked.{}", imm),
            MasmOp::U32CheckedRotr => f.write_str("u32.rotr.checked"),
            MasmOp::U32CheckedRotrImm(imm) => write!(f, "u32.rotr.checked.{}", imm),
            MasmOp::U32UncheckedRotr => f.write_str("u32.rotr.unchecked"),
            MasmOp::U32UncheckedRotrImm(imm) => write!(f, "u32.rotr.unchecked.{}", imm),
            MasmOp::U32CheckedPopcnt => f.write_str("u32.popcnt.checked"),
            MasmOp::U32UncheckedPopcnt => f.write_str("u32.popcnt.unchecked"),
            MasmOp::U32Eq => f.write_str("u32.eq"),
            MasmOp::U32EqImm(imm) => write!(f, "u32.eq.{:#0x}", imm),
            MasmOp::U32Neq => f.write_str("u32.neq"),
            MasmOp::U32NeqImm(imm) => write!(f, "u32.neq.{:#0x}", imm),
            MasmOp::U32CheckedLt => f.write_str("u32.lt.checked"),
            MasmOp::U32UncheckedLt => f.write_str("u32.lt.unchecked"),
            MasmOp::U32CheckedLte => f.write_str("u32.lte.checked"),
            MasmOp::U32UncheckedLte => f.write_str("u32.lte.unchecked"),
            MasmOp::U32CheckedGt => f.write_str("u32.gt.checked"),
            MasmOp::U32UncheckedGt => f.write_str("u32.gt.unchecked"),
            MasmOp::U32CheckedGte => f.write_str("u32.gte.checked"),
            MasmOp::U32UncheckedGte => f.write_str("u32.gte.unchecked"),
            MasmOp::U32CheckedMin => f.write_str("u32.min.checked"),
            MasmOp::U32UncheckedMin => f.write_str("u32.min.unchecked"),
            MasmOp::U32CheckedMax => f.write_str("u32.max.checked"),
            MasmOp::U32UncheckedMax => f.write_str("u32.max.unchecked"),
        }
    }
}
