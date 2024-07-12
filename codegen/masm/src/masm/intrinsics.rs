use miden_assembly::{ast::ModuleKind, LibraryPath};
use miden_diagnostics::{CodeMap, FileName};

use super::Module;

const I32_INTRINSICS: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/intrinsics/i32.masm"));
const MEM_INTRINSICS: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/intrinsics/mem.masm"));

/// This is a mapping of intrinsics module name to the raw MASM source for that module
const INTRINSICS: [(&str, &str, &str); 2] = [
    ("intrinsics::i32", I32_INTRINSICS, "i32.masm"),
    ("intrinsics::mem", MEM_INTRINSICS, "mem.masm"),
];

/// This helper loads the named module from the set of intrinsics modules defined in this crate.
///
/// Expects the fully-qualified name to be given, e.g. `intrinsics::mem`
pub fn load<N: AsRef<str>>(name: N, codemap: &CodeMap) -> Option<Module> {
    let name = name.as_ref();
    let (name, source, filename) = INTRINSICS.iter().copied().find(|(n, ..)| *n == name)?;
    let id = codemap.add(FileName::Virtual(filename.into()), source.to_string());
    let source_file = codemap.get(id).unwrap();
    let path = LibraryPath::new(name).expect("invalid module name");
    match Module::parse_source_file(path, ModuleKind::Library, source_file, codemap) {
        Ok(module) => Some(module),
        Err(err) => {
            panic!("unexpected syntax error in intrinsic module: {err}");
        }
    }
}

/// This helper loads the Miden Standard Library modules from the current miden-stdlib crate
pub fn load_stdlib(codemap: &CodeMap) -> &'static [Module] {
    use std::sync::OnceLock;

    use miden_assembly::Library;
    use miden_diagnostics::SourceSpan;
    use miden_stdlib::StdLibrary;

    static LOADED: OnceLock<Vec<Module>> = OnceLock::new();

    LOADED
        .get_or_init(|| {
            let library = StdLibrary::default();

            let mut loaded = Vec::with_capacity(library.modules().len());
            for module in library.modules() {
                let ir_module = Module::from_ast(module, SourceSpan::UNKNOWN, codemap);
                loaded.push(ir_module);
            }
            loaded
        })
        .as_slice()
}
