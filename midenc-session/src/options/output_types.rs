use core::fmt;
use core::str::FromStr;

use clap::ValueEnum;

/// This enum represents the type of outputs the compiler can produce
#[derive(Debug, Copy, Clone, Default, ValueEnum)]
pub enum OutputType {
    /// The compiler will emit the abstract syntax tree of the input, if applicable
    Ast,
    /// The compiler will emit Miden IR
    Hir,
    /// The compiler will emit Miden Assembly
    #[default]
    Masm,
    /// The compiler will emit all artifacts
    All,
}
impl OutputType {
    pub fn unwrap_extension(&self) -> &'static str {
        match self {
            Self::Ast => "ast",
            Self::Hir => "hir",
            Self::Masm => "masm",
            Self::All => panic!("cannot get extension for output type 'all'"),
        }
    }
}
impl fmt::Display for OutputType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Ast => f.write_str("ast"),
            Self::Hir => f.write_str("hir"),
            Self::Masm => f.write_str("masm"),
            Self::All => f.write_str("all"),
        }
    }
}
impl FromStr for OutputType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ast" => Ok(Self::Ast),
            "hir" => Ok(Self::Hir),
            "masm" => Ok(Self::Masm),
            "all" => Ok(Self::All),
            _ => Err(()),
        }
    }
}

bitflags::bitflags! {
    /// This struct represents the set of outputs the compiler should produce
    pub struct OutputTypes: u32 {
        /// The compiler will emit the abstract syntax tree of the input, if applicable
        const AST = 1;
        /// The compiler will emit Miden IR
        const HIR = 1 << 1;
        /// The compiler will emit Miden Assembly
        const MASM = 1 << 2;

        /// An alias which represents output of all artifact types
        const ALL = Self::AST.bits | Self::HIR.bits | Self::MASM.bits;
    }
}
impl Default for OutputTypes {
    /// By default the compiler will emit Miden Assembly
    fn default() -> Self {
        Self::MASM
    }
}
impl From<OutputType> for OutputTypes {
    fn from(ty: OutputType) -> Self {
        match ty {
            OutputType::Ast => Self::AST,
            OutputType::Hir => Self::HIR,
            OutputType::Masm => Self::MASM,
            OutputType::All => Self::ALL,
        }
    }
}
impl OutputTypes {
    pub fn from_slice(tys: &[OutputType]) -> Self {
        let mut output_types = Self::empty();
        for ty in tys.iter().copied() {
            output_types |= ty.into();
        }
        output_types
    }
}
