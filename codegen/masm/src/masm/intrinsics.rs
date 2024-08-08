use miden_assembly::{ast::ModuleKind, LibraryPath};
use midenc_hir::diagnostics::{PrintDiagnostic, SourceManager};

use super::Module;

const I32_INTRINSICS: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/intrinsics/i32.masm"));
const I64_INTRINSICS: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/intrinsics/i64.masm"));
const MEM_INTRINSICS: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/intrinsics/mem.masm"));

/// This is a mapping of intrinsics module name to the raw MASM source for that module
const INTRINSICS: [(&str, &str, &str); 3] = [
    (
        "intrinsics::i32",
        I32_INTRINSICS,
        concat!(env!("CARGO_MANIFEST_DIR"), "/intrinsics/i32.masm"),
    ),
    (
        "intrinsics::i64",
        I64_INTRINSICS,
        concat!(env!("CARGO_MANIFEST_DIR"), "/intrinsics/i64.masm"),
    ),
    (
        "intrinsics::mem",
        MEM_INTRINSICS,
        concat!(env!("CARGO_MANIFEST_DIR"), "/intrinsics/mem.masm"),
    ),
];

/// This helper loads the named module from the set of intrinsics modules defined in this crate.
///
/// Expects the fully-qualified name to be given, e.g. `intrinsics::mem`
pub fn load<N: AsRef<str>>(name: N, source_manager: &dyn SourceManager) -> Option<Module> {
    let name = name.as_ref();
    let (name, source, filename) = INTRINSICS.iter().copied().find(|(n, ..)| *n == name)?;
    let source_file = source_manager.load(filename, source.to_string());
    let path = LibraryPath::new(name).expect("invalid module name");
    match Module::parse(ModuleKind::Library, path, source_file.clone()) {
        Ok(module) => Some(module),
        Err(err) => {
            let err = PrintDiagnostic::new(err);
            panic!("failed to parse intrinsic module: {err}");
        }
    }
}
