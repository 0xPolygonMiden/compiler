mod parser;
mod scanner;
mod source;

pub use parser::*;
pub use scanner::*;
pub use source::*;

/// The result of parsing some source input
pub enum ParseResult<R, E> {
    /// Hard error, cannot proceed
    Fail(E),
    /// Soft error, can proceed, but may fail at a later stage
    Warn(E, R),
    /// Success
    Ok(R),
}
impl<R, E> ParseResult<R, E>
where
    E: std::error::Error,
{
    pub fn unwrap(self) -> R {
        match self {
            Self::Fail(err) => {
                panic!("{err}");
            }
            Self::Warn(_err, res) => res,
            Self::Ok(res) => res,
        }
    }
}
