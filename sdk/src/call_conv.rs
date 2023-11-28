use crate::Felt;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FuncArgPassingMedium {
    Stack,
    AdvProvider,
}

pub struct FuncArgPassingConv {
    pub medium: FuncArgPassingMedium,
    pub felt_count: u32,
}

impl FuncArgPassingConv {
    pub fn to_felt(&self) -> Felt {
        todo!()
    }

    pub fn from_felt(felt: Felt) -> Self {
        let _ = felt;
        todo!()
    }
}
