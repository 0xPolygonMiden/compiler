mod builder;
mod display;
mod isa;
mod stack;

pub use self::builder::*;
pub use self::display::{DisplayInlineAsm, DisplayMasmBlock};
pub use self::isa::*;
pub use self::stack::{OperandStack, Stack, StackElement};

use cranelift_entity::PrimaryMap;
use smallvec::smallvec;

use super::{DataFlowGraph, Opcode, Type, ValueList};

/// Represents Miden Assembly (MASM) directly in the IR
///
/// Each block of inline assembly executes in its own pseudo-isolated environment,
/// i.e. other than arguments provided to the inline assembly, and values introduced
/// within the inline assembly, it is not permitted to access anything else on the
/// operand stack.
///
/// In addition to arguments, inline assembly can produce zero or more results,
/// see [MasmBuilder] for more info.
///
/// Inline assembly can be built using [InstBuilder::inline_asm].
#[derive(Debug, Clone)]
pub struct InlineAsm {
    pub op: Opcode,
    /// Arguments on which the inline assembly can operate
    ///
    /// The operand stack will be set up such that the given arguments
    /// will appear in LIFO order, i.e. the first argument will be on top
    /// of the stack, and so on.
    ///
    /// The inline assembly will be validated so that all other values on
    /// the operand stack below the given arguments will remain on the stack
    /// when the inline assembly finishes executing.
    pub args: ValueList,
    /// The types of the results produced by this inline assembly block
    pub results: Vec<Type>,
    /// The main code block
    pub body: MasmBlockId,
    /// The set of all code blocks contained in this inline assembly
    ///
    /// This is necessary to support control flow operations within asm blocks
    pub blocks: PrimaryMap<MasmBlockId, MasmBlock>,
}
impl InlineAsm {
    /// Constructs a new, empty inline assembly block with the given result type(s).
    pub fn new(results: Vec<Type>) -> Self {
        let mut blocks = PrimaryMap::<MasmBlockId, MasmBlock>::new();
        let id = blocks.next_key();
        let body = blocks.push(MasmBlock {
            id,
            ops: smallvec![],
        });
        Self {
            op: Opcode::InlineAsm,
            args: ValueList::default(),
            results,
            body,
            blocks,
        }
    }

    /// Create a new code block for use with this inline assembly
    pub fn create_block(&mut self) -> MasmBlockId {
        let id = self.blocks.next_key();
        self.blocks.push(MasmBlock {
            id,
            ops: smallvec![],
        });
        id
    }

    /// Appends `op` to the end of `block`
    pub fn push(&mut self, block: MasmBlockId, op: MasmOp) {
        self.blocks[block].push(op);
    }

    pub fn display<'a, 'b: 'a>(
        &'b self,
        dfg: &'b DataFlowGraph,
        indent: usize,
    ) -> DisplayInlineAsm<'a> {
        DisplayInlineAsm::new(self, dfg, indent)
    }
}
