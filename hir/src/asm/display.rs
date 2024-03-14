use std::fmt::{self, Write};

use super::*;
use crate::{write::DisplayIndent, FunctionIdent, Ident, Symbol};

pub struct DisplayInlineAsm<'a> {
    function: Option<FunctionIdent>,
    asm: &'a InlineAsm,
    dfg: &'a DataFlowGraph,
    indent: usize,
}
impl<'a> DisplayInlineAsm<'a> {
    pub fn new(
        function: Option<FunctionIdent>,
        asm: &'a InlineAsm,
        dfg: &'a DataFlowGraph,
        indent: usize,
    ) -> Self {
        Self {
            function,
            asm,
            dfg,
            indent,
        }
    }
}
impl<'a> fmt::Display for DisplayInlineAsm<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::display::DisplayValues;

        {
            let args = self.asm.args.as_slice(&self.dfg.value_lists);
            writeln!(f, "({}) {{", DisplayValues::new(args.iter()))?;
        }

        let indent = self.indent;
        let block = self.asm.body;
        writeln!(
            f,
            "{}",
            DisplayMasmBlock {
                function: self.function,
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
    function: Option<FunctionIdent>,
    imports: Option<&'a ModuleImportInfo>,
    blocks: &'a PrimaryMap<MasmBlockId, MasmBlock>,
    block: MasmBlockId,
    indent: usize,
}
impl<'a> DisplayMasmBlock<'a> {
    pub fn new(
        function: Option<FunctionIdent>,
        imports: Option<&'a ModuleImportInfo>,
        blocks: &'a PrimaryMap<MasmBlockId, MasmBlock>,
        block: MasmBlockId,
        indent: usize,
    ) -> Self {
        Self {
            function,
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
        for (i, op) in block.ops.iter().enumerate() {
            if i > 0 {
                f.write_char('\n')?;
            }
            write!(
                f,
                "{}",
                DisplayOp {
                    function: self.function,
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
    function: Option<FunctionIdent>,
    imports: Option<&'a ModuleImportInfo>,
    blocks: &'a PrimaryMap<MasmBlockId, MasmBlock>,
    op: &'a MasmOp,
    indent: usize,
}
impl<'a> DisplayOp<'a> {
    #[inline(always)]
    pub fn is_local_module(&self, id: &Ident) -> bool {
        match self.function {
            Some(function) => &function.module == id,
            None => self.imports.map(|imports| !imports.is_import(id)).unwrap_or(false),
        }
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
            MasmOp::Pushw(word) => {
                write!(f, "push.{}.{}.{}.{}", &word[0], &word[1], &word[2], &word[3])
            }
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
            op @ MasmOp::AdvPush(n) => {
                write!(f, "{op}.{n}")
            }
            MasmOp::If(then_blk, else_blk) => {
                write!(
                    f,
                    "if.true\n{}\n{}else\n{}\n{}end",
                    DisplayMasmBlock {
                        function: self.function,
                        imports: self.imports,
                        blocks: self.blocks,
                        block: *then_blk,
                        indent: self.indent + 1
                    },
                    DisplayIndent(self.indent),
                    DisplayMasmBlock {
                        function: self.function,
                        imports: self.imports,
                        blocks: self.blocks,
                        block: *else_blk,
                        indent: self.indent + 1
                    },
                    DisplayIndent(self.indent),
                )
            }
            MasmOp::While(blk) => {
                write!(
                    f,
                    "while.true\n{}\n{}end",
                    DisplayMasmBlock {
                        function: self.function,
                        imports: self.imports,
                        blocks: self.blocks,
                        block: *blk,
                        indent: self.indent + 1
                    },
                    DisplayIndent(self.indent),
                )
            }
            MasmOp::Repeat(n, blk) => {
                write!(
                    f,
                    "repeat.{n}\n{}\n{}end",
                    DisplayMasmBlock {
                        function: self.function,
                        imports: self.imports,
                        blocks: self.blocks,
                        block: *blk,
                        indent: self.indent + 1
                    },
                    DisplayIndent(self.indent),
                )
            }
            op @ (MasmOp::Exec(id) | MasmOp::Syscall(id) | MasmOp::ProcRef(id)) => {
                let FunctionIdent { module, function } = id;
                if self.is_local_module(module) {
                    write!(f, "{op}.{function}")
                } else {
                    let alias = self.get_module_alias(*module);
                    write!(f, "{op}.{alias}::{function}")
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
            op @ (MasmOp::U32OverflowingAddImm(imm)
            | MasmOp::U32WrappingAddImm(imm)
            | MasmOp::U32OverflowingSubImm(imm)
            | MasmOp::U32WrappingSubImm(imm)
            | MasmOp::U32OverflowingMulImm(imm)
            | MasmOp::U32WrappingMulImm(imm)
            | MasmOp::U32DivImm(imm)
            | MasmOp::U32ModImm(imm)
            | MasmOp::U32DivModImm(imm)
            | MasmOp::U32ShlImm(imm)
            | MasmOp::U32ShrImm(imm)
            | MasmOp::U32RotlImm(imm)
            | MasmOp::U32RotrImm(imm)) => write!(f, "{op}.{imm}"),
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
