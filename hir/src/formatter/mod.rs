mod document;
mod print;

#[cfg(not(feature = "std"))]
use alloc::string::String;
use core::fmt;

pub use self::document::{concat, const_text, display, flatten, indent, nl, text, Document};

pub trait PrettyPrint {
    fn render(&self) -> Document;
    fn to_pretty_string(&self) -> String {
        format!("{:width$}", Prettier(self), width = 80)
    }
    fn pretty_print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let doc = self.render();
        let width = f.width().unwrap_or(80);
        print::pretty_print(&doc, width, f)
    }
}

impl fmt::Display for dyn PrettyPrint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        PrettyPrint::pretty_print(self, f)
    }
}

macro_rules! pretty_via_display {
    ($name:ty) => {
        impl PrettyPrint for $name {
            fn render(&self) -> Document {
                display(*self)
            }
        }
    };
}

macro_rules! pretty_via_string {
    ($name:ty) => {
        impl PrettyPrint for $name {
            fn render(&self) -> Document {
                text(&**self)
            }
        }
    };
}

pretty_via_display!(bool);
pretty_via_display!(u8);
pretty_via_display!(i8);
pretty_via_display!(u16);
pretty_via_display!(i16);
pretty_via_display!(u32);
pretty_via_display!(i32);
pretty_via_display!(u64);
pretty_via_display!(i64);
pretty_via_display!(crate::Felt);
pretty_via_string!(alloc::rc::Rc<str>);
pretty_via_string!(alloc::sync::Arc<str>);

impl PrettyPrint for miden_core::crypto::hash::RpoDigest {
    fn render(&self) -> Document {
        DisplayHex(self.as_bytes().as_slice()).render()
    }
}

impl<'a, T: ?Sized + PrettyPrint> PrettyPrint for &'a T {
    #[inline]
    fn render(&self) -> Document {
        (**self).render()
    }

    #[inline]
    fn to_pretty_string(&self) -> String {
        (**self).to_pretty_string()
    }

    #[inline]
    fn pretty_print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (**self).pretty_print(f)
    }
}

impl PrettyPrint for str {
    fn render(&self) -> Document {
        self.lines()
            .map(text)
            .reduce(|acc, doc| match acc {
                Document::Empty => doc + nl(),
                other => other + doc + nl(),
            })
            .unwrap_or(Document::Empty)
    }
}

impl PrettyPrint for String {
    fn render(&self) -> Document {
        PrettyPrint::render(self.as_str())
    }

    fn pretty_print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        PrettyPrint::pretty_print(self.as_str(), f)
    }
}

impl<'a> PrettyPrint for alloc::borrow::Cow<'a, str> {
    fn render(&self) -> Document {
        PrettyPrint::render(self.as_ref())
    }

    fn pretty_print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        PrettyPrint::pretty_print(self.as_ref(), f)
    }
}

impl<T: PrettyPrint> PrettyPrint for alloc::boxed::Box<T> {
    fn render(&self) -> Document {
        PrettyPrint::render(self.as_ref())
    }

    fn pretty_print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        PrettyPrint::pretty_print(self.as_ref(), f)
    }
}

struct Prettier<'a, P: ?Sized + PrettyPrint>(&'a P);

impl<'a, P: ?Sized + PrettyPrint> fmt::Display for Prettier<'a, P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.pretty_print(f)
    }
}

/// A display helper for formatting a slice of bytes as hex
/// with different options using Rust's builtin format language
pub struct DisplayHex<'a>(pub &'a [u8]);

impl<'a> fmt::Display for DisplayHex<'a> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::LowerHex::fmt(self, f)
    }
}

impl<'a> PrettyPrint for DisplayHex<'a> {
    fn render(&self) -> Document {
        text(format!("{:#x}", self))
    }
}

impl<'a> fmt::LowerHex for DisplayHex<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            f.write_str("0x")?;
        }
        for byte in self.0.iter() {
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}

/// Builds a hex string from a byte slice
pub fn write_hex_bytes(f: &mut fmt::Formatter<'_>, bytes: &[u8]) -> fmt::Result {
    write!(f, "{:#x}", DisplayHex(bytes))
}

pub struct DisplayIndent(pub usize);
impl fmt::Display for DisplayIndent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const INDENT: &str = "  ";
        for _ in 0..self.0 {
            f.write_str(INDENT)?;
        }
        Ok(())
    }
}
