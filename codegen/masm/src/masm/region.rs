use std::fmt;

use cranelift_entity::PrimaryMap;
use miden_assembly::ast;
use miden_hir::FunctionIdent;
use rustc_hash::FxHashMap;
use smallvec::smallvec;

use super::*;
use crate::InstructionPointer;

/// This struct represents the top-level initialization code for a [Program]
#[derive(Default)]
pub struct Begin {
    /// The imports available in `body`
    pub imports: ModuleImportInfo,
    /// The body of the `begin` block
    pub body: Region,
}
impl fmt::Display for Begin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("begin\n")?;

        writeln!(f, "{}", self.body.display(None, &self.imports, 1))?;

        f.write_str("end")
    }
}

/// This struct represents a region of code in Miden Assembly.
///
/// A region is a tree of blocks with isolated scope. In many
/// ways a [Region] is basically a [Function], just without any
/// identity. Additionally, a [Region] does not have local variables,
/// those must be provided by a parent [Function].
///
/// In short, this represents both the body of a function, and the
/// body of a `begin` block in Miden Assembly.
#[derive(Debug, Clone)]
pub struct Region {
    pub body: BlockId,
    pub blocks: PrimaryMap<BlockId, Block>,
}
impl Default for Region {
    fn default() -> Self {
        let mut blocks = PrimaryMap::<BlockId, Block>::default();
        let id = blocks.next_key();
        let body = blocks.push(Block {
            id,
            ops: smallvec![],
        });
        Self { body, blocks }
    }
}
impl Region {
    /// Get the [BlockId] for the block which forms the body of this region
    #[inline(always)]
    pub const fn id(&self) -> BlockId {
        self.body
    }

    /// Get a reference to a [Block] by [BlockId]
    #[inline]
    pub fn block(&self, id: BlockId) -> &Block {
        &self.blocks[id]
    }

    /// Get a mutable reference to a [Block] by [BlockId]
    #[inline]
    pub fn block_mut(&mut self, id: BlockId) -> &mut Block {
        &mut self.blocks[id]
    }

    /// Get the instruction under `ip`, if valid
    pub fn get(&self, ip: InstructionPointer) -> Option<Op> {
        self.blocks[ip.block].ops.get(ip.index).copied()
    }

    /// Allocate a new code block in this region
    pub fn create_block(&mut self) -> BlockId {
        let id = self.blocks.next_key();
        self.blocks.push(Block {
            id,
            ops: smallvec![],
        });
        id
    }

    /// Render the code in this region as Miden Assembly, at the specified indentation level (in
    /// units of 4 spaces)
    pub fn display<'a, 'b: 'a>(
        &'b self,
        function: Option<FunctionIdent>,
        imports: &'b ModuleImportInfo,
        indent: usize,
    ) -> impl fmt::Display + 'a {
        DisplayRegion {
            region: self,
            function,
            imports,
            indent,
        }
    }

    /// Convert this [Region] to a [miden_assembly::ast::CodeBody] using the provided
    /// local/external function maps to handle calls present in the body of the region.
    pub fn to_code_body(
        &self,
        codemap: &miden_diagnostics::CodeMap,
        imports: &ModuleImportInfo,
        local_ids: &FxHashMap<FunctionIdent, u16>,
        proc_ids: &FxHashMap<FunctionIdent, miden_assembly::ProcedureId>,
    ) -> ast::CodeBody {
        emit_block(self.body, &self.blocks, codemap, imports, local_ids, proc_ids)
    }

    /// Create a [Region] from a [miden_assembly::ast::CodeBody] and the set of imports
    /// and local procedures which will be used to map references to procedures to their
    /// fully-qualified names.
    pub fn from_code_body(
        code: &ast::CodeBody,
        locals: &[FunctionIdent],
        imported: &ast::ModuleImports,
    ) -> Self {
        let mut region = Self::default();

        let body = region.body;
        import_code_body(&mut region, body, code, locals, imported);

        region
    }
}
impl core::ops::Index<InstructionPointer> for Region {
    type Output = Op;

    #[inline]
    fn index(&self, ip: InstructionPointer) -> &Self::Output {
        &self.blocks[ip.block].ops[ip.index]
    }
}

#[doc(hidden)]
pub struct DisplayRegion<'a> {
    region: &'a Region,
    function: Option<FunctionIdent>,
    imports: &'a ModuleImportInfo,
    indent: usize,
}
impl<'a> fmt::Display for DisplayRegion<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use miden_hir::DisplayMasmBlock;

        write!(
            f,
            "{}",
            DisplayMasmBlock::new(
                self.function,
                Some(self.imports),
                &self.region.blocks,
                self.region.body,
                self.indent
            )
        )
    }
}

/// Import code from a [miden_assembly::ast::CodeBody] into the specified [Block] in `region`.
fn import_code_body(
    region: &mut Region,
    current_block_id: BlockId,
    code: &ast::CodeBody,
    locals: &[FunctionIdent],
    imported: &ast::ModuleImports,
) {
    for node in code.nodes() {
        match node {
            ast::Node::Instruction(ix) => {
                let current_block = region.block_mut(current_block_id);
                let mut ops = Op::from_masm(ix.clone(), locals, imported);
                current_block.append(&mut ops);
            }
            ast::Node::IfElse {
                ref true_case,
                ref false_case,
            } => {
                let then_blk = region.create_block();
                let else_blk = region.create_block();
                import_code_body(region, then_blk, true_case, locals, imported);
                import_code_body(region, else_blk, false_case, locals, imported);
                region.block_mut(current_block_id).push(Op::If(then_blk, else_blk));
            }
            ast::Node::Repeat { times, ref body } => {
                let body_blk = region.create_block();
                import_code_body(region, body_blk, body, locals, imported);
                region
                    .block_mut(current_block_id)
                    .push(Op::Repeat((*times).try_into().expect("too many repetitions"), body_blk));
            }
            ast::Node::While { ref body } => {
                let body_blk = region.create_block();
                import_code_body(region, body_blk, body, locals, imported);
                region.block_mut(current_block_id).push(Op::While(body_blk));
            }
        }
    }
}

/// Emit a [miden_assembly::ast::CodeBlock] by recursively visiting a tree of blocks
/// starting with `block_id`, using the provided imports and local/external procedure maps.
fn emit_block(
    block_id: BlockId,
    blocks: &PrimaryMap<BlockId, Block>,
    codemap: &miden_diagnostics::CodeMap,
    imports: &ModuleImportInfo,
    local_ids: &FxHashMap<FunctionIdent, u16>,
    proc_ids: &FxHashMap<FunctionIdent, miden_assembly::ProcedureId>,
) -> ast::CodeBody {
    let current_block = &blocks[block_id];
    let mut ops = Vec::with_capacity(current_block.ops.len());
    for op in current_block.ops.iter().copied() {
        match op {
            Op::If(then_blk, else_blk) => {
                let true_case = emit_block(then_blk, blocks, codemap, imports, local_ids, proc_ids);
                let false_case =
                    emit_block(else_blk, blocks, codemap, imports, local_ids, proc_ids);
                ops.push(ast::Node::IfElse {
                    true_case,
                    false_case,
                });
            }
            Op::While(blk) => {
                let body = emit_block(blk, blocks, codemap, imports, local_ids, proc_ids);
                ops.push(ast::Node::While { body });
            }
            Op::Repeat(n, blk) => {
                let body = emit_block(blk, blocks, codemap, imports, local_ids, proc_ids);
                ops.push(ast::Node::Repeat {
                    times: n as u32,
                    body,
                });
            }
            op => {
                ops.extend(op.into_node(codemap, imports, local_ids, proc_ids));
            }
        }
    }

    ast::CodeBody::new(ops)
}
