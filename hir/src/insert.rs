use crate::{Block, Function, ProgramPoint};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Insert {
    Before,
    After,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct InsertionPoint {
    pub at: ProgramPoint,
    pub action: Insert,
}
impl InsertionPoint {
    #[inline]
    pub const fn new(at: ProgramPoint, action: Insert) -> Self {
        Self { at, action }
    }

    #[inline]
    pub const fn before(at: ProgramPoint) -> Self {
        Self {
            at,
            action: Insert::Before,
        }
    }

    #[inline]
    pub const fn after(at: ProgramPoint) -> Self {
        Self {
            at,
            action: Insert::After,
        }
    }

    pub fn block(&self, function: &Function) -> Block {
        match self.at {
            ProgramPoint::Block(block) => block,
            ProgramPoint::Inst(inst) => function
                .dfg
                .inst_block(inst)
                .expect("cannot insert relative to detached instruction"),
        }
    }
}
