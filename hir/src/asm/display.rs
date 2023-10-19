use std::fmt;

use cranelift_entity::PrimaryMap;

use super::*;
use crate::{write::DisplayIndent, DataFlowGraph, FunctionIdent, Ident, Symbol};

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
            DisplayMasmBlock {
                imports: None,
                blocks: &self.asm.blocks,
                block,
                indent: indent + 1,
            }
        )?;

        writeln!(f, "{}}}", DisplayIndent(indent))
    }
}

pub struct DisplayMasmBlock<'a> {
    imports: Option<&'a ModuleImportInfo>,
    blocks: &'a PrimaryMap<MasmBlockId, MasmBlock>,
    block: MasmBlockId,
    indent: usize,
}
impl<'a> DisplayMasmBlock<'a> {
    pub fn new(
        imports: Option<&'a ModuleImportInfo>,
        blocks: &'a PrimaryMap<MasmBlockId, MasmBlock>,
        block: MasmBlockId,
        indent: usize,
    ) -> Self {
        Self {
            imports,
            blocks,
            block,
            indent,
        }
    }
}
impl<'a> fmt::Display for DisplayMasmBlock<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let block = &self.blocks[self.block];
        let indent = self.indent;
        for op in block.ops.iter() {
            writeln!(
                f,
                "{}",
                DisplayOp {
                    imports: self.imports,
                    blocks: self.blocks,
                    op,
                    indent
                }
            )?;
        }
        Ok(())
    }
}

struct DisplayOp<'a> {
    imports: Option<&'a ModuleImportInfo>,
    blocks: &'a PrimaryMap<MasmBlockId, MasmBlock>,
    op: &'a MasmOp,
    indent: usize,
}
impl<'a> DisplayOp<'a> {
    #[inline(always)]
    pub fn is_local_module(&self, id: &Ident) -> bool {
        self.imports
            .map(|imports| !imports.is_import(id))
            .unwrap_or(true)
    }

    pub fn get_module_alias(&self, module: Ident) -> Symbol {
        self.imports
            .and_then(|imports| imports.alias(&module))
            .unwrap_or(module)
            .as_symbol()
    }
}
impl<'a> fmt::Display for DisplayOp<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", DisplayIndent(self.indent))?;
        match self.op {
            MasmOp::Push(imm) => write!(f, "push.{imm}"),
            MasmOp::Push2([a, b]) => write!(f, "push.{a}.{b}"),
            MasmOp::Pushw(word) => write!(
                f,
                "push.{}.{}.{}.{}",
                &word[0], &word[1], &word[2], &word[3]
            ),
            MasmOp::PushU8(imm) => write!(f, "push.{imm}"),
            MasmOp::PushU16(imm) => write!(f, "push.{imm}"),
            MasmOp::PushU32(imm) => write!(f, "push.{imm}"),
            op @ (MasmOp::Dup(idx)
            | MasmOp::Dupw(idx)
            | MasmOp::Swap(idx)
            | MasmOp::Swapw(idx)
            | MasmOp::Movup(idx)
            | MasmOp::Movupw(idx)
            | MasmOp::Movdn(idx)
            | MasmOp::Movdnw(idx)) => write!(f, "{op}.{idx}"),
            op @ (MasmOp::LocAddr(id) | MasmOp::LocStore(id) | MasmOp::LocStorew(id)) => {
                write!(f, "{op}.{}", id.as_usize())
            }
            op @ (MasmOp::MemLoadImm(addr)
            | MasmOp::MemLoadwImm(addr)
            | MasmOp::MemStoreImm(addr)
            | MasmOp::MemStorewImm(addr)) => write!(f, "{op}.{}", Address(*addr)),
            op @ (MasmOp::MemLoadOffsetImm(addr, offset)
            | MasmOp::MemStoreOffsetImm(addr, offset)) => {
                write!(f, "{op}.{}.{offset}", Address(*addr))
            }
            MasmOp::If(then_blk, else_blk) => {
                f.write_str("if.true\n")?;
                {
                    let then_block = &self.blocks[*then_blk];
                    let indent = self.indent + 1;
                    for op in then_block.ops.iter() {
                        writeln!(
                            f,
                            "{}",
                            DisplayOp {
                                imports: self.imports,
                                blocks: self.blocks,
                                op,
                                indent
                            }
                        )?;
                    }
                }
                writeln!(f, "{}else", DisplayIndent(self.indent))?;
                {
                    let else_block = &self.blocks[*else_blk];
                    let indent = self.indent + 1;
                    for op in else_block.ops.iter() {
                        writeln!(
                            f,
                            "{}",
                            DisplayOp {
                                imports: self.imports,
                                blocks: self.blocks,
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
                    let body = &self.blocks[*blk];
                    let indent = self.indent + 1;
                    for op in body.ops.iter() {
                        writeln!(
                            f,
                            "{}",
                            DisplayOp {
                                imports: self.imports,
                                blocks: self.blocks,
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
                    let body = &self.blocks[*blk];
                    let indent = self.indent + 1;
                    for op in body.ops.iter() {
                        writeln!(
                            f,
                            "{}",
                            DisplayOp {
                                imports: self.imports,
                                blocks: self.blocks,
                                op,
                                indent
                            }
                        )?;
                    }
                }
                write!(f, "{}end", DisplayIndent(self.indent))
            }
            MasmOp::Exec(FunctionIdent { module, function }) => {
                if self.is_local_module(module) {
                    write!(f, "exec.{function}")
                } else {
                    let alias = self.get_module_alias(*module);
                    write!(f, "exec.{alias}::{function}")
                }
            }
            MasmOp::Syscall(FunctionIdent { module, function }) => {
                if self.is_local_module(module) {
                    write!(f, "syscall.{function}")
                } else {
                    let alias = self.get_module_alias(*module);
                    write!(f, "syscall.{alias}::{function}")
                }
            }
            op @ (MasmOp::AndImm(imm) | MasmOp::OrImm(imm) | MasmOp::XorImm(imm)) => {
                write!(f, "{op}.{imm}")
            }
            op @ MasmOp::ExpImm(imm) => write!(f, "{op}.{imm}"),
            op @ (MasmOp::AddImm(imm)
            | MasmOp::SubImm(imm)
            | MasmOp::MulImm(imm)
            | MasmOp::DivImm(imm)
            | MasmOp::EqImm(imm)
            | MasmOp::NeqImm(imm)
            | MasmOp::GtImm(imm)
            | MasmOp::GteImm(imm)
            | MasmOp::LtImm(imm)
            | MasmOp::LteImm(imm)) => write!(f, "{op}.{imm}"),
            op @ (MasmOp::U32CheckedAddImm(imm)
            | MasmOp::U32OverflowingAddImm(imm)
            | MasmOp::U32WrappingAddImm(imm)
            | MasmOp::U32CheckedSubImm(imm)
            | MasmOp::U32OverflowingSubImm(imm)
            | MasmOp::U32WrappingSubImm(imm)
            | MasmOp::U32CheckedMulImm(imm)
            | MasmOp::U32OverflowingMulImm(imm)
            | MasmOp::U32WrappingMulImm(imm)
            | MasmOp::U32CheckedDivImm(imm)
            | MasmOp::U32UncheckedDivImm(imm)
            | MasmOp::U32CheckedModImm(imm)
            | MasmOp::U32UncheckedModImm(imm)
            | MasmOp::U32CheckedDivModImm(imm)
            | MasmOp::U32UncheckedDivModImm(imm)
            | MasmOp::U32CheckedShlImm(imm)
            | MasmOp::U32UncheckedShlImm(imm)
            | MasmOp::U32CheckedShrImm(imm)
            | MasmOp::U32UncheckedShrImm(imm)
            | MasmOp::U32CheckedRotlImm(imm)
            | MasmOp::U32UncheckedRotlImm(imm)
            | MasmOp::U32CheckedRotrImm(imm)
            | MasmOp::U32UncheckedRotrImm(imm)
            | MasmOp::U32EqImm(imm)
            | MasmOp::U32NeqImm(imm)) => write!(f, "{op}.{imm}"),
            op => write!(f, "{op}"),
        }
    }
}

struct Address(u32);
impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x")?;
        for byte in self.0.to_be_bytes() {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}
