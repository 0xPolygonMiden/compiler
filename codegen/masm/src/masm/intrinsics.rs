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
    let (name, source, filename) = INTRINSICS.iter().copied().find(|(n, _, _)| *n == name)?;
    let id = codemap.add(FileName::Virtual(filename.into()), source.to_string());
    let source_file = codemap.get(id).unwrap();
    match Module::parse_source_file(source_file, name, codemap) {
        Ok(module) => Some(module),
        Err(err) => {
            panic!("unexpected syntax error in intrinsic module: {err}");
        }
    }
}
