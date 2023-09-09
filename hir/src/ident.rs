use core::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    str::FromStr,
};

use anyhow::anyhow;
use miden_diagnostics::{SourceSpan, Spanned};

use super::{symbols, Symbol};

/// Represents a globally-unique module/function name pair, with corresponding source spans.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Spanned)]
pub struct FunctionIdent {
    pub module: Ident,
    #[span]
    pub function: Ident,
}
impl FromStr for FunctionIdent {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.rsplit_once("::") {
            Some((ns, id)) => {
                let module = Ident::with_empty_span(Symbol::intern(ns));
                let function = Ident::with_empty_span(Symbol::intern(id));
                Ok(Self {
                    module,
                    function,
                })
            }
            None => Err(anyhow!("invalid function name, expected fully-qualified identifier, e.g. 'std::math::u64::checked_add'")),
        }
    }
}
impl fmt::Debug for FunctionIdent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("FunctionIdent")
            .field("module", &self.module.name)
            .field("function", &self.function.name)
            .field("span", &self.function.span)
            .finish()
    }
}
impl fmt::Display for FunctionIdent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}::{}", &self.module, &self.function)
    }
}
impl PartialOrd for FunctionIdent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for FunctionIdent {
    fn cmp(&self, other: &Self) -> Ordering {
        self.module
            .cmp(&other.module)
            .then(self.function.cmp(&other.function))
    }
}

/// Represents an identifier in the IR.
///
/// An identifier is some string, along with an associated source span
#[derive(Copy, Clone, Eq, Spanned)]
pub struct Ident {
    pub name: Symbol,
    #[span]
    pub span: SourceSpan,
}
impl Default for Ident {
    fn default() -> Self {
        Self {
            name: symbols::Empty,
            span: SourceSpan::UNKNOWN,
        }
    }
}
impl Ident {
    #[inline]
    pub const fn new(name: Symbol, span: SourceSpan) -> Ident {
        Ident { name, span }
    }

    #[inline]
    pub const fn with_empty_span(name: Symbol) -> Ident {
        Ident::new(name, SourceSpan::UNKNOWN)
    }

    /// Maps a string to an identifier with an empty syntax context.
    #[inline]
    pub fn from_str(string: &str) -> Ident {
        Ident::with_empty_span(Symbol::intern(string))
    }

    #[inline]
    pub fn as_str(self) -> &'static str {
        self.name.as_str()
    }

    #[inline(always)]
    pub fn as_symbol(self) -> Symbol {
        self.name
    }
}
impl std::borrow::Borrow<str> for Ident {
    #[inline]
    fn borrow(&self) -> &str {
        self.as_str()
    }
}
impl Ord for Ident {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_str().cmp(&other.as_str())
    }
}
impl PartialOrd for Ident {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Ident {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.name == rhs.name
    }
}
impl PartialEq<Symbol> for Ident {
    #[inline]
    fn eq(&self, rhs: &Symbol) -> bool {
        self.name.eq(rhs)
    }
}
impl Hash for Ident {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
impl fmt::Debug for Ident {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ident<{} {:?}>", self.name, self.span)
    }
}
impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.name, f)
    }
}
