mod multitrait;

pub(crate) use self::multitrait::MultiTraitVtable;
use crate::{Immediate, Op, Symbol, Value};

pub trait BranchOpInterface {
    fn successor_operands(&self, succ_index: usize) -> &[Value];
    fn successor_block_argument(&self, succ_index: usize, index: usize) -> Option<Value>;
    fn get_successor_for_operands(&self, operands: &[Immediate]) -> Block;
}

pub trait RegionBranchOpInterface {}

pub trait RegionBranchTerminatorOpInterface {}

pub trait SelectLikeOpInterface {}

/// Marker trait for commutative ops, e.g. `X op Y == Y op X`
pub trait Commutative {}

/// Marker trait for constant-like ops
pub trait ConstantLike {}

/// Marker trait for ops with side effects
pub trait HasSideEffects {}

/// Marker trait for ops which read memory
pub trait MemoryRead {}

/// Marker trait for ops which write memory
pub trait MemoryWrite {}

/// Marker trait for return-like ops
pub trait ReturnLike {}

/// All operands of the given op are the same type
pub trait SameTypeOperands {}

/// Marker trait for ops whose regions contain only a single block
pub trait SingleBlock {}

/// Marker trait for ops which can terminate a block
pub trait Terminator {}

pub trait HasSymbol {
    fn name(&self) -> Symbol;
}

pub trait SymbolTable {
    type Entry;

    fn contains_symbol(&self, name: Symbol) -> bool;
    fn get_symbol(&self, name: Symbol) -> Option<&dyn Op>;
}
