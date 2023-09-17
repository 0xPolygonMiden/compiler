//! Helper functions to gather information for each of the non-function sections of a
//! WebAssembly module.

use crate::{
    environ::ModuleEnvironment,
    error::{WasmError, WasmResult},
    unsupported_diag,
    wasm_types::{
        convert_func_type, convert_global_type, DataSegment, DataSegmentIndex, DataSegmentOffset,
        FuncIndex, GlobalIndex, GlobalInit, TypeIndex,
    },
};
use miden_diagnostics::DiagnosticsHandler;
use wasmparser::{
    Data, DataKind, DataSectionReader, ElementSectionReader, FunctionSectionReader,
    GlobalSectionReader, ImportSectionReader, MemorySectionReader, NameSectionReader, Naming,
    Operator, Type, TypeRef, TypeSectionReader,
};

/// Parses the Type section of the wasm module.
pub fn parse_type_section<'a>(
    types: TypeSectionReader<'a>,
    environ: &mut ModuleEnvironment<'a>,
    diagnostics: &DiagnosticsHandler,
) -> WasmResult<()> {
    for entry in types {
        match entry? {
            Type::Func(wasm_func_ty) => {
                let ty = convert_func_type(&wasm_func_ty)?;
                environ.declare_type_func(ty);
            }
            Type::Array(_) => {
                unsupported_diag!(diagnostics, "Array types are not supported");
            }
        }
    }
    Ok(())
}

/// Parses the Import section of the wasm module.
pub fn parse_import_section<'a>(
    imports: ImportSectionReader<'a>,
    environ: &mut ModuleEnvironment<'a>,
    diagnostics: &DiagnosticsHandler,
) -> WasmResult<()> {
    for entry in imports {
        let import = entry?;
        match import.ty {
            TypeRef::Func(sig) => {
                environ.declare_func_import(TypeIndex::from_u32(sig), import.module, import.name);
            }
            TypeRef::Memory(_) => {
                unsupported_diag!(diagnostics, "Memory imports are not supported");
            }
            TypeRef::Tag(_) => {
                unsupported_diag!(diagnostics, "Tag imports are not supported");
            }
            TypeRef::Global(_) => {
                unsupported_diag!(diagnostics, "Global imports are not supported");
            }
            TypeRef::Table(_) => {
                unsupported_diag!(diagnostics, "Table imports are not supported");
            }
        }
    }

    Ok(())
}

/// Parses the Function section of the wasm module.
pub fn parse_function_section<'a>(
    functions: FunctionSectionReader,
    environ: &mut ModuleEnvironment<'a>,
) -> WasmResult<()> {
    let num_functions = functions.count();
    if num_functions == std::u32::MAX {
        // We reserve `u32::MAX` for our own use in cranelift-entity.
        return Err(WasmError::FuncNumLimitExceeded);
    }

    for entry in functions {
        let sigindex = entry?;
        environ.declare_func_type(TypeIndex::from_u32(sigindex));
    }

    Ok(())
}

/// Parses the Memory section of the wasm module.
pub fn parse_memory_section<'a>(
    memories: MemorySectionReader,
    environ: &mut ModuleEnvironment<'a>,
) -> WasmResult<()> {
    for entry in memories {
        environ.declare_memory(entry?.into());
    }
    Ok(())
}

/// Parses the Global section of the wasm module.
pub fn parse_global_section<'a>(
    globals: GlobalSectionReader,
    environ: &mut ModuleEnvironment<'a>,
    diagnostics: &DiagnosticsHandler,
) -> WasmResult<()> {
    for entry in globals {
        let wasmparser::Global { ty, init_expr } = entry?;
        let mut init_expr_reader = init_expr.get_binary_reader();
        let initializer = match init_expr_reader.read_operator()? {
            Operator::I32Const { value } => GlobalInit::I32Const(value),
            Operator::I64Const { value } => GlobalInit::I64Const(value),
            Operator::F32Const { value } => GlobalInit::F32Const(value.bits()),
            Operator::F64Const { value } => GlobalInit::F64Const(value.bits()),
            Operator::GlobalGet { global_index } => {
                GlobalInit::GetGlobal(GlobalIndex::from_u32(global_index))
            }
            ref s => {
                unsupported_diag!(
                    diagnostics,
                    "unsupported init expr in global section: {:?}",
                    s
                );
            }
        };
        let global = convert_global_type(&ty, initializer)?;
        environ.declare_global(global);
    }

    Ok(())
}

/// Parses the Element section of the wasm module.
pub fn parse_element_section<'a>(
    _elements: ElementSectionReader<'a>,
    _environ: &mut ModuleEnvironment<'a>,
    diagnostics: &DiagnosticsHandler,
) -> WasmResult<()> {
    unsupported_diag!(diagnostics, "Element sections are not supported");
}

/// Parses the Data section of the wasm module.
pub fn parse_data_section<'a>(
    data: DataSectionReader<'a>,
    environ: &mut ModuleEnvironment<'a>,
    diagnostics: &DiagnosticsHandler,
) -> WasmResult<()> {
    for (_index, entry) in data.into_iter().enumerate() {
        let Data {
            kind,
            data,
            range: _,
        } = entry?;
        match kind {
            DataKind::Active {
                // ignored, since for Wasm spec v1 it's always 0
                memory_index: _,
                offset_expr,
            } => {
                let mut offset_expr_reader = offset_expr.get_binary_reader();
                let offset = match offset_expr_reader.read_operator()? {
                    Operator::I32Const { value } => DataSegmentOffset::I32Const(value),
                    Operator::GlobalGet { global_index } => {
                        DataSegmentOffset::GetGlobal(GlobalIndex::from_u32(global_index))
                    }
                    ref s => {
                        unsupported_diag!(
                            diagnostics,
                            "unsupported init expr in data section offset: {:?}",
                            s
                        );
                    }
                };
                let segment = DataSegment {
                    offset,
                    data: data.to_owned(),
                };
                environ.declare_data_segment(segment);
            }
            DataKind::Passive => {
                // Passive data segments type is added in Wasm spec 2.0
                unsupported_diag!(diagnostics, "Passive data segments are not supported");
            }
        }
    }
    Ok(())
}

/// Parses the Name section of the wasm module.
pub fn parse_name_section<'a>(
    names: NameSectionReader<'a>,
    environ: &mut ModuleEnvironment<'a>,
) -> WasmResult<()> {
    for subsection in names {
        match subsection? {
            wasmparser::Name::Function(names) => {
                for name in names {
                    let Naming { index, name } = name?;
                    // We reserve `u32::MAX` for our own use in cranelift-entity.
                    if index != u32::max_value() {
                        environ.declare_func_name(FuncIndex::from_u32(index), name);
                    }
                }
            }
            wasmparser::Name::Module { name, .. } => {
                environ.declare_module_name(name);
            }
            wasmparser::Name::Local(reader) => {
                for f in reader {
                    let f = f?;
                    if f.index == u32::max_value() {
                        continue;
                    }
                    for name in f.names {
                        let Naming { index, name } = name?;
                        environ.declare_local_name(FuncIndex::from_u32(f.index), index, name)
                    }
                }
            }
            wasmparser::Name::Global(names) => {
                for name in names {
                    let Naming { index, name } = name?;
                    if index != u32::max_value() {
                        environ.declare_global_name(GlobalIndex::from_u32(index), name);
                    }
                }
            }
            wasmparser::Name::Data(names) => {
                for name in names {
                    let Naming { index, name } = name?;
                    if index != u32::max_value() {
                        environ.declare_data_segment_name(DataSegmentIndex::from_u32(index), name);
                    }
                }
            }
            wasmparser::Name::Label(_)
            | wasmparser::Name::Type(_)
            | wasmparser::Name::Table(_)
            | wasmparser::Name::Memory(_)
            | wasmparser::Name::Element(_)
            | wasmparser::Name::Unknown { .. } => {}
        }
    }
    Ok(())
}
