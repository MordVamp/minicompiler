use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TokenType {
    // Keywords
    If, Else, While, For, Int, Float, Bool, Return, True, False, Void, Struct, Fn,
    // Literals
    Identifier, IntLiteral, FloatLiteral, StringLiteral, BoolLiteral,
    // Operators
    Plus, Minus, Star, Slash, Percent, Equal, EqualEqual, NotEqual,
    Less, LessEqual, Greater, GreaterEqual, AndAnd, OrOr, Bang,
    PlusEqual, MinusEqual, StarEqual, SlashEqual,
    // Delimiters
    LParen, RParen, LBrace, RBrace, LBracket, RBracket, Semicolon, Comma, Colon,
    // Special
    EndOfFile, Error,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TokenType::If => "KW_IF",
            TokenType::Else => "KW_ELSE",
            TokenType::While => "KW_WHILE",
            TokenType::For => "KW_FOR",
            TokenType::Int => "KW_INT",
            TokenType::Float => "KW_FLOAT",
            TokenType::Bool => "KW_BOOL",
            TokenType::Return => "KW_RETURN",
            TokenType::True => "KW_TRUE",
            TokenType::False => "KW_FALSE",
            TokenType::Void => "KW_VOID",
            TokenType::Struct => "KW_STRUCT",
            TokenType::Fn => "KW_FN",
            TokenType::Identifier => "IDENTIFIER",
            TokenType::IntLiteral => "INT_LITERAL",
            TokenType::FloatLiteral => "FLOAT_LITERAL",
            TokenType::StringLiteral => "STRING_LITERAL",
            TokenType::BoolLiteral => "BOOL_LITERAL",
            TokenType::Plus => "PLUS",
            TokenType::Minus => "MINUS",
            TokenType::Star => "STAR",
            TokenType::Slash => "SLASH",
            TokenType::Percent => "PERCENT",
            TokenType::Equal => "ASSIGN",
            TokenType::EqualEqual => "EQ",
            TokenType::NotEqual => "NEQ",
            TokenType::Less => "LT",
            TokenType::LessEqual => "LTE",
            TokenType::Greater => "GT",
            TokenType::GreaterEqual => "GTE",
            TokenType::AndAnd => "AND",
            TokenType::OrOr => "OR",
            TokenType::Bang => "NOT",
            TokenType::PlusEqual => "PLUS_ASSIGN",
            TokenType::MinusEqual => "MINUS_ASSIGN",
            TokenType::StarEqual => "STAR_ASSIGN",
            TokenType::SlashEqual => "SLASH_ASSIGN",
            TokenType::LParen => "LPAREN",
            TokenType::RParen => "RPAREN",
            TokenType::LBrace => "LBRACE",
            TokenType::RBrace => "RBRACE",
            TokenType::LBracket => "LBRACKET",
            TokenType::RBracket => "RBRACKET",
            TokenType::Semicolon => "SEMICOLON",
            TokenType::Comma => "COMMA",
            TokenType::Colon => "COLON",
            TokenType::EndOfFile => "END_OF_FILE",
            TokenType::Error => "ERROR",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
        Self::new(TokenType::Error, lexeme, line, column, LiteralValue::None)
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
