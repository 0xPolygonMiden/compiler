/// Possible encodings of strings within the component model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StringEncoding {
    Utf8,
    Utf16,
    CompactUtf16,
}
