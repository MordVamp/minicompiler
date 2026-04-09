use crate::parser::ast::Position;
use std::fmt;

#[derive(Debug, Clone)]
pub struct SemanticError {
    pub message: String,
    pub position: Position,
}

impl SemanticError {
    pub fn new(message: String, position: Position) -> Self {
        Self { message, position }
    }
}

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Semantic Error at line {}, column {}: {}",
            self.position.line, self.position.column, self.message
        )
    }
}
