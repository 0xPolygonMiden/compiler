use core::{fmt, mem, num::IntErrorKind};

use miden_diagnostics::{Diagnostic, SourceIndex, SourceSpan, ToDiagnostic};
use miden_parsing::{Scanner, Source};

use crate::{parser::ParseError, Symbol};

/// The value produced by the Lexer when iterated
pub type Lexed = Result<(SourceIndex, Token, SourceIndex), ParseError>;

/// Errors that may occur during lexing of the source
#[derive(Clone, Debug, thiserror::Error)]
pub enum LexicalError {
    #[error("invalid integer value: {}", DisplayIntErrorKind(reason))]
    InvalidInt {
        span: SourceSpan,
        reason: IntErrorKind,
    },
    #[error("encountered unexpected character '{found}'")]
    UnexpectedCharacter { start: SourceIndex, found: char },
}
impl PartialEq for LexicalError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::InvalidInt { reason: lhs, .. }, Self::InvalidInt { reason: rhs, .. }) => {
                lhs == rhs
            }
            (
                Self::UnexpectedCharacter { found: lhs, .. },
                Self::UnexpectedCharacter { found: rhs, .. },
            ) => lhs == rhs,
            _ => false,
        }
    }
}
impl ToDiagnostic for LexicalError {
    fn to_diagnostic(self) -> Diagnostic {
        use miden_diagnostics::Label;

        match self {
            Self::InvalidInt { span, ref reason } => Diagnostic::error()
                .with_message("invalid integer literal")
                .with_labels(vec![Label::primary(span.source_id(), span)
                    .with_message(format!("{}", DisplayIntErrorKind(reason)))]),
            Self::UnexpectedCharacter { start, .. } => Diagnostic::error()
                .with_message("unexpected character")
                .with_labels(vec![Label::primary(
                    start.source_id(),
                    SourceSpan::new(start, start),
                )]),
        }
    }
}

struct DisplayIntErrorKind<'a>(&'a IntErrorKind);
impl<'a> fmt::Display for DisplayIntErrorKind<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            IntErrorKind::Empty => write!(f, "unable to parse empty string as integer"),
            IntErrorKind::InvalidDigit => write!(f, "invalid digit"),
            IntErrorKind::PosOverflow => write!(f, "value is too big"),
            IntErrorKind::NegOverflow => write!(f, "value is too big"),
            IntErrorKind::Zero => write!(f, "zero is not a valid value here"),
            other => write!(f, "unable to parse integer value: {:?}", other),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Token {
    Eof,
    Error(LexicalError),
    Comment,
    // PRIMITIVES
    // --------------------------------------------------------------------------------------------
    /// Identifiers should start with alphabet followed by one or more alpha numeric characters
    /// or an underscore.
    Ident(Symbol),
    /// Function identifiers should be a non-empty sequence of identifiers separated by double colons "::".
    FuncIdent(Symbol),
    /// Integers should only contain numeric characters.
    Num(u128),
    /// Hex strings are used to initialize global variables
    Hex(Vec<u8>),

    // DECLARATION KEYWORDS
    // --------------------------------------------------------------------------------------------
    /// Used to declare kernel modules. Also used to declare a function with kernel calling convention.
    Kernel,
    /// Used to declare normal modules.
    Module,
    /// Used to declare a global variable with internal linkage.
    Internal,
    /// Used to declare a global variable with "one definition rule" linkage.
    Odr,
    /// Used to declare a global variable with external linkage.
    External,
    /// Keyword to declare that a function is publicly visible.
    Pub,
    /// Keyword to declare a function.
    Fn,
    /// Keyword to declare a function's calling convention.
    Cc,
    /// Keyword to declare that a function uses fast calling convention.
    Fast,
    /// Keyword to declare that a function parameter is a struct return
    Sret,
    /// Keyword to declare that a function parameter is extended with 0s when filling up a word
    /// Also used as an operation to pad a value with 0s.
    Zext,
    /// Keyword to declare that a function parameter is extended with sign bits when filling up a word
    /// Also used as an operation to pad a value with sign bits.
    Sext,

    // OPERATION KEYWORDS
    // --------------------------------------------------------------------------------------------
    /// Keyword to return from a function
    Ret,
    /// Keyword to call a function in user space
    Call,
    /// Keyword to call a function in kernel space
    SysCall,
    /// Keyword to perform a conditional jump
    Cond,
    /// Keyword to perform an unconditional jump
    Branch,
    /// Keyword to perform a multi-branch conditional jump
    Switch,
    /// Keyword to test whether a value has a specific type
    Test,
    /// Keyword to load a value from memory. Also used for the load operation on globals
    Load,
    /// Keyword to copy data from one memory location to another
    MemCpy,
    /// Keyword to indicate a sequence of assembly instructions
    InlineAsm,
    /// Keyword to indicate a memory management operation
    Memory,
    /// Keyword to indicate that the currently assigned amount of memory should grow
    Grow,
    /// Keyword to perform an addition
    Add,
    /// Keyword to perform a subtraction
    Sub,
    /// Keyword to perform a multiplication
    Mul,
    /// Keyword to perform a division
    Div,
    /// Keyword to determine a minimum value
    Min,
    /// Keyword to determine a maximum value
    Max,
    /// Keyword to perform a modulo
    Mod,
    /// Keyword to perform a division modulo 2^32
    DivMod,
    /// Keyword to perform an exponentiation
    Exp,
    /// Keyword to perform a boolean and
    And,
    /// Keyword to perform a bitwise and
    BAnd,
    /// Keyword to perform a boolean or
    Or,
    /// Keyword to perform a bitwise or
    BOr,
    /// Keyword to perform a boolean xor
    Xor,
    /// Keyword to perform a bitwise xor
    BXor,
    /// Keyword to perform a left shift
    Shl,
    /// Keyword to perform a right shift
    Shr,
    /// Keyword to perform a left rotation
    Rotl,
    /// Keyword to perform a right rotation
    Rotr,
    /// Keyword to test for equality
    Eq,
    /// Keyword to test for inequality
    Neq,
    /// Keyword to test for greater-than
    Gt,
    /// Keyword to test for greater-than-or-equal
    Gte,
    /// Keyword to test for less-than
    Lt,
    /// Keyword to test for less-than-or-equal
    Lte,
    /// Keyword to perform a store
    Store,
    /// Keyword to perform an addition with an immediate parameter
    AddImm,
    /// Keyword to perform a subtraction with an immediate parameter
    SubImm,
    /// Keyword to perform a multiplication with an immediate parameter
    MulImm,
    /// Keyword to perform a division with an immediate parameter
    DivImm,
    /// Keyword to determine a minimum with an immediate parameter
    MinImm,
    /// Keyword to determine a maximum with an immediate parameter
    MaxImm,
    /// Keyword to perform a modulo with an immediate parameter
    ModImm,
    /// Keyword to perform a division modulo 2^32 with an immediate parameter
    DivModImm,
    /// Keyword to perform an exponentiation with an immediate parameter
    ExpImm,
    /// Keyword to perform a boolaen and with an immediate parameter
    AndImm,
    /// Keyword to perform a bitwise and with an immediate parameter
    BAndImm,
    /// Keyword to perform a boolean or with an immediate parameter
    OrImm,
    /// Keyword to perform a bitwise or with an immediate parameter
    BOrImm,
    /// Keyword to perform a boolean xor with an immediate parameter
    XorImm,
    /// Keyword to perform a bitwise xor with an immediate parameter
    BXorImm,
    /// Keyword to perform a left shift with an immediate parameter
    ShlImm,
    /// Keyword to perform a right shift with an immediate parameter
    ShrImm,
    /// Keyword to perform a left rotation with an immediate parameter
    RotlImm,
    /// Keyword to perform a right rotation with an immediate parameter
    RotrImm,
    /// Keyword to perform an inversion within the field
    Inv,
    /// Keyword to perform an increment
    Incr,
    /// Keyword to perform a power-of-2 operation
    Pow2,
    /// Keyword to perform a boolean negation
    Not,
    /// Keyword to perform a bitwise negation
    BNot,
    /// Keyword to count the number of set bits in a value
    PopCnt,
    /// Keyword to check if a value is odd
    IsOdd,
    /// Keyword to perform a type cast
    Cast,
    /// Keyword to cast a pointer to an integer
    PtrToInt,
    /// Keyword to cast an integer to a pointer
    IntToPtr,
    /// Keyword to truncate a word
    TruncW,
    /// Keyword to perform a numerical negation
    Neg,
    /// Keyword to indicate an immediate unary operation (used alongside the type of the immediate)
    Const,
    /// Keyword to access elements in a struct
    Select,
    /// Keyword to perform an assertion
    Assert,
    /// Keyword to perform an 0-check assertion
    Assertz,
    /// Keyword to allocate an array
    Alloca,
    /// Keyword to indicate an unreachable part of the code
    Unreachable,
    /// Keyword used to indicate a global variable operation
    Global,
    /// Keyword used for type casts in global variable operations
    As,
    /// Keyword used to indicate the symbol global variable operation
    Symbol,
    /// Keyword used to indicate the iadd global variable operation
    IAdd,
    /// Keyword to indicated an unchecked aritmetic operation
    Unchecked,
    /// Keyword to indicated a checked aritmetic operation
    Checked,
    /// Keyword to indicated a wrapping aritmetic operation
    Wrapping,
    /// Keyword to indicated an overflowing aritmetic operation
    Overflowing,

    // TYPES
    // --------------------------------------------------------------------------------------------
    I1,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    I128,
    U128,
    U256,
    F64,
    Felt,
    Mut,

    // PUNCTUATION
    // --------------------------------------------------------------------------------------------
    DoubleQuote,
    Colon,
    Semicolon,
    Comma,
    Dot,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Equal,
    RDoubleArrow,
    Plus,
    Minus,
    RArrow,
    Star,
    Ampersand,
    Bang,
    At,
}
impl Token {
    pub fn from_keyword_or_ident(s: &str) -> Self {
        match s {
            "kernel" => Self::Kernel,
            "module" => Self::Module,
            "internal" => Self::Internal,
            "odr" => Self::Odr,
            "external" => Self::External,
            "pub" => Self::Pub,
            "fn" => Self::Fn,
            "cc" => Self::Cc,
            "fast" => Self::Fast,
            "sret" => Self::Sret,
            "zext" => Self::Zext,
            "sext" => Self::Sext,
            "ret" => Self::Ret,
            "call" => Self::Call,
            "syscall" => Self::SysCall,
            "cond" => Self::Cond,
            "branch" => Self::Branch,
            "switch" => Self::Switch,
            "test" => Self::Test,
            "load" => Self::Load,
            "memcpy" => Self::MemCpy,
            "inlineasm" => Self::InlineAsm,
            "memory" => Self::Memory,
            "grow" => Self::Grow,
            "add" => Self::Add,
            "sub" => Self::Sub,
            "mul" => Self::Mul,
            "div" => Self::Div,
            "min" => Self::Min,
            "max" => Self::Max,
            "mod" => Self::Mod,
            "divmod" => Self::DivMod,
            "exp" => Self::Exp,
            "and" => Self::And,
            "band" => Self::BAnd,
            "or" => Self::Or,
            "bor" => Self::BOr,
            "xor" => Self::Xor,
            "bxor" => Self::BXor,
            "shl" => Self::Shl,
            "shr" => Self::Shr,
            "rotl" => Self::Rotl,
            "rotr" => Self::Rotr,
            "eq" => Self::Eq,
            "neq" => Self::Neq,
            "gt" => Self::Gt,
            "gte" => Self::Gte,
            "lt" => Self::Lt,
            "lte" => Self::Lte,
            "store" => Self::Store,
            "add_imm" => Self::AddImm,
            "sub_imm" => Self::SubImm,
            "mul_imm" => Self::MulImm,
            "div_imm" => Self::DivImm,
            "min_imm" => Self::MinImm,
            "max_imm" => Self::MaxImm,
            "mod_imm" => Self::ModImm,
            "divmod_imm" => Self::DivModImm,
            "exp_imm" => Self::ExpImm,
            "and_imm" => Self::AndImm,
            "band_imm" => Self::BAndImm,
            "or_imm" => Self::OrImm,
            "bor_imm" => Self::BOrImm,
            "xor_imm" => Self::XorImm,
            "bxor_imm" => Self::BXorImm,
            "shl_imm" => Self::ShlImm,
            "shr_imm" => Self::ShrImm,
            "rotl_imm" => Self::RotlImm,
            "rotr_imm" => Self::RotrImm,
            "inv" => Self::Inv,
            "incr" => Self::Incr,
            "pow2" => Self::Pow2,
            "not" => Self::Not,
            "bnot" => Self::BNot,
            "popcnt" => Self::PopCnt,
            "is_odd" => Self::IsOdd,
            "cast" => Self::Cast,
            "ptrtoint" => Self::PtrToInt,
            "inttoprt" => Self::IntToPtr,
            "truncw" => Self::TruncW,
            "neg" => Self::Neg,
            "const" => Self::Const,
            "select" => Self::Select,
            "assert" => Self::Assert,
            "assertz" => Self::Assertz,
            "alloca" => Self::Alloca,
            "unreachable" => Self::Unreachable,
            "as" => Self::As,
            "global" => Self::Global,
            "symbol" => Self::Symbol,
            "iadd" => Self::IAdd,
            "unchecked" => Self::Unchecked,
            "checked" => Self::Checked,
            "wrapping" => Self::Wrapping,
            "overflowing" => Self::Overflowing,
            "i1" => Self::I1,
            "i8" => Self::I8,
            "u8" => Self::U8,
            "i16" => Self::I16,
            "u16" => Self::U16,
            "i32" => Self::I32,
            "u32" => Self::U32,
            "i64" => Self::I64,
            "u64" => Self::U64,
            "i128" => Self::I128,
            "u128" => Self::U128,
            "u256" => Self::U256,
            "f64" => Self::F64,
            "felt" => Self::Felt,
            "mut" => Self::Mut,
            other => Self::Ident(Symbol::intern(other)),
        }
    }
}
impl Eq for Token {}
impl PartialEq for Token {
    fn eq(&self, other: &Token) -> bool {
        match self {
            Self::Num(i) => {
                if let Self::Num(i2) = other {
                    return *i == *i2;
                }
            }
            Self::Error(_) => {
                if let Self::Error(_) = other {
                    return true;
                }
            }
            Self::Ident(i) => {
                if let Self::Ident(i2) = other {
                    return i == i2;
                }
            }
            Self::FuncIdent(i) => {
                if let Self::FuncIdent(i2) = other {
                    return i == i2;
                }
            }
            _ => return mem::discriminant(self) == mem::discriminant(other),
        }
        false
    }
}
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Eof => write!(f, "EOF"),
            Self::Error(_) => write!(f, "ERROR"),
            Self::Comment => write!(f, "COMMENT"),
            Self::Ident(ref id) => write!(f, "{}", id),
            Self::FuncIdent(ref id) => write!(f, "{}", id),
            Self::Num(ref i) => write!(f, "{}", i),
            Self::Hex(ref data) => {
                write!(f, "0x")?;
                for i in data.iter().rev() {
                    write!(f, "{:02x}", i)?;
                }
                Ok(())
            }
            Self::Kernel => write!(f, "kernel"),
            Self::Module => write!(f, "module"),
            Self::Internal => write!(f, "internal"),
            Self::Odr => write!(f, "odr"),
            Self::External => write!(f, "external"),
            Self::Pub => write!(f, "pub"),
            Self::Fn => write!(f, "fn"),
            Self::Cc => write!(f, "cc"),
            Self::Fast => write!(f, "fast"),
            Self::Sret => write!(f, "sret"),
            Self::Zext => write!(f, "zext"),
            Self::Sext => write!(f, "sext"),
            Self::Ret => write!(f, "ret"),
            Self::Call => write!(f, "call"),
            Self::SysCall => write!(f, "syscall"),
            Self::Cond => write!(f, "cond"),
            Self::Branch => write!(f, "branch"),
            Self::Switch => write!(f, "switch"),
            Self::Test => write!(f, "test"),
            Self::Load => write!(f, "load"),
            Self::MemCpy => write!(f, "memcpy"),
            Self::InlineAsm => write!(f, "inlineasm"),
            Self::Memory => write!(f, "memory"),
            Self::Grow => write!(f, "grow"),
            Self::Add => write!(f, "add"),
            Self::Sub => write!(f, "sub"),
            Self::Mul => write!(f, "mul"),
            Self::Div => write!(f, "div"),
            Self::Min => write!(f, "min"),
            Self::Max => write!(f, "max"),
            Self::Mod => write!(f, "mod"),
            Self::DivMod => write!(f, "divmod"),
            Self::Exp => write!(f, "exp"),
            Self::And => write!(f, "and"),
            Self::BAnd => write!(f, "band"),
            Self::Or => write!(f, "or"),
            Self::BOr => write!(f, "bor"),
            Self::Xor => write!(f, "xor"),
            Self::BXor => write!(f, "bxor"),
            Self::Shl => write!(f, "shl"),
            Self::Shr => write!(f, "shr"),
            Self::Rotl => write!(f, "rotl"),
            Self::Rotr => write!(f, "rotr"),
            Self::Eq => write!(f, "eq"),
            Self::Neq => write!(f, "neq"),
            Self::Gt => write!(f, "gt"),
            Self::Gte => write!(f, "gte"),
            Self::Lt => write!(f, "lt"),
            Self::Lte => write!(f, "lte"),
            Self::Store => write!(f, "store"),
            Self::AddImm => write!(f, "add_imm"),
            Self::SubImm => write!(f, "sub_imm"),
            Self::MulImm => write!(f, "mul_imm"),
            Self::DivImm => write!(f, "div_imm"),
            Self::MinImm => write!(f, "min_imm"),
            Self::MaxImm => write!(f, "max_imm"),
            Self::ModImm => write!(f, "mod_imm"),
            Self::DivModImm => write!(f, "divmod_imm"),
            Self::ExpImm => write!(f, "exp_imm"),
            Self::AndImm => write!(f, "and_imm"),
            Self::BAndImm => write!(f, "band_imm"),
            Self::OrImm => write!(f, "or_imm"),
            Self::BOrImm => write!(f, "bor_imm"),
            Self::XorImm => write!(f, "xor_imm"),
            Self::BXorImm => write!(f, "bxor_imm"),
            Self::ShlImm => write!(f, "shl_imm"),
            Self::ShrImm => write!(f, "shr_imm"),
            Self::RotlImm => write!(f, "rotl_imm"),
            Self::RotrImm => write!(f, "rotr_imm"),
            Self::Inv => write!(f, "inv"),
            Self::Incr => write!(f, "incr"),
            Self::Pow2 => write!(f, "pow2"),
            Self::Not => write!(f, "not"),
            Self::BNot => write!(f, "bnot"),
            Self::PopCnt => write!(f, "popcnt"),
            Self::IsOdd => write!(f, "is_odd"),
            Self::Cast => write!(f, "cast"),
            Self::PtrToInt => write!(f, "ptrtoint"),
            Self::IntToPtr => write!(f, "inttoptr"),
            Self::TruncW => write!(f, "truncw"),
            Self::Neg => write!(f, "neg"),
            Self::Const => write!(f, "const"),
            Self::Select => write!(f, "select"),
            Self::Assert => write!(f, "assert"),
            Self::Assertz => write!(f, "assertz"),
            Self::Alloca => write!(f, "alloca"),
            Self::Unreachable => write!(f, "unreachable"),
            Self::Global => write!(f, "global"),
            Self::As => write!(f, "as"),
            Self::Symbol => write!(f, "symbol"),
            Self::IAdd => write!(f, "iadd"),
            Self::Unchecked => write!(f, "unchecked"),
            Self::Checked => write!(f, "checked"),
            Self::Wrapping => write!(f, "wrapping"),
            Self::Overflowing => write!(f, "overflowing"),
            Self::I1 => write!(f, "i1"),
            Self::I8 => write!(f, "i8"),
            Self::U8 => write!(f, "u8"),
            Self::I16 => write!(f, "i16"),
            Self::U16 => write!(f, "u16"),
            Self::I32 => write!(f, "i32"),
            Self::U32 => write!(f, "u32"),
            Self::I64 => write!(f, "i64"),
            Self::U64 => write!(f, "u64"),
            Self::I128 => write!(f, "i128"),
            Self::U128 => write!(f, "u128"),
            Self::U256 => write!(f, "u256"),
            Self::F64 => write!(f, "f64"),
            Self::Felt => write!(f, "felt"),
            Self::Mut => write!(f, "mut"),
            Self::DoubleQuote => write!(f, "\""),
            Self::Colon => write!(f, ":"),
            Self::Semicolon => write!(f, ";"),
            Self::Comma => write!(f, ","),
            Self::Dot => write!(f, "."),
            Self::LParen => write!(f, "("),
            Self::RParen => write!(f, ")"),
            Self::LBracket => write!(f, "["),
            Self::RBracket => write!(f, "]"),
            Self::LBrace => write!(f, "{{"),
            Self::RBrace => write!(f, "}}"),
            Self::Equal => write!(f, "="),
            Self::RDoubleArrow => write!(f, "=>"),
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::RArrow => write!(f, "->"),
            Self::Star => write!(f, "*"),
            Self::Ampersand => write!(f, "&"),
            Self::Bang => write!(f, "!"),
            Self::At => write!(f, "@"),
        }
    }
}

macro_rules! pop {
    ($lex:ident) => {{
        $lex.skip();
    }};
    ($lex:ident, $code:expr) => {{
        $lex.skip();
        $code
    }};
}

macro_rules! pop2 {
    ($lex:ident) => {{
        $lex.skip();
        $lex.skip();
    }};
    ($lex:ident, $code:expr) => {{
        $lex.skip();
        $lex.skip();
        $code
    }};
}

/// The lexer that is used to perform lexical analysis on the Miden IR grammar. The lexer implements
/// the `Iterator` trait, so in order to retrieve the tokens, you simply have to iterate over it.
///
/// # Errors
///
/// Because the lexer is implemented as an iterator over tokens, this means that you can continue
/// to get tokens even if a lexical error occurs. The lexer will attempt to recover from an error
/// by injecting tokens it expects.
///
/// If an error is unrecoverable, the lexer will continue to produce tokens, but there is no
/// guarantee that parsing them will produce meaningful results, it is primarily to assist in
/// gathering as many errors as possible.
pub struct Lexer<S> {
    /// The scanner produces a sequence of chars + location, and can be controlled
    /// The location type is SourceIndex
    scanner: Scanner<S>,

    /// The most recent token to be lexed.
    /// At the start and end, this should be Token::Eof
    token: Token,

    /// The position in the input where the current token starts
    /// At the start this will be the byte index of the beginning of the input
    token_start: SourceIndex,

    /// The position in the input where the current token ends
    /// At the start this will be the byte index of the beginning of the input
    token_end: SourceIndex,

    /// When we have reached true Eof, this gets set to true, and the only token
    /// produced after that point is Token::Eof, or None, depending on how you are
    /// consuming the lexer
    eof: bool,
}
impl<S> Lexer<S>
where
    S: Source,
{
    /// Produces an instance of the lexer with the lexical analysis to be performed on the `input`
    /// string. Note that no lexical analysis occurs until the lexer has been iterated over.
    pub fn new(scanner: Scanner<S>) -> Self {
        use miden_diagnostics::ByteOffset;

        let start = scanner.start();
        let mut lexer = Lexer {
            scanner,
            token: Token::Eof,
            token_start: start + ByteOffset(0),
            token_end: start + ByteOffset(0),
            eof: false,
        };
        lexer.advance();
        lexer
    }

    pub fn lex(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.eof && self.token == Token::Eof {
            return None;
        }

        let token = std::mem::replace(&mut self.token, Token::Eof);
        let start = self.token_start;
        let end = self.token_end;
        self.advance();
        match token {
            Token::Error(err) => Some(Err(err.into())),
            token => Some(Ok((start, token, end))),
        }
    }

    fn advance(&mut self) {
        self.advance_start();
        self.token = self.tokenize();
    }

    #[inline]
    fn advance_start(&mut self) {
        let mut position: SourceIndex;
        loop {
            let (pos, c) = self.scanner.read();

            position = pos;

            if c == '\0' {
                self.eof = true;
                return;
            }

            if c.is_whitespace() {
                self.scanner.advance();
                continue;
            }

            break;
        }

        self.token_start = position;
    }

    #[inline]
    fn pop(&mut self) -> char {
        use miden_diagnostics::ByteOffset;

        let (pos, c) = self.scanner.pop();
        self.token_end = pos + ByteOffset::from_char_len(c);
        c
    }

    #[inline]
    fn peek(&mut self) -> char {
        let (_, c) = self.scanner.peek();
        c
    }

    #[inline]
    fn read(&mut self) -> char {
        let (_, c) = self.scanner.read();
        c
    }

    #[inline]
    fn skip(&mut self) {
        self.pop();
    }

    /// Get the span for the current token in `Source`.
    #[inline]
    fn span(&self) -> SourceSpan {
        SourceSpan::new(self.token_start, self.token_end)
    }

    /// Get a string slice of the current token.
    #[inline]
    fn slice(&self) -> &str {
        self.scanner.slice(self.span())
    }

    #[inline]
    fn skip_whitespace(&mut self) {
        let mut c: char;
        loop {
            c = self.read();

            if !c.is_whitespace() {
                break;
            }

            self.skip();
        }
    }

    fn tokenize(&mut self) -> Token {
        let c = self.read();

        if c == '\\' {
            match self.peek() {
                '\\' => {
                    self.skip();
                    self.skip();
                    self.lex_comment()
                }
                _ => Token::Error(LexicalError::UnexpectedCharacter {
                    start: self.span().start(),
                    found: c,
                }),
            };
        }

        if c == '\0' {
            self.eof = true;
            return Token::Eof;
        }

        if c.is_whitespace() {
            self.skip_whitespace();
        }

        match self.read() {
            ',' => pop!(self, Token::Comma),
            '.' => pop!(self, Token::Dot),
            ':' => pop!(self, Token::Colon),
            ';' => pop!(self, Token::Semicolon),
            '"' => pop!(self, Token::DoubleQuote),
            '(' => pop!(self, Token::LParen),
            ')' => pop!(self, Token::RParen),
            '[' => pop!(self, Token::LBracket),
            ']' => pop!(self, Token::RBracket),
            '{' => pop!(self, Token::LBrace),
            '}' => pop!(self, Token::RBrace),
            '=' => match self.peek() {
                '>' => pop2!(self, Token::RDoubleArrow),
                _ => pop!(self, Token::Equal),
            },
            '+' => pop!(self, Token::Plus),
            '-' => match self.peek() {
                '>' => pop2!(self, Token::RArrow),
                _ => pop!(self, Token::Minus),
            },
            '*' => pop!(self, Token::Star),
            '&' => pop!(self, Token::Ampersand),
            '!' => pop!(self, Token::Bang),
            '@' => pop!(self, Token::At),
            '0' => match self.peek() {
                'x' => {
                    self.skip();
                    self.skip();
                    self.lex_hex()
                }
                '0'..='9' => self.lex_number(),
                _ => Token::Error(LexicalError::UnexpectedCharacter {
                    start: self.span().start(),
                    found: c,
                }),
            },
            '1'..='9' => self.lex_number(),
            'a'..='z' => self.lex_keyword_or_ident(),
            'A'..='Z' => self.lex_identifier(),
            c => Token::Error(LexicalError::UnexpectedCharacter {
                start: self.span().start(),
                found: c,
            }),
        }
    }

    fn lex_comment(&mut self) -> Token {
        let mut c;
        loop {
            c = self.read();

            if c == '\n' {
                break;
            }

            if c == '\0' {
                self.eof = true;
                break;
            }

            self.skip();
        }

        Token::Comment
    }

    #[inline]
    fn lex_keyword_or_ident(&mut self) -> Token {
        let c = self.pop();
        debug_assert!(c.is_ascii_alphabetic() && c.is_lowercase());

        if self.skip_ident() {
            Token::FuncIdent(Symbol::intern(self.slice()))
        }
        else {
            Token::from_keyword_or_ident(self.slice())
        }
    }

    #[inline]
    fn lex_identifier(&mut self) -> Token {
        let c = self.pop();
        debug_assert!(c.is_ascii_alphabetic());

        if self.skip_ident() {
            Token::FuncIdent(Symbol::intern(self.slice()))
        }
        else {
            Token::Ident(Symbol::intern(self.slice()))
        }
    }

    // Returns true if the identifier is a function identifier (contains double colons), false otherwise
    fn skip_ident(&mut self) -> bool {
        let mut func_ident = false;
        loop {
            match self.read() {
                '_' => self.skip(),
                '0'..='9' => self.skip(),
                ':' => {
                    match self.peek() {
                        ':' => {
                            func_ident = true;
                            self.skip();
                            self.skip()
                        },
                        _ => break,
                    }
                }
                c if c.is_ascii_alphabetic() => self.skip(),
                _ => break,
            }
        };
        func_ident
    }

    #[inline]
    fn lex_number(&mut self) -> Token {
        let mut num = String::new();

        // Expect the first character to be a digit
        debug_assert!(self.read().is_ascii_digit());

        while let '0'..='9' = self.read() {
            num.push(self.pop());
        }

        match num.parse::<u128>() {
            Ok(i) => Token::Num(i),
            Err(err) => Token::Error(LexicalError::InvalidInt {
                span: self.span(),
                reason: err.kind().clone(),
            }),
        }
    }

    #[inline]
    fn lex_hex(&mut self) -> Token {
        let mut res: Vec<u8> = Vec::new();

        loop {
            match self.read() {
                '0'..='9' | 'a'..='f' | 'A'..='F' => {
                    res.push(self.pop() as u8);
                }
                _ => {
                    break;
                }
            }
        }

        Token::Hex(res)
    }
}

impl<S> Iterator for Lexer<S>
where
    S: Source,
{
    type Item = Lexed;

    fn next(&mut self) -> Option<Self::Item> {
        let mut res = self.lex();
        while let Some(Ok((_, Token::Comment, _))) = res {
            res = self.lex();
        }
        res
    }
}
