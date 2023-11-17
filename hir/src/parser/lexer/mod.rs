mod error;
mod token;

use core::{num::IntErrorKind, ops::Range};

use miden_diagnostics::{SourceIndex, SourceSpan};
use miden_parsing::{Scanner, Source};
use num_traits::Num;

use crate::{parser::ParseError, Block, Symbol, Value};

pub use self::error::LexicalError;
pub use self::token::Token;

/// The value produced by the [Lexer] when iterated
pub type Lexed = Result<(SourceIndex, Token, SourceIndex), ParseError>;

/// Pops a single token from the [Lexer]
macro_rules! pop {
    ($lex:ident) => {{
        $lex.skip();
    }};
    ($lex:ident, $code:expr) => {{
        $lex.skip();
        $code
    }};
}

/// Pops two tokens from the [Lexer]
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

    #[inline]
    fn slice_span(&self, span: impl Into<Range<usize>>) -> &str {
        self.scanner.slice(span)
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

        if c == '/' {
            match self.peek() {
                '/' => {
                    self.skip();
                    self.skip();
                    return self.lex_comment();
                }
                c => Token::Error(LexicalError::UnexpectedCharacter {
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
            '"' => self.lex_quoted_identifier(),
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
            '+' => self.lex_number(),
            '-' => match self.peek() {
                '>' => pop2!(self, Token::RArrow),
                _ => self.lex_number(),
            },
            '*' => pop!(self, Token::Star),
            '&' => pop!(self, Token::Ampersand),
            '!' => pop!(self, Token::Bang),
            '@' => pop!(self, Token::At),
            '$' => pop!(self, Token::Dollar),
            '#' => pop!(self, Token::Hash),
            '0' => match self.peek() {
                'x' => {
                    self.skip();
                    self.skip();
                    self.lex_hex()
                }
                '0'..='9' => self.lex_number(),
                _ => pop!(self, Token::Int(0)),
            },
            '1'..='9' => self.lex_number(),
            'a'..='z' => self.lex_keyword_or_ident(),
            'A'..='Z' => self.lex_identifier(),
            '_' => match self.peek() {
                c if c.is_ascii_alphanumeric() || c == '_' => self.lex_identifier(),
                _ => pop!(self, Token::Underscore),
            },
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

        if self.skip_ident(true) {
            self.handle_function_ident(self.slice())
        } else {
            let s = self.slice();
            if let Some(rest) = s.strip_prefix('v') {
                return match rest.parse::<u32>() {
                    Ok(id) => Token::Value(Value::from_u32(id)),
                    Err(_) => Token::from_keyword_or_ident(s),
                };
            }
            if let Some(rest) = s.strip_prefix("block") {
                return match rest.parse::<u32>() {
                    Ok(id) => Token::Block(Block::from_u32(id)),
                    Err(_) => Token::from_keyword_or_ident(s),
                };
            }
            Token::from_keyword_or_ident(self.slice())
        }
    }

    fn lex_quoted_identifier(&mut self) -> Token {
        use miden_diagnostics::ByteOffset;

        let quote = self.pop();
        debug_assert!(quote == '"' || quote == '\'');
        let mut buf = None;
        loop {
            match self.read() {
                '\0' if quote == '"' => {
                    return Token::Error(LexicalError::UnclosedString { span: self.span() });
                }
                c if c == quote => {
                    let span = self.span().shrink_front(ByteOffset(1));

                    self.skip();
                    self.advance_start();
                    if self.read() == quote {
                        self.skip();

                        buf = Some(self.slice_span(span).to_string());
                        continue;
                    }

                    let symbol = if let Some(mut buf) = buf {
                        buf.push_str(self.slice_span(span));
                        Symbol::intern(&buf)
                    } else {
                        Symbol::intern(self.slice_span(span))
                    };

                    return Token::Ident(symbol);
                }
                _ => {
                    self.skip();
                    continue;
                }
            }
        }
    }

    #[inline]
    fn lex_identifier(&mut self) -> Token {
        let c = self.pop();
        debug_assert!(c.is_ascii_alphabetic() || c == '_');

        if self.skip_ident(false) {
            self.handle_function_ident(self.slice())
        } else {
            Token::Ident(Symbol::intern(self.slice()))
        }
    }

    // Returns true if the identifier is a namespaced identifier (contains double colons), false otherwise
    fn skip_ident(&mut self, allow_dot: bool) -> bool {
        let mut is_namespaced = false;
        loop {
            match self.read() {
                '_' => self.skip(),
                '.' if allow_dot => {
                    // We only allow '.' when followed by a alpha character
                    match self.peek() {
                        c if c.is_ascii_alphabetic() => self.skip(),
                        _ => break,
                    }
                }
                '0'..='9' => self.skip(),
                ':' => match self.peek() {
                    ':' => {
                        is_namespaced = true;
                        self.skip();
                        self.skip()
                    }
                    _ => break,
                },
                c if c.is_ascii_alphabetic() => self.skip(),
                _ => break,
            }
        }
        is_namespaced
    }

    fn handle_function_ident(&self, s: &str) -> Token {
        if let Some((offset, c)) = s
            .char_indices()
            .find(|(_, c)| *c == '.' || c.is_whitespace())
        {
            Token::Error(LexicalError::UnexpectedCharacter {
                start: self.span().start() + offset,
                found: c,
            })
        } else {
            match s.rsplit_once("::").unwrap() {
                (_, function) if function.is_empty() => {
                    Token::Error(LexicalError::InvalidFunctionIdentifier { span: self.span() })
                }
                (module, _) if module.is_empty() => {
                    Token::Error(LexicalError::InvalidModuleIdentifier { span: self.span() })
                }
                (module, function) => {
                    Token::FunctionIdent((Symbol::intern(module), Symbol::intern(function)))
                }
            }
        }
    }

    #[inline]
    fn lex_number(&mut self) -> Token {
        use num_bigint::BigInt;

        let mut num = String::new();

        // Expect the first character to be a digit or sign
        let c = self.read();
        debug_assert!(c == '-' || c == '+' || c.is_ascii_digit());
        if c == '-' {
            num.push(self.pop());
        } else if c == '+' {
            self.skip();
        }

        while let '0'..='9' = self.read() {
            num.push(self.pop());
        }

        match num.parse::<isize>() {
            Ok(value) => Token::Int(value),
            Err(err) => match err.kind() {
                IntErrorKind::PosOverflow | IntErrorKind::NegOverflow => {
                    let value = BigInt::from_str_radix(&num, 10).expect("invalid bigint");
                    Token::BigInt(value)
                }
                reason => Token::Error(LexicalError::InvalidInt {
                    span: self.span(),
                    reason: reason.clone(),
                }),
            },
        }
    }

    #[inline]
    fn lex_hex(&mut self) -> Token {
        let mut res: Vec<u8> = Vec::new();

        // Expect the first character to be a valid hexadecimal digit
        debug_assert!(self.read().is_ascii_hexdigit());

        loop {
            // If we hit a non-hex digit, we're done
            let c1 = self.read();
            if !c1.is_ascii_hexdigit() {
                break;
            }
            self.skip();

            // All hex-encoded bytes are zero-padded, and thus occur
            // in pairs, if we observe a non-hex digit at this point,
            // it is invalid
            let c2 = self.read();
            if !c2.is_ascii_hexdigit() {
                return Token::Error(LexicalError::InvalidInt {
                    span: self.span(),
                    reason: IntErrorKind::InvalidDigit,
                });
            }
            self.skip();

            // The data is big-endian, so shift the first char left by 4 bits, and
            // add to the value of the second char
            let byte = (c1.to_digit(16).unwrap() << 4) + c2.to_digit(16).unwrap();
            res.push(byte as u8);
        }

        Token::Hex(res.into())
    }
}

impl<S> Iterator for Lexer<S>
where
    S: Source,
{
    type Item = Lexed;

    fn next(&mut self) -> Option<Self::Item> {
        let last = self.token.clone();
        let mut res = self.lex();
        while let Some(Ok((_, Token::Comment, _))) = res {
            res = self.lex();
        }
        match res {
            Some(Ok((start, Token::FunctionIdent((mid, fid)), end))) => {
                match last {
                    // If we parse a namespaced identifier right after the `module` or `kernel` keyword,
                    // it is a module name, not a function name, so convert it into a Ident token when this
                    // happens.
                    Token::Module | Token::Kernel => {
                        let module_name = format!("{}::{}", mid, fid);
                        let module_id = Symbol::intern(&module_name);
                        Some(Ok((start, Token::Ident(module_id), end)))
                    }
                    _ => Some(Ok((start, Token::FunctionIdent((mid, fid)), end))),
                }
            }
            res => res,
        }
    }
}
