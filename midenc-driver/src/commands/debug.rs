use std::ffi::OsStr;
use std::fmt;

use miden_hir::FunctionIdent;

/// Describes a breakpoint expression provided as a command-line argument
#[derive(Debug, Clone)]
pub enum Breakpoint {
    /// Break on every instruction
    All,
    /// Break at the given location in the program
    At(miden_codegen_masm::InstructionPointer),
    /// Break before the first instruction of the given function when called
    Call(FunctionIdent),
    /// Break at every iteration of a loop instruction
    Loops,
    /// Break when a write to memory in the range `addr..(addr + size)` occurs
    ///
    /// The address is specified in bytes, not words
    MemoryWrite(core::ops::Range<usize>),
}
impl fmt::Display for Breakpoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::All => f.write_str("all"),
            Self::Loops => f.write_str("loops"),
            Self::At(ip) => write!(f, "ip={}:{}", ip.block.as_u32(), ip.index),
            Self::Call(callee) => write!(f, "call={}", callee),
            Self::MemoryWrite(range) => write!(f, "mem={}..{}", range.start, range.end),
        }
    }
}
impl Into<miden_codegen_masm::Breakpoint> for Breakpoint {
    fn into(self) -> miden_codegen_masm::Breakpoint {
        use miden_codegen_masm as masm;
        match self {
            Self::All => masm::Breakpoint::Step,
            Self::At(ip) => masm::Breakpoint::StepUntil(ip),
            Self::Call(callee) => masm::Breakpoint::Call(callee),
            Self::Loops => masm::Breakpoint::Loop,
            Self::MemoryWrite(range) => masm::Breakpoint::MemoryWrite {
                addr: range.start,
                size: range.end - range.start,
            },
        }
    }
}
impl clap::builder::ValueParserFactory for Breakpoint {
    type Parser = BreakpointParser;

    fn value_parser() -> Self::Parser {
        BreakpointParser
    }
}

#[doc(hidden)]
#[derive(Clone)]
pub struct BreakpointParser;
impl clap::builder::TypedValueParser for BreakpointParser {
    type Value = Breakpoint;

    fn parse_ref(
        &self,
        _cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &OsStr,
    ) -> Result<Self::Value, clap::Error> {
        use clap::{error::ErrorKind, Error};
        use miden_codegen_masm::{BlockId, InstructionPointer};

        let value = value
            .to_str()
            .ok_or_else(|| Error::new(ErrorKind::InvalidUtf8))?;

        match value {
            "all" => Ok(Breakpoint::All),
            "loops" => Ok(Breakpoint::Loops),
            value => match value.split_once('=') {
                Some(("ip", expr)) => match expr.split_once(':') {
                    Some((block_id, index)) => {
                        let block_id = block_id.parse::<u32>().map_err(|err| {
                            Error::raw(
                                ErrorKind::ValueValidation,
                                format!("invalid block id: {err}"),
                            )
                        })?;
                        let index = index.parse::<usize>().map_err(|err| {
                            Error::raw(ErrorKind::ValueValidation, format!("invalid index: {err}"))
                        })?;
                        Ok(Breakpoint::At(InstructionPointer {
                            block: BlockId::from_u32(block_id),
                            index,
                        }))
                    }
                    None => Err(Error::raw(
                        ErrorKind::ValueValidation,
                        "invalid instruction pointer specification, expected `<block>:<index>`",
                    )),
                },
                Some(("call", callee)) => {
                    let callee = callee
                        .parse()
                        .map_err(|err| Error::raw(ErrorKind::ValueValidation, err))?;
                    Ok(Breakpoint::Call(callee))
                }
                Some(("mem", expr)) if expr.contains("..") => {
                    let (start_addr, end_addr) = expr.split_once("..").unwrap();
                    let start_addr = parse_memory_address(start_addr)?;
                    let end_addr = parse_memory_address(end_addr)?;
                    Ok(Breakpoint::MemoryWrite(start_addr..end_addr))
                }
                Some(("mem", expr)) => {
                    let addr = parse_memory_address(expr)?;
                    Ok(Breakpoint::MemoryWrite(addr..(addr + 1)))
                }
                _ => Err(Error::raw(
                    ErrorKind::ValueValidation,
                    format!("unrecognized breakpoint expression: {value}"),
                )),
            },
        }
    }
}

fn parse_memory_address(addr: &str) -> Result<usize, clap::Error> {
    use clap::{error::ErrorKind, Error};
    use core::num::IntErrorKind;

    let (addr, radix) = if let Some(addr) = addr.strip_prefix("0x") {
        (addr, 16)
    } else {
        (addr, 10)
    };
    u32::from_str_radix(addr, radix)
        .map_err(|err| match err.kind() {
            IntErrorKind::PosOverflow => Error::raw(
                ErrorKind::ValueValidation,
                "invalid memory address: must be smaller than 2^32",
            ),
            IntErrorKind::NegOverflow => Error::raw(
                ErrorKind::ValueValidation,
                "invalid memory address: must be a non-negative integer",
            ),
            _ => Error::raw(ErrorKind::ValueValidation, "invalid memory address"),
        })
        .map(|addr| addr as usize)
}
