use std::str::FromStr;

use midenc_codegen_masm::NativePtr;
use midenc_hir::Type;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadMemoryExpr {
    pub addr: NativePtr,
    pub ty: Type,
    pub count: u8,
    pub mode: MemoryMode,
    pub format: FormatType,
}
impl FromStr for ReadMemoryExpr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.trim();

        let mut expr = Self {
            addr: NativePtr::from_ptr(0),
            ty: Type::Array(Box::new(Type::Felt), 4),
            count: 1,
            mode: MemoryMode::default(),
            format: FormatType::default(),
        };

        // Parse any leading options
        let s = parse_options(s, &mut expr)?;

        let (addr, s) = parse_address(s)?;

        // Parse any trailing options
        let s = parse_options(s, &mut expr)?;

        // If the remaining string is non-empty, raise an error
        if !s.trim().is_empty() {
            return Err(format!("invalid read expression at '{s}'"));
        }

        // Handle address conversion, etc.
        //
        // If the mode is Byte, convert the address to NativePtr.
        //
        // Otherwise we can trivially construct the NativePtr from the word address
        expr.addr = if expr.mode == MemoryMode::Byte {
            NativePtr::from_ptr(addr)
        } else {
            NativePtr::new(addr, 0, 0)
        };

        Ok(expr)
    }
}

fn parse_options<'a>(s: &'a str, expr: &mut ReadMemoryExpr) -> Result<&'a str, String> {
    let mut s = s.trim_start();
    while let Some(rest) = s.strip_prefix('-') {
        // Try to split FLAG from any of its possible arguments
        let (flag, rest) = rest.split_once(' ').unwrap_or((rest, ""));
        match flag.trim() {
            "m" | "mode" => {
                // Try to split MODE from rest of expression
                match rest.split_once(' ') {
                    Some((mode, rest)) => {
                        expr.mode = mode.trim().parse()?;
                        s = rest.trim_start();
                    }
                    None => {
                        expr.mode = rest.trim().parse()?;
                        s = "";
                    }
                }
            }
            "c" | "count" => {
                // Try to split COUNT from rest of expression
                match rest.split_once(' ') {
                    Some((count, rest)) => {
                        expr.count =
                            count.trim().parse().map_err(|err| format!("invalid count: {err}"))?;
                        s = rest.trim_start();
                    }
                    None => {
                        expr.count =
                            rest.trim().parse().map_err(|err| format!("invalid count: {err}"))?;
                        s = "";
                    }
                }
            }
            "f" | "format" => {
                // Try to split FORMAT from rest of expression
                match rest.split_once(' ') {
                    Some((format, rest)) => {
                        expr.format = format.trim().parse()?;
                        s = rest.trim_start();
                    }
                    None => {
                        expr.format = rest.trim().parse()?;
                        s = "";
                    }
                }
            }
            "t" | "type" => {
                // Try to split TYPE from rest of expression
                match rest.split_once(' ') {
                    Some((ty, rest)) => {
                        let (ty, rest) = parse_type(ty.trim())?;
                        expr.ty = ty;
                        s = rest.trim_start();
                    }
                    None => {
                        let (ty, rest) = parse_type(rest.trim())?;
                        expr.ty = ty;
                        s = rest;
                    }
                }
            }
            _ => return Err(format!("unrecognized option: {flag}")),
        }
    }

    Ok(s)
}

fn parse_type(s: &str) -> Result<(Type, &str), String> {
    Ok((
        match s {
            "i1" => Type::I1,
            "i8" => Type::I8,
            "i16" => Type::I16,
            "i32" => Type::I32,
            "i64" => Type::I64,
            "i128" => Type::I128,
            "u8" => Type::U8,
            "u16" => Type::U16,
            "u32" => Type::U32,
            "u64" => Type::U64,
            "u128" => Type::U128,
            "felt" => Type::Felt,
            "word" => Type::Array(Box::new(Type::Felt), 4),
            "ptr" | "pointer" => Type::Ptr(Box::new(Type::U32)),
            _ => return Err(format!("invalid/unsupported type '{s}'")),
        },
        "",
    ))
}

fn parse_address(s: &str) -> Result<(u32, &str), String> {
    let mut buf = String::new();
    let mut last_index = 0;
    let radix = if let Some(rest) = s.strip_prefix("0x") {
        last_index = 2;
        let mut chars = rest.chars().peekable();
        while let Some(c) = chars.next_if(|&c| c.is_ascii_hexdigit()) {
            buf.push(c);
            last_index += c.len_utf8();
        }
        16
    } else {
        let mut chars = s.chars().peekable();
        while let Some(c) = chars.next_if(|&c| c.is_ascii_digit()) {
            buf.push(c);
            last_index += c.len_utf8();
        }
        10
    };

    if buf.is_empty() {
        return Err(format!("expected memory address at '{s}'"));
    }

    let addr = u32::from_str_radix(buf.as_str(), radix)
        .map_err(|err| format!("invalid memory address: {err}"))?;
    let (_, rest) = s.split_at_checked(last_index).unwrap_or((s, ""));
    Ok((addr, rest.trim_start()))
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum MemoryMode {
    #[default]
    Word,
    Byte,
}
impl FromStr for MemoryMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" | "word" | "words" | "miden" => Ok(Self::Word),
            "b" | "byte" | "bytes" | "rust" => Ok(Self::Byte),
            _ => Err(format!("invalid memory mode '{s}'")),
        }
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum FormatType {
    #[default]
    Decimal,
    Hex,
    Binary,
}
impl FromStr for FormatType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "d" | "decimal" => Ok(Self::Decimal),
            "x" | "hex" | "hexadecimal" => Ok(Self::Hex),
            "b" | "bin" | "binary" | "bits" => Ok(Self::Binary),
            _ => Err(format!("invalid format type '{s}'")),
        }
    }
}
