#![deny(warnings)]

pub use intrusive_collections::UnsafeRef;
pub use miden_diagnostics::SourceSpan;
pub use miden_hir_symbol::{symbols, Symbol};
pub use miden_hir_type::{AddressSpace, Alignable, FunctionType, StructType, Type};
pub use winter_math::{FieldElement, StarkField};

/// Represents a field element in Miden
pub type Felt = winter_math::fields::f64::BaseElement;

/// Represents an offset from the base of linear memory in Miden
pub type Offset = u32;

macro_rules! assert_matches {
    ($left:expr, $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )? $(,)?) => {
        match $left {
            $( $pattern )|+ $( if $guard )? => {}
            ref left_val => {
                panic!(r#"
assertion failed: `(left matches right)`
    left: `{:?}`,
    right: `{}`"#, left_val, stringify!($($pattern)|+ $(if $guard)?));
            }
        }
    };

    ($left:expr, $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )?, $msg:literal $(,)?) => {
        match $left {
            $( $pattern )|+ $( if $guard )? => {}
            ref left_val => {
                panic!(concat!(r#"
assertion failed: `(left matches right)`
    left: `{:?}`,
    right: `{}`
"#, $msg), left_val, stringify!($($pattern)|+ $(if $guard)?));
            }
        }
    };

    ($left:expr, $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )?, $msg:literal, $($arg:tt)+) => {
        match $left {
            $( $pattern )|+ $( if $guard )? => {}
            ref left_val => {
                panic!(concat!(r#"
assertion failed: `(left matches right)`
    left: `{:?}`,
    right: `{}`
"#, $msg), left_val, stringify!($($pattern)|+ $(if $guard)?), $($arg)+);
            }
        }
    }
}

mod asm;
mod block;
mod builder;
mod constants;
mod dataflow;
mod display;
mod function;
mod globals;
mod ident;
mod immediates;
mod insert;
mod instruction;
mod layout;
mod locals;
mod module;
mod program;
mod segments;
#[cfg(test)]
mod tests;
mod value;
mod write;

pub use self::asm::*;
pub use self::block::{Block, BlockData};
pub use self::builder::{
    DefaultInstBuilder, FunctionBuilder, InstBuilder, InstBuilderBase, ReplaceBuilder,
};
pub use self::constants::{Constant, ConstantData, ConstantPool, IntoBytes};
pub use self::dataflow::DataFlowGraph;
pub use self::display::{Decorator, DisplayValues};
pub use self::function::*;
pub use self::globals::*;
pub use self::ident::{FunctionIdent, Ident};
pub use self::immediates::Immediate;
pub use self::insert::{Insert, InsertionPoint};
pub use self::instruction::*;
pub use self::layout::{ArenaMap, LayoutAdapter, LayoutNode, OrderedArenaMap};
pub use self::locals::{Local, LocalId};
pub use self::module::*;
pub use self::program::{Linker, LinkerError, Program};
pub use self::segments::{DataSegment, DataSegmentAdapter, DataSegmentError, DataSegmentTable};
pub use self::value::{Value, ValueData, ValueList, ValueListPool};
pub use self::write::{write_external_function, write_function};

use core::fmt;

/// A `ProgramPoint` represents a position in a function where the live range of an SSA value can
/// begin or end. It can be either:
///
/// 1. An instruction or
/// 2. A block header.
///
/// This corresponds more or less to the lines in the textual form of the IR.
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub enum ProgramPoint {
    /// An instruction in the function.
    Inst(Inst),
    /// A block header.
    Block(Block),
}
impl ProgramPoint {
    /// Get the instruction we know is inside.
    pub fn unwrap_inst(self) -> Inst {
        match self {
            Self::Inst(x) => x,
            Self::Block(x) => panic!("expected inst: {}", x),
        }
    }
}
impl From<Inst> for ProgramPoint {
    fn from(inst: Inst) -> Self {
        Self::Inst(inst)
    }
}
impl From<Block> for ProgramPoint {
    fn from(block: Block) -> Self {
        Self::Block(block)
    }
}
impl fmt::Display for ProgramPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Inst(x) => write!(f, "{}", x),
            Self::Block(x) => write!(f, "{}", x),
        }
    }
}
impl fmt::Debug for ProgramPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ProgramPoint({})", self)
    }
}
