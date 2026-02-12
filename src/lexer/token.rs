use std::fmt;

// -----------------------------------------------------------------------------
// TokenType: enumeration of all possible token kinds.
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    // Keywords
    If,
    Else,
    While,
    For,
    Int,
    Float,
    Bool,
    Return,
    True,
    False,
    Void,
    Struct,
    Fn,

    // Literals
    Identifier,
    IntLiteral,
    FloatLiteral,
    StringLiteral,
    BoolLiteral,

    // Operators (single & multi-character)
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    AndAnd,
    OrOr,
    Bang,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,

    // Delimiters
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Semicolon,
    Comma,
    Colon,

    // Special
    EndOfFile,
    Error,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// -----------------------------------------------------------------------------
// LiteralValue: a discriminated union for literal values extracted from source.
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    None,
}

impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiteralValue::Integer(i) => write!(f, "{}", i),
            LiteralValue::Float(fl) => write!(f, "{}", fl),
            LiteralValue::String(s) => write!(f, "\"{}\"", s),
            LiteralValue::Boolean(b) => write!(f, "{}", b),
            LiteralValue::None => write!(f, ""),
        }
    }
}

// -----------------------------------------------------------------------------
// Token: unit of output from the scanner.
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
    pub literal: LiteralValue,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: impl Into<String>,
        line: usize,
        column: usize,
        literal: LiteralValue,
    ) -> Self {
        Self {
            token_type,
            lexeme: lexeme.into(),
            line,
            column,
            literal,
        }
    }

    pub fn simple(
        token_type: TokenType,
        lexeme: impl Into<String>,
        line: usize,
        column: usize,
    ) -> Self {
        Self::new(token_type, lexeme, line, column, LiteralValue::None)
    }

    pub fn error(lexeme: impl Into<String>, line: usize, column: usize) -> Self {
        Self::new(
            TokenType::Error,
            lexeme,
            line,
            column,
            LiteralValue::None,
        )
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let literal_str = if self.literal != LiteralValue::None {
            format!(" {}", self.literal)
        } else {
            String::new()
        };
        write!(
            f,
            "{}:{} {} \"{}\"{}",
            self.line, self.column, self.token_type, self.lexeme, literal_str
        )
    }
}