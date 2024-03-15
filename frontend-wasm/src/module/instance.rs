use miden_hir::{ComponentImport, FunctionIdent};

#[derive(Debug, Clone)]
pub enum ModuleArgument {
    Function(FunctionIdent),
    ComponentImport(ComponentImport),
    Table,
}
