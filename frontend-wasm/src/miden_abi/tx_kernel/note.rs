use midenc_hir::FunctionType;
use midenc_hir_type::Type::*;

use crate::miden_abi::{FunctionTypeMap, ModuleFunctionTypeMap};

pub const GET_INPUTS: &str = "get_inputs";

pub(crate) fn signatures() -> ModuleFunctionTypeMap {
    let mut m: ModuleFunctionTypeMap = Default::default();
    let mut note: FunctionTypeMap = Default::default();
    note.insert(GET_INPUTS, FunctionType::new([I32], [I32, Felt]));
    m.insert("miden:tx_kernel/note", note);
    m
}
