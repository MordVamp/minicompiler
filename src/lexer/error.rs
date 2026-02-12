use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum LexicalError {
    #[error("invalid character: '{0}'")]
    InvalidCharacter(char),

    #[error("unterminated string literal")]
    UnterminatedString,

    #[error("unterminated block comment")]
    UnterminatedComment,

    #[error("malformed number: '{0}'")]
    MalformedNumber(String),

    #[error("integer literal out of range: {0}")]
    IntegerOutOfRange(String),
}