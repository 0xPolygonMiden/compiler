use std::fmt;
use std::str::FromStr;

use clap::ValueEnum;
use miden_diagnostics::Verbosity;

/// This struct is a wrapper for [miden_hir::Felt] that allows it to be parsed
/// from command-line arguments given to `midenc`.
#[derive(Debug, Copy, Clone, Default)]
pub struct Operand(miden_hir::Felt);
impl From<u64> for Operand {
    #[inline(always)]
    fn from(n: u64) -> Self {
        Self(miden_hir::Felt::new(n))
    }
}
impl From<i64> for Operand {
    #[inline(always)]
    fn from(n: i64) -> Self {
        Self(miden_hir::Felt::new(n as u64))
    }
}
impl FromStr for Operand {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use miden_hir::FieldElement;

        match s {
            "0" => Ok(Self(miden_hir::Felt::ZERO)),
            "1" => Ok(Self(miden_hir::Felt::ONE)),
            s if s.starts_with('-') => {
                // This must be a negative base-10 number, or it is invalid
                i64::from_str_radix(s, 10).map(Operand::from)
            }
            s if s.starts_with("0x") => {
                // This must be a non-negative base-16 number, or it is invalid
                u64::from_str_radix(s.trim_start_matches("0x"), 16).map(Operand::from)
            }
            s => {
                // This must be a non-negative base-10 number, or it is invalid
                u64::from_str_radix(s, 10).map(Operand::from)
            }
        }
    }
}

/// This enum represents the type of messages produced by the compiler during execution
#[derive(Debug, Copy, Clone, Default, ValueEnum)]
pub enum VerbosityFlag {
    /// Emit additional debug/trace information during compilation
    Debug,
    /// Emit the standard informational, warning, and error messages
    #[default]
    Info,
    /// Only emit warnings and errors
    Warning,
    /// Only emit errors
    Error,
    /// Do not emit anything to stdout/stderr
    Silent,
}
impl From<Verbosity> for VerbosityFlag {
    fn from(v: Verbosity) -> Self {
        match v {
            Verbosity::Debug => Self::Debug,
            Verbosity::Info => Self::Info,
            Verbosity::Warning => Self::Warning,
            Verbosity::Error => Self::Error,
            Verbosity::Silent => Self::Silent,
        }
    }
}
impl Into<Verbosity> for VerbosityFlag {
    fn into(self) -> Verbosity {
        match self {
            Self::Debug => Verbosity::Debug,
            Self::Info => Verbosity::Info,
            Self::Warning => Verbosity::Warning,
            Self::Error => Verbosity::Error,
            Self::Silent => Verbosity::Silent,
        }
    }
}
