use std::sync::Arc;

use miden_assembly::{diagnostics::SourceFile, SourceSpan as MasmSpan};
use miden_diagnostics::{CodeMap, SourceId, SourceSpan as HirSpan};

/// Obtain a [miden_assembly::diagnostics::SourceFile] from a [miden_diagnostics::SourceSpan]
pub fn source_file_for_span(span: HirSpan, codemap: &CodeMap) -> Option<Arc<SourceFile>> {
    let source_id = span.source_id();
    let source_file = codemap.get(source_id).ok()?;
    Some(Arc::new(SourceFile::new(
        source_file.name().as_str().unwrap(),
        source_file.source().to_string(),
    )))
}

/// Obtain a [miden_assembly::diagnostics::SourceSpan] from a [miden_diagnostics::SourceSpan]
#[inline]
pub fn translate_span(span: HirSpan) -> MasmSpan {
    if span.is_unknown() {
        MasmSpan::default()
    } else {
        MasmSpan::new(span.start_index().0..span.end_index().0)
    }
}

/// Convert a [miden_assembly::diagnostics::SourceSpan] to a [miden_diagnostics::SourceSpan],
/// using the provided [SourceId] as the context for the resulting span.
//
// TODO(pauls): We should probably assert that the byte offsets are valid for the given source
pub fn from_masm_span(source_id: SourceId, span: MasmSpan) -> HirSpan {
    use miden_diagnostics::SourceIndex;

    if span == MasmSpan::default() {
        HirSpan::UNKNOWN
    } else {
        let start: u32 =
            span.start().try_into().expect("invalid start: byte offset is > 2^32 bytes");
        let end: u32 = span.end().try_into().expect("invalid start: byte offset is > 2^32 bytes");
        HirSpan::new(
            SourceIndex::new(source_id, start.into()),
            SourceIndex::new(source_id, end.into()),
        )
    }
}
