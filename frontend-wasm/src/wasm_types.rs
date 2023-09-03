//! Internal types for parsed WebAssembly.

use miden_ir::cranelift_entity::entity_impl;
use miden_ir::types::FunctionType;
use miden_ir::types::Type;

use crate::environ::ModuleInfo;
use crate::error::WasmError;
use crate::error::WasmResult;

/// Index type of a function (imported or defined) inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct FuncIndex(u32);
entity_impl!(FuncIndex);

/// Index type of a defined function inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct DefinedFuncIndex(u32);
entity_impl!(DefinedFuncIndex);

/// Index type of a global variable (imported or defined) inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct GlobalIndex(u32);
entity_impl!(GlobalIndex);

/// Index type of a linear memory (imported or defined) inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct MemoryIndex(u32);
entity_impl!(MemoryIndex);

/// Index type of a type inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct TypeIndex(u32);
entity_impl!(TypeIndex);

/// A WebAssembly global.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Global {
    /// The Miden IR type of the value stored in the global.
    pub ty: Type,
    /// A flag indicating whether the value may change at runtime.
    pub mutability: bool,
}

/// Globals are initialized via the `const` operators or by referring to another import.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum GlobalInit {
    /// An `i32.const`.
    I32Const(i32),
    /// An `i64.const`.
    I64Const(i64),
    /// An `f32.const`.
    F32Const(u32),
    /// An `f64.const`.
    F64Const(u64),
    /// A `global.get` of another global.
    GetGlobal(GlobalIndex),
}

/// WebAssembly linear memory.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Memory {
    /// The minimum number of pages in the memory.
    pub minimum: u64,
    /// The maximum number of pages in the memory.
    pub maximum: Option<u64>,
    /// Whether or not this is a 64-bit memory
    pub memory64: bool,
}

impl From<wasmparser::MemoryType> for Memory {
    fn from(ty: wasmparser::MemoryType) -> Memory {
        Memory {
            minimum: ty.initial,
            maximum: ty.maximum,
            memory64: ty.memory64,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct BlockType {
    pub params: Vec<Type>,
    pub results: Vec<Type>,
}

impl BlockType {
    pub fn from_wasm(
        block_ty: &wasmparser::BlockType,
        module_info: &ModuleInfo,
    ) -> WasmResult<Self> {
        Ok(match block_ty {
            wasmparser::BlockType::Empty => Self::default(),
            wasmparser::BlockType::Type(ty) => Self {
                params: vec![],
                results: vec![valtype_to_type(ty)?],
            },
            wasmparser::BlockType::FuncType(ty_index) => {
                let func_type = &module_info.func_types[TypeIndex::from_u32(*ty_index)];
                Self {
                    params: func_type.params.clone(),
                    results: func_type.results.clone(),
                }
            }
        })
    }
}

pub fn convert_global_type(ty: &wasmparser::GlobalType) -> WasmResult<Global> {
    Ok(Global {
        ty: valtype_to_type(&ty.content_type)?,
        mutability: ty.mutable,
    })
}

/// Converts a wasmparser function type into a Miden IR function type
pub fn convert_func_type(ty: &wasmparser::FuncType) -> WasmResult<FunctionType> {
    let params = ty
        .params()
        .iter()
        .map(|t| valtype_to_type(t))
        .collect::<WasmResult<Vec<Type>>>()?;
    let results = ty
        .results()
        .iter()
        .map(|t| valtype_to_type(t))
        .collect::<WasmResult<Vec<Type>>>()?;
    Ok(FunctionType { results, params })
}

pub fn valtype_to_type(ty: &wasmparser::ValType) -> WasmResult<Type> {
    Ok(match ty {
        wasmparser::ValType::I32 => Type::I32,
        wasmparser::ValType::I64 => Type::I64,
        wasmparser::ValType::F32 => {
            todo!("no f32 type in Miden IR")
        }
        wasmparser::ValType::F64 => Type::F64,
        wasmparser::ValType::V128 => {
            return Err(WasmError::Unsupported(
                "V128 type is not supported".to_string(),
            ));
        }
        wasmparser::ValType::Ref(_) => {
            return Err(WasmError::Unsupported(
                "Ref type is not supported".to_string(),
            ));
        }
    })
}
