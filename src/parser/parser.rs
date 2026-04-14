use crate::lexer::token::{Token, TokenType};
use crate::parser::ast::*;
use crate::parser::symbol_table::{SymbolTable, SymbolKind};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    pub symbols: SymbolTable,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0, symbols: SymbolTable::new() }
    }

    pub fn parse(&mut self) -> Result<ProgramNode, String> {
        let mut declarations = Vec::new();
        while !self.is_at_end() {
            declarations.push(self.parse_declaration()?);
        }

        Ok(ProgramNode {
            declarations,
            position: Position { line: 1, column: 1 },
        })
    }

    // ---------------------------------------------------------
    // Declaration Parsing
    // ---------------------------------------------------------

    fn parse_declaration(&mut self) -> Result<DeclarationNode, String> {
        if self.match_token(&[TokenType::Fn]) {
            self.parse_function_declaration()
        } else if self.match_token(&[TokenType::Struct]) {
            self.parse_struct_declaration()
        } else {
            // VarDecl or we default into parsing it as VarDeclStmt essentially
            let _pos = self.current_position();
            let var_decl = self.parse_var_declaration()?;
            self.consume(TokenType::Semicolon, "Expected ';' after variable declaration.")?;
            Ok(var_decl)
        }
    }

    fn parse_function_declaration(&mut self) -> Result<DeclarationNode, String> {
        let name_token = self.consume(TokenType::Identifier, "Expected function name.")?.clone();
        let pos = Position { line: name_token.line, column: name_token.column };
        
        self.consume(TokenType::LParen, "Expected '(' after function name.")?;
        
        let mut parameters = Vec::new();
        if !self.check(TokenType::RParen) {
            loop {
                let p_type = self.parse_type()?.lexeme.clone();
                let p_name = self.consume(TokenType::Identifier, "Expected parameter name.")?.lexeme.clone();
                parameters.push(ParamNode { var_type: p_type, name: p_name });
                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        self.consume(TokenType::RParen, "Expected ')' after parameters.")?;
        
        let mut return_type = "void".to_string();
        if self.match_token(&[TokenType::Minus]) { // "->" is two tokens: Minus, Greater
            self.consume(TokenType::Greater, "Expected '>' after '-' for return type.")?;
            return_type = self.parse_type()?.lexeme.clone();
        } else if self.match_token(&[TokenType::Identifier, TokenType::Int, TokenType::Float, TokenType::Bool, TokenType::Void]) {
            // Or just single token return type without arrow
            return_type = self.previous().lexeme.clone();
        }
        
        let _ = self.symbols.define(name_token.lexeme.clone(), SymbolKind::Function, return_type.clone(), pos.line, pos.column);
        self.symbols.enter_scope();
        for p in &parameters {
            let _ = self.symbols.define(p.name.clone(), SymbolKind::Variable, p.var_type.clone(), pos.line, pos.column);
        }

        self.consume(TokenType::LBrace, "Expected '{' before function body.")?;
        let body = self.parse_block_statement()?;
        self.symbols.exit_scope();
        
        Ok(DeclarationNode::FunctionDecl {
            name: name_token.lexeme,
            return_type,
            parameters,
            body: Box::new(body),
            position: pos,
        })
    }

    fn parse_struct_declaration(&mut self) -> Result<DeclarationNode, String> {
        let name_token = self.consume(TokenType::Identifier, "Expected struct name.")?.clone();
        let pos = Position { line: name_token.line, column: name_token.column };
        self.consume(TokenType::LBrace, "Expected '{' before struct body.")?;
        
        let mut fields = Vec::new();
        while !self.check(TokenType::RBrace) && !self.is_at_end() {
            let decl = self.parse_var_declaration()?;
            self.consume(TokenType::Semicolon, "Expected ';' after struct field.")?;
            fields.push(StatementNode::VarDeclStmt {
                decl,
                position: self.current_position(),
            });
        }
        
        self.consume(TokenType::RBrace, "Expected '}' after struct body.")?;
        Ok(DeclarationNode::StructDecl {
            name: name_token.lexeme,
            fields,
            position: pos,
        })
    }

    fn parse_var_declaration(&mut self) -> Result<DeclarationNode, String> {
        let type_token = self.parse_type()?.clone();
        let name_token = self.consume(TokenType::Identifier, "Expected variable name.")?.clone();
        let pos = Position { line: type_token.line, column: type_token.column };
        
        let initializer = if self.match_token(&[TokenType::Equal]) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        let _ = self.symbols.define(name_token.lexeme.clone(), SymbolKind::Variable, type_token.lexeme.clone(), pos.line, pos.column);

        Ok(DeclarationNode::VarDecl {
            var_type: type_token.lexeme,
            name: name_token.lexeme,
            initializer,
            position: pos,
        })
    }

    fn parse_type(&mut self) -> Result<&Token, String> {
        if self.match_token(&[TokenType::Int, TokenType::Float, TokenType::Bool, TokenType::Void, TokenType::Identifier]) {
            Ok(self.previous())
        } else {
            Err(self.error(self.peek(), "Expected a type."))
        }
    }

    // ---------------------------------------------------------
    // Statement Parsing
    // ---------------------------------------------------------

    fn parse_statement(&mut self) -> Result<StatementNode, String> {
        if self.match_token(&[TokenType::LBrace]) {
            self.parse_block_statement()
        } else if self.match_token(&[TokenType::If]) {
            self.parse_if_statement()
        } else if self.match_token(&[TokenType::While]) {
            self.parse_while_statement()
        } else if self.match_token(&[TokenType::For]) {
            self.parse_for_statement()
        } else if self.match_token(&[TokenType::Return]) {
            self.parse_return_statement()
        } else if self.check(TokenType::Int) || self.check(TokenType::Float) || self.check(TokenType::Bool) {
            let pos = self.current_position();
            let decl = self.parse_var_declaration()?;
            self.consume(TokenType::Semicolon, "Expected ';' after variable declaration.")?;
            Ok(StatementNode::VarDeclStmt { decl, position: pos })
        } else if self.match_token(&[TokenType::Semicolon]) {
            Ok(StatementNode::Empty { position: self.current_position() })
        } else {
            self.parse_expression_statement()
        }
    }

    fn parse_block_statement(&mut self) -> Result<StatementNode, String> {
        let pos = self.current_position();
        let mut statements = Vec::new();
        
        while !self.check(TokenType::RBrace) && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }
        
        self.consume(TokenType::RBrace, "Expected '}' after block.")?;
        Ok(StatementNode::Block { statements, position: pos })
    }

    fn parse_if_statement(&mut self) -> Result<StatementNode, String> {
        let pos = self.current_position();
        self.consume(TokenType::LParen, "Expected '(' after 'if'.")?;
        let condition = self.parse_expression()?;
        self.consume(TokenType::RParen, "Expected ')' after if condition.")?;
        
        let then_branch = Box::new(self.parse_statement()?);
        
        let else_branch = if self.match_token(&[TokenType::Else]) {
            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };
        
        Ok(StatementNode::IfStmt { condition, then_branch, else_branch, position: pos })
    }

    fn parse_while_statement(&mut self) -> Result<StatementNode, String> {
        let pos = self.current_position();
        self.consume(TokenType::LParen, "Expected '(' after 'while'.")?;
        let condition = self.parse_expression()?;
        self.consume(TokenType::RParen, "Expected ')' after while condition.")?;
        let body = Box::new(self.parse_statement()?);
        Ok(StatementNode::WhileStmt { condition, body, position: pos })
    }

    fn parse_for_statement(&mut self) -> Result<StatementNode, String> {
        let pos = self.current_position();
        self.consume(TokenType::LParen, "Expected '(' after 'for'.")?;
        
        let init = if self.match_token(&[TokenType::Semicolon]) {
            None
        } else if self.check(TokenType::Int) || self.check(TokenType::Float) || self.check(TokenType::Bool) {
            let decl = self.parse_var_declaration()?;
            self.consume(TokenType::Semicolon, "Expected ';' after loop initialization.")?;
            Some(Box::new(StatementNode::VarDeclStmt { decl, position: self.current_position() }))
        } else {
            let expr = self.parse_expression()?;
            self.consume(TokenType::Semicolon, "Expected ';' after loop initialization.")?;
            Some(Box::new(StatementNode::ExprStmt { expression: expr, position: self.current_position() }))
        };

        let condition = if !self.check(TokenType::Semicolon) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        self.consume(TokenType::Semicolon, "Expected ';' after loop condition.")?;

        let update = if !self.check(TokenType::RParen) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        self.consume(TokenType::RParen, "Expected ')' after for clauses.")?;

        let body = Box::new(self.parse_statement()?);

        Ok(StatementNode::ForStmt { init, condition, update, body, position: pos })
    }

    fn parse_return_statement(&mut self) -> Result<StatementNode, String> {
        let pos = Position { line: self.previous().line, column: self.previous().column };
        let value = if !self.check(TokenType::Semicolon) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        self.consume(TokenType::Semicolon, "Expected ';' after return value.")?;
        Ok(StatementNode::ReturnStmt { value, position: pos })
    }

    fn parse_expression_statement(&mut self) -> Result<StatementNode, String> {
        let pos = self.current_position();
        let expression = self.parse_expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after expression.")?;
        Ok(StatementNode::ExprStmt { expression, position: pos })
    }

    // ---------------------------------------------------------
    // Expression Parsing
    // ---------------------------------------------------------

    fn parse_expression(&mut self) -> Result<ExpressionNode, String> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<ExpressionNode, String> {
        let expr = self.parse_logical_or()?;
        
        if self.match_token(&[TokenType::Equal, TokenType::PlusEqual, TokenType::MinusEqual, TokenType::StarEqual, TokenType::SlashEqual]) {
            let operator = self.previous().token_type;
            let value = self.parse_assignment()?; // Right-associative
            
            if let ExpressionNode::Identifier { name, position, .. } = expr {
                return Ok(ExpressionNode::Assignment {
                    target: name,
                    operator,
                    value: Box::new(value),
                    position,
                    type_info: None,
                });
            }
            return Err(self.error(self.previous(), "Invalid assignment target."));
        }
        Ok(expr)
    }

    fn parse_logical_or(&mut self) -> Result<ExpressionNode, String> {
        let mut expr = self.parse_logical_and()?;
        while self.match_token(&[TokenType::OrOr]) {
            let operator = self.previous().token_type;
            let right = self.parse_logical_and()?;
            expr = ExpressionNode::Binary {
                position: expr.position().clone(), type_info: None, // helper
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_logical_and(&mut self) -> Result<ExpressionNode, String> {
        let mut expr = self.parse_equality()?;
        while self.match_token(&[TokenType::AndAnd]) {
            let operator = self.previous().token_type;
            let right = self.parse_equality()?;
            expr = ExpressionNode::Binary {
                position: expr.position().clone(), type_info: None,
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<ExpressionNode, String> {
        let mut expr = self.parse_relational()?;
        while self.match_token(&[TokenType::EqualEqual, TokenType::NotEqual]) {
            let operator = self.previous().token_type;
            let right = self.parse_relational()?;
            expr = ExpressionNode::Binary {
                position: expr.position().clone(), type_info: None,
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_relational(&mut self) -> Result<ExpressionNode, String> {
        let mut expr = self.parse_additive()?;
        while self.match_token(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous().token_type;
            let right = self.parse_additive()?;
            expr = ExpressionNode::Binary {
                position: expr.position().clone(), type_info: None,
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_additive(&mut self) -> Result<ExpressionNode, String> {
        let mut expr = self.parse_multiplicative()?;
        while self.match_token(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous().token_type;
            let right = self.parse_multiplicative()?;
            expr = ExpressionNode::Binary {
                position: expr.position().clone(), type_info: None,
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_multiplicative(&mut self) -> Result<ExpressionNode, String> {
        let mut expr = self.parse_unary()?;
        while self.match_token(&[TokenType::Star, TokenType::Slash, TokenType::Percent]) {
            let operator = self.previous().token_type;
            let right = self.parse_unary()?;
            expr = ExpressionNode::Binary {
                position: expr.position().clone(), type_info: None,
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<ExpressionNode, String> {
        if self.match_token(&[TokenType::Bang, TokenType::Minus, TokenType::PlusPlus, TokenType::MinusMinus]) {
            let operator = self.previous().token_type;
            let pos = Position { line: self.previous().line, column: self.previous().column };
            let right = self.parse_unary()?;
            return Ok(ExpressionNode::Unary {
                operator,
                operand: Box::new(right),
                position: pos,
                type_info: None,
            });
        }
        self.parse_postfix()
    }

    fn parse_postfix(&mut self) -> Result<ExpressionNode, String> {
        let mut expr = self.parse_primary()?;
        loop {
            if self.match_token(&[TokenType::PlusPlus, TokenType::MinusMinus]) {
                let operator = self.previous().token_type;
                let pos = Position { line: self.previous().line, column: self.previous().column };
                expr = ExpressionNode::Postfix {
                    target: Box::new(expr),
                    operator,
                    position: pos,
                    type_info: None,
                };
            } else if self.match_token(&[TokenType::Dot]) {
                let pos = self.current_position();
                let member = self.consume(TokenType::Identifier, "Expected member name after '.'.")?.lexeme.clone();
                expr = ExpressionNode::MemberAccess {
                    target: Box::new(expr),
                    member,
                    position: pos,
                    type_info: None,
                };
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<ExpressionNode, String> {
        let pos = self.current_position();
        
        if self.match_token(&[TokenType::False]) {
            return Ok(ExpressionNode::Literal { value: crate::lexer::token::LiteralValue::Boolean(false), position: pos, type_info: None });
        }
        if self.match_token(&[TokenType::True]) {
            return Ok(ExpressionNode::Literal { value: crate::lexer::token::LiteralValue::Boolean(true), position: pos, type_info: None });
        }
        if self.match_token(&[TokenType::IntLiteral, TokenType::FloatLiteral, TokenType::StringLiteral, TokenType::BoolLiteral]) {
            return Ok(ExpressionNode::Literal { value: self.previous().literal.clone(), position: pos, type_info: None });
        }
        
        if self.match_token(&[TokenType::Identifier]) {
            let name = self.previous().lexeme.clone();
            
            // Function call
            if self.match_token(&[TokenType::LParen]) {
                let mut arguments = Vec::new();
                if !self.check(TokenType::RParen) {
                    loop {
                        arguments.push(self.parse_expression()?);
                        if !self.match_token(&[TokenType::Comma]) {
                            break;
                        }
                    }
                }
                self.consume(TokenType::RParen, "Expected ')' after arguments.")?;
                return Ok(ExpressionNode::Call { callee: name, arguments, position: pos, type_info: None });
            }
            
            return Ok(ExpressionNode::Identifier { name, position: pos, type_info: None });
        }
        
        if self.match_token(&[TokenType::LParen]) {
            let expr = self.parse_expression()?;
            self.consume(TokenType::RParen, "Expected ')' after expression.")?;
            return Ok(expr); // Grouping drops out in AST just keeping inner expr
        }

        Err(self.error(self.peek(), "Expected expression."))
    }

    // ---------------------------------------------------------
    // Helpers
    // ---------------------------------------------------------

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for &t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, t: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == t
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EndOfFile
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, t: TokenType, msg: &str) -> Result<&Token, String> {
        if self.check(t) {
            Ok(self.advance())
        } else {
            Err(self.error(self.peek(), msg))
        }
    }

    fn error(&self, token: &Token, message: &str) -> String {
        if token.token_type == TokenType::EndOfFile {
            format!("Syntax Error at end: {}", message)
        } else {
            format!("Syntax Error at line {}, col {} '{}': {}", token.line, token.column, token.lexeme, message)
        }
    }

    fn current_position(&self) -> Position {
        let tk = self.peek();
        Position { line: tk.line, column: tk.column }
    }
}

