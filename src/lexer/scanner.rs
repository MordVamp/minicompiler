use super::error::LexicalError;
use super::token::{LiteralValue, Token, TokenType};
use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

pub struct Scanner<'a> {
    source: &'a str,
    chars: Peekable<Chars<'a>>,
    line: usize,
    column: usize,
    start: usize,
    current: usize,
    keywords: HashMap<&'static str, TokenType>,
}

struct ScannerState {
    start: usize,
    current: usize,
    line: usize,
    column: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("if", TokenType::If);
        keywords.insert("else", TokenType::Else);
        keywords.insert("while", TokenType::While);
        keywords.insert("for", TokenType::For);
        keywords.insert("int", TokenType::Int);
        keywords.insert("float", TokenType::Float);
        keywords.insert("bool", TokenType::Bool);
        keywords.insert("return", TokenType::Return);
        keywords.insert("true", TokenType::True);
        keywords.insert("false", TokenType::False);
        keywords.insert("void", TokenType::Void);
        keywords.insert("struct", TokenType::Struct);
        keywords.insert("fn", TokenType::Fn);

        Self {
            source,
            chars: source.chars().peekable(),
            line: 1,
            column: 1,
            start: 0,
            current: 0,
            keywords,
        }
    }

    pub fn get_line(&self) -> usize {
        self.line
    }

    pub fn get_column(&self) -> usize {
        self.column
    }

    pub fn is_at_end(&mut self) -> bool {
        self.chars.peek().is_none()
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::EndOfFile, LiteralValue::None);
        }

        let c = self.advance().unwrap();

        match c {
            '(' => return self.simple_token(TokenType::LParen),
            ')' => return self.simple_token(TokenType::RParen),
            '{' => return self.simple_token(TokenType::LBrace),
            '}' => return self.simple_token(TokenType::RBrace),
            '[' => return self.simple_token(TokenType::LBracket),
            ']' => return self.simple_token(TokenType::RBracket),
            ';' => return self.simple_token(TokenType::Semicolon),
            ',' => return self.simple_token(TokenType::Comma),
            ':' => return self.simple_token(TokenType::Colon),
            '+' => {
                if self.r#match('=') {
                    self.simple_token(TokenType::PlusEqual)
                } else {
                    self.simple_token(TokenType::Plus)
                }
            }
            '-' => {
                if self.r#match('=') {
                    self.simple_token(TokenType::MinusEqual)
                } else {
                    self.simple_token(TokenType::Minus)
                }
            }
            '*' => {
                if self.r#match('=') {
                    self.simple_token(TokenType::StarEqual)
                } else {
                    self.simple_token(TokenType::Star)
                }
            }
            '/' => {
                if self.r#match('/') {
                    self.single_line_comment();
                    return self.next_token();
                } else if self.r#match('*') {
                    if let Err(e) = self.block_comment() {
                        return self.error_token(e);
                    }
                    return self.next_token();
                } else if self.r#match('=') {
                    self.simple_token(TokenType::SlashEqual)
                } else {
                    self.simple_token(TokenType::Slash)
                }
            }
            '=' => {
                if self.r#match('=') {
                    self.simple_token(TokenType::EqualEqual)
                } else {
                    self.simple_token(TokenType::Equal)
                }
            }
            '!' => {
                if self.r#match('=') {
                    self.simple_token(TokenType::NotEqual)
                } else {
                    self.simple_token(TokenType::Bang)
                }
            }
            '<' => {
                if self.r#match('=') {
                    self.simple_token(TokenType::LessEqual)
                } else {
                    self.simple_token(TokenType::Less)
                }
            }
            '>' => {
                if self.r#match('=') {
                    self.simple_token(TokenType::GreaterEqual)
                } else {
                    self.simple_token(TokenType::Greater)
                }
            }
            '&' => {
                if self.r#match('&') {
                    self.simple_token(TokenType::AndAnd)
                } else {
                    self.error_token(LexicalError::InvalidCharacter('&'))
                }
            }
            '|' => {
                if self.r#match('|') {
                    self.simple_token(TokenType::OrOr)
                } else {
                    self.error_token(LexicalError::InvalidCharacter('|'))
                }
            }
            '"' => return self.string(),
            _ if c.is_ascii_digit() => return self.number(c),
            _ if is_identifier_start(c) => return self.identifier(),
            _ => self.error_token(LexicalError::InvalidCharacter(c)),
        }
    }

    pub fn peek_token(&mut self) -> Token {
        let snapshot = self.save();
        let token = self.next_token();
        self.restore(snapshot);
        token
    }

    // -------------------------------------------------------------------------
    // Private helpers
    // -------------------------------------------------------------------------

    fn advance(&mut self) -> Option<char> {
        let c = self.chars.next();
        if let Some(ch) = c {
            self.current += ch.len_utf8();
            self.column += 1;
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            }
        }
        c
    }

    fn r#match(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c == ' ' || c == '\t' || c == '\r' {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn single_line_comment(&mut self) {
        while let Some(c) = self.peek() {
            if c == '\n' {
                break;
            }
            self.advance();
        }
    }

    fn block_comment(&mut self) -> Result<(), LexicalError> {
        let mut nesting = 1;
        while nesting > 0 {
            match self.peek() {
                None => return Err(LexicalError::UnterminatedComment),
                Some('/') => {
                    self.advance();
                    if self.peek() == Some('*') {
                        self.advance();
                        nesting += 1;
                    }
                }
                Some('*') => {
                    self.advance();
                    if self.peek() == Some('/') {
                        self.advance();
                        nesting -= 1;
                    }
                }
                Some(_) => {
                    self.advance();
                }
            }
        }
        Ok(())
    }

    fn string(&mut self) -> Token {
        let mut value = String::new();
        let start_line = self.line;
        let start_column = self.column - 1;

        while let Some(c) = self.peek() {
            if c == '"' {
                self.advance();
                return Token::new(
                    TokenType::StringLiteral,
                    &self.source[self.start..self.current],
                    start_line,
                    start_column,
                    LiteralValue::String(value),
                );
            }
            if c == '\n' {
                break;
            }
            let ch = self.advance().unwrap();
            value.push(ch);
        }

        self.error_token(LexicalError::UnterminatedString)
    }

    fn number(&mut self, _first: char) -> Token {
        let start_line = self.line;
        let start_column = self.column - 1;

        let mut is_float = false;
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }

        if self.peek() == Some('.') {
            is_float = true;
            self.advance();

            if !self.peek().map_or(false, |c| c.is_ascii_digit()) {
                return self.error_token(LexicalError::MalformedNumber(
                    self.source[self.start..self.current].to_string(),
                ));
            }
            while let Some(c) = self.peek() {
                if c.is_ascii_digit() {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        let lexeme = &self.source[self.start..self.current];
        if is_float {
            match lexeme.parse::<f64>() {
                Ok(val) => Token::new(
                    TokenType::FloatLiteral,
                    lexeme,
                    start_line,
                    start_column,
                    LiteralValue::Float(val),
                ),
                Err(_) => self.error_token(LexicalError::MalformedNumber(lexeme.to_string())),
            }
        } else {
            match lexeme.parse::<i64>() {
                Ok(val) => {
                    if val < i32::MIN as i64 || val > i32::MAX as i64 {
                        self.error_token(LexicalError::IntegerOutOfRange(lexeme.to_string()))
                    } else {
                        Token::new(
                            TokenType::IntLiteral,
                            lexeme,
                            start_line,
                            start_column,
                            LiteralValue::Integer(val),
                        )
                    }
                }
                Err(_) => self.error_token(LexicalError::MalformedNumber(lexeme.to_string())),
            }
        }
    }

    fn identifier(&mut self) -> Token {
        let start_line = self.line;
        let start_column = self.column - 1;

        while let Some(c) = self.peek() {
            if is_identifier_continue(c) {
                self.advance();
            } else {
                break;
            }
        }

        let lexeme = &self.source[self.start..self.current];
        if let Some(&token_type) = self.keywords.get(lexeme) {
            let literal = match token_type {
                TokenType::True => LiteralValue::Boolean(true),
                TokenType::False => LiteralValue::Boolean(false),
                _ => LiteralValue::None,
            };
            Token::new(token_type, lexeme, start_line, start_column, literal)
        } else {
            if lexeme.len() > 255 {
                self.error_token(LexicalError::MalformedNumber(lexeme.to_string()))
            } else {
                Token::new(
                    TokenType::Identifier,
                    lexeme,
                    start_line,
                    start_column,
                    LiteralValue::None,
                )
            }
        }
    }

    fn simple_token(&self, token_type: TokenType) -> Token {
        Token::simple(
            token_type,
            &self.source[self.start..self.current],
            self.line,
            self.column - (self.current - self.start),
        )
    }

    fn make_token(&self, token_type: TokenType, literal: LiteralValue) -> Token {
        Token::new(
            token_type,
            &self.source[self.start..self.current],
            self.line,
            self.column - (self.current - self.start),
            literal,
        )
    }

    fn error_token(&self, err: LexicalError) -> Token {
        Token::error(
            format!("{}", err),
            self.line,
            self.column - (self.current - self.start),
        )
    }

    fn save(&self) -> ScannerState {
        ScannerState {
            start: self.start,
            current: self.current,
            line: self.line,
            column: self.column,
        }
    }

    fn restore(&mut self, state: ScannerState) {
        self.start = state.start;
        self.current = state.current;
        self.line = state.line;
        self.column = state.column;
        self.chars = self.source[self.current..].chars().peekable();
    }
}

fn is_identifier_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_identifier_continue(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}