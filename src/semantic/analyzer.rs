use crate::parser::ast::*;
use crate::lexer::token::TokenType;
use crate::semantic::symbol_table::{SymbolTable, SymbolKind};
use crate::semantic::types::Type;
use crate::semantic::errors::SemanticError;

pub struct SemanticAnalyzer {
    pub symbol_table: SymbolTable,
    pub errors: Vec<SemanticError>,
    current_function_return_type: Option<Type>,
    struct_defs: std::collections::HashMap<String, std::collections::HashMap<String, Type>>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        let mut sa = Self {
            symbol_table: SymbolTable::new(),
            errors: Vec::new(),
            current_function_return_type: None,
            struct_defs: std::collections::HashMap::new(),
        };
        
        let _ = sa.symbol_table.define("print_int".to_string(), SymbolKind::Function, Type::Function { params: vec![Type::Int], ret: Box::new(Type::Void) }, 0, 0);
        let _ = sa.symbol_table.define("read_int".to_string(), SymbolKind::Function, Type::Function { params: vec![], ret: Box::new(Type::Int) }, 0, 0);
        let _ = sa.symbol_table.define("malloc".to_string(), SymbolKind::Function, Type::Function { params: vec![Type::Int], ret: Box::new(Type::Int) }, 0, 0);
        let _ = sa.symbol_table.define("free".to_string(), SymbolKind::Function, Type::Function { params: vec![Type::Int], ret: Box::new(Type::Void) }, 0, 0);
        let _ = sa.symbol_table.define("printf".to_string(), SymbolKind::Function, Type::Function { params: vec![], ret: Box::new(Type::Int) }, 0, 0);
        let _ = sa.symbol_table.define("scanf".to_string(), SymbolKind::Function, Type::Function { params: vec![], ret: Box::new(Type::Int) }, 0, 0);
        
        sa
    }

    fn check_unreachable_code(&mut self, body: &StatementNode) {
        if let StatementNode::Block { statements, .. } = body {
            let mut unreachable = false;
            for stmt in statements {
                if unreachable {
                    let pos = stmt.position().clone();
                    self.report_error(&pos, "Unreachable statement after `return`".to_string());
                }
                if let StatementNode::ReturnStmt { .. } = stmt {
                    unreachable = true;
                }
                // Для простоты не обрабатываем вложенные блоки, но при желании можно добавить рекурсию
            }
        }
    }

    fn check_missing_return(&mut self, body: &StatementNode, ret_ty: &Type, position: &Position) {
        if *ret_ty == Type::Void || *ret_ty == Type::Unknown {
            return;
        }
        
        if !self.has_return_statement(body) {
            self.report_error(position, format!("Missing return statement in function returning {}", ret_ty.to_string()));
        }
    }

    fn has_return_statement(&self, stmt: &StatementNode) -> bool {
        match stmt {
            StatementNode::Block { statements, .. } => {
                statements.iter().any(|s| self.has_return_statement(s))
            }
            StatementNode::IfStmt { then_branch, else_branch, .. } => {
                let then_ret = self.has_return_statement(then_branch);
                let else_ret = else_branch.as_ref().map_or(false, |eb| self.has_return_statement(eb));
                then_ret && else_ret
            }
            StatementNode::ReturnStmt { .. } => true,
            _ => false,
        }
    }

    fn report_error(&mut self, pos: &Position, msg: String) {
        self.errors.push(SemanticError::new(msg, pos.clone()));
    }

    pub fn analyze(&mut self, program: &mut ProgramNode) -> bool {
        for decl in &mut program.declarations {
            self.visit_declaration(decl);
        }
        self.errors.is_empty()
    }

    fn visit_declaration(&mut self, decl: &mut DeclarationNode) {
        match decl {
            DeclarationNode::FunctionDecl { name, return_type, parameters, body, position } => {
                let ret_ty = Type::from_string(return_type);
                let mut param_types = Vec::new();
                for p in parameters.iter() {
                    param_types.push(Type::from_string(&p.var_type));
                }

                let func_type = Type::Function { params: param_types, ret: Box::new(ret_ty.clone()) };
                if let Err(e) = self.symbol_table.define(name.clone(), SymbolKind::Function, func_type, position.line, position.column) {
                    self.report_error(position, e);
                }

                self.symbol_table.enter_scope();
                for p in parameters.iter() {
                    let p_ty = Type::from_string(&p.var_type);
                    if let Err(e) = self.symbol_table.define(p.name.clone(), SymbolKind::Variable, p_ty, position.line, position.column) {
                        self.report_error(position, e);
                    }
                }

                self.current_function_return_type = Some(ret_ty.clone());
                
                // Проверка недостижимого кода после return
                self.check_unreachable_code(body);
                
                // Проверка на наличие return
                self.check_missing_return(body, &ret_ty, position);
                
                self.visit_statement(body);
                self.current_function_return_type = None;

                self.symbol_table.exit_scope();
            }
            DeclarationNode::StructDecl { name, fields, position } => {
                if let Err(e) = self.symbol_table.define(name.clone(), SymbolKind::Struct, Type::Struct(name.clone()), position.line, position.column) {
                    self.report_error(position, e);
                }
                
                let mut field_map = std::collections::HashMap::new();
                for f in fields.iter_mut() {
                    if let StatementNode::VarDeclStmt { decl: DeclarationNode::VarDecl { var_type, name: f_name, .. }, .. } = f {
                        field_map.insert(f_name.clone(), Type::from_string(var_type));
                    }
                }
                self.struct_defs.insert(name.clone(), field_map);

                self.symbol_table.enter_scope();
                for f in fields.iter_mut() {
                    self.visit_statement(f);
                }
                self.symbol_table.exit_scope();
            }
            DeclarationNode::VarDecl { var_type, name, initializer, position } => {
                let declared_ty = Type::from_string(var_type);
                
                if let Some(init_expr) = initializer {
                    let init_ty = self.visit_expression(init_expr);
                    if init_ty != declared_ty && init_ty != Type::Unknown {
                        self.report_error(position, format!("Type mismatch: cannot assign {} to {} '{}'", init_ty.to_string(), declared_ty.to_string(), name));
                    }
                }

                if let Err(e) = self.symbol_table.define(name.clone(), SymbolKind::Variable, declared_ty, position.line, position.column) {
                    self.report_error(position, e);
                }
            }
            DeclarationNode::ArrayDecl { var_type, name, size, initializer, position } => {
                let declared_ty = Type::from_string(&format!("{}[]", var_type));
                
                if let Some(sz) = size {
                    let sz_ty = self.visit_expression(sz);
                    if sz_ty != Type::Int && sz_ty != Type::Unknown {
                        self.report_error(position, format!("Array size must be of type int, got {}", sz_ty.to_string()));
                    }
                }

                if let Err(e) = self.symbol_table.define(name.clone(), SymbolKind::Variable, declared_ty, position.line, position.column) {
                    self.report_error(position, e);
                }
            }
        }
    }

    fn visit_statement(&mut self, stmt: &mut StatementNode) {
        match stmt {
            StatementNode::Block { statements, .. } => {
                self.symbol_table.enter_scope();
                for s in statements.iter_mut() {
                    self.visit_statement(s);
                }
                self.symbol_table.exit_scope();
            }
            StatementNode::ExprStmt { expression, .. } => {
                self.visit_expression(expression);
            }
            StatementNode::IfStmt { condition, then_branch, else_branch, position, .. } => {
                let cond_ty = self.visit_expression(condition);
                if cond_ty != Type::Bool && cond_ty != Type::Unknown {
                    self.report_error(position, format!("Condition of 'if' must be bool, got {}", cond_ty.to_string()));
                }
                self.visit_statement(then_branch);
                if let Some(eb) = else_branch {
                    self.visit_statement(eb);
                }
            }
            StatementNode::WhileStmt { condition, body, position, .. } => {
                let cond_ty = self.visit_expression(condition);
                if cond_ty != Type::Bool && cond_ty != Type::Unknown {
                    self.report_error(position, format!("Condition of 'while' must be bool, got {}", cond_ty.to_string()));
                }
                self.visit_statement(body);
            }
            StatementNode::ForStmt { init, condition, update, body, position, .. } => {
                self.symbol_table.enter_scope();
                if let Some(i) = init {
                    self.visit_statement(i);
                }
                if let Some(c) = condition {
                    let cond_ty = self.visit_expression(c);
                    if cond_ty != Type::Bool && cond_ty != Type::Unknown {
                        self.report_error(position, format!("Condition of 'for' must be bool, got {}", cond_ty.to_string()));
                    }
                }
                if let Some(u) = update {
                    self.visit_expression(u);
                }
                self.visit_statement(body);
                self.symbol_table.exit_scope();
            }
            StatementNode::ReturnStmt { value, position, .. } => {
                let ret_ty = if let Some(v) = value {
                    self.visit_expression(v)
                } else {
                    Type::Void
                };
                if let Some(expected) = &self.current_function_return_type {
                    if *expected != ret_ty && ret_ty != Type::Unknown {
                        self.report_error(position, format!("Return type mismatch: expected {}, got {}", expected.to_string(), ret_ty.to_string()));
                    }
                } else {
                    self.report_error(position, "Return statement outside of function".to_string());
                }
            }
            StatementNode::VarDeclStmt { decl, .. } => {
                self.visit_declaration(decl);
            }
            StatementNode::Empty { .. } => {}
        }
    }

    fn visit_expression(&mut self, expr: &mut ExpressionNode) -> Type {
        let ty = match expr {
            ExpressionNode::Literal { value, .. } => {
                match value {
                    crate::lexer::token::LiteralValue::Integer(_) => Type::Int,
                    crate::lexer::token::LiteralValue::Float(_) => Type::Float,
                    crate::lexer::token::LiteralValue::String(_) => Type::String,
                    crate::lexer::token::LiteralValue::Boolean(_) => Type::Bool,
                    crate::lexer::token::LiteralValue::None => Type::Unknown,
                }
            }
            ExpressionNode::Identifier { name, position, .. } => {
                if let Some(symbol) = self.symbol_table.lookup(name) {
                    symbol.typ.clone()
                } else {
                    self.report_error(position, format!("Undefined variable '{}'", name));
                    Type::Unknown
                }
            }
            ExpressionNode::Binary { left, operator, right, position, .. } => {
                let l_ty = self.visit_expression(left);
                let r_ty = self.visit_expression(right);
                if l_ty == Type::Unknown || r_ty == Type::Unknown {
                    Type::Unknown
                } else if l_ty != r_ty {
                    self.report_error(position, format!("Type mismatch in binary operation: {} {} {}", l_ty.to_string(), operator, r_ty.to_string()));
                    Type::Unknown
                } else {
                    match operator {
                        TokenType::Plus | TokenType::Minus | TokenType::Star | TokenType::Slash | TokenType::Percent => l_ty,
                        TokenType::EqualEqual | TokenType::NotEqual | TokenType::Less | TokenType::LessEqual | TokenType::Greater | TokenType::GreaterEqual => Type::Bool,
                        TokenType::AndAnd | TokenType::OrOr => {
                            if l_ty != Type::Bool {
                                self.report_error(position, "Logical operators require boolean operands".to_string());
                            }
                            Type::Bool
                        }
                        _ => Type::Unknown,
                    }
                }
            }
            ExpressionNode::Unary { operator, operand, position, .. } => {
                let op_ty = self.visit_expression(operand);
                match operator {
                    TokenType::Minus => {
                        if op_ty != Type::Int && op_ty != Type::Float && op_ty != Type::Unknown {
                            self.report_error(position, "Unary minus requires numeric operand".to_string());
                        }
                        op_ty
                    }
                    TokenType::Bang => {
                        if op_ty != Type::Bool && op_ty != Type::Unknown {
                            self.report_error(position, "Logical NOT requires bool operand".to_string());
                        }
                        Type::Bool
                    }
                    _ => Type::Unknown,
                }
            }
            ExpressionNode::Postfix { target, position, .. } => {
                let t_ty = self.visit_expression(target);
                if t_ty != Type::Int && t_ty != Type::Float && t_ty != Type::Unknown {
                    self.report_error(position, "Postfix operators require numeric target".to_string());
                }
                t_ty
            }
            ExpressionNode::Assignment { target, value, position, .. } => {
                let v_ty = self.visit_expression(value);
                let target_ty = self.visit_expression(target);

                if target_ty != v_ty && v_ty != Type::Unknown && target_ty != Type::Unknown {
                    self.report_error(position, format!("Type mismatch in assignment: cannot assign {} to {}", v_ty.to_string(), target_ty.to_string()));
                }
                
                target_ty
            }
            ExpressionNode::Call { callee, arguments, position, .. } => {
                let mut arg_types = Vec::new();
                for arg in arguments.iter_mut() {
                    arg_types.push(self.visit_expression(arg));
                }
                
                let symbol_type = if let Some(symbol) = self.symbol_table.lookup(callee) {
                    Some(symbol.typ.clone())
                } else {
                    None
                };

                if let Some(s_ty) = symbol_type {
                    if let Type::Function { params, ret } = s_ty {
                        if callee == "printf" || callee == "scanf" || callee == "free" || callee == "malloc" {
                            // Skip parameter validation for variadic / pointer-accepting functions.
                            // malloc and free accept both int and int[] at the ABI level.
                        } else if params.len() != arg_types.len() {
                            self.report_error(position, format!("Function '{}' requires {} arguments, but {} were provided", callee, params.len(), arg_types.len()));
                        } else {
                            for (i, (p_ty, a_ty)) in params.iter().zip(arg_types.iter()).enumerate() {
                                if p_ty != a_ty && *a_ty != Type::Unknown {
                                    self.report_error(position, format!("Argument {} of '{}' expected {}, got {}", i+1, callee, p_ty.to_string(), a_ty.to_string()));
                                }
                            }
                        }
                        *ret
                    } else {
                        self.report_error(position, format!("'{}' is not a function", callee));
                        Type::Unknown
                    }
                } else {
                    self.report_error(position, format!("Undefined function '{}'", callee));
                    Type::Unknown
                }
            }
            ExpressionNode::MemberAccess { target, member, position, .. } => {
                let target_ty = self.visit_expression(target);
                if let Type::Struct(struct_name) = target_ty {
                    if let Some(field_map) = self.struct_defs.get(&struct_name) {
                        if let Some(field_ty) = field_map.get(member) {
                            field_ty.clone()
                        } else {
                            self.report_error(position, format!("Struct '{}' has no field named '{}'", struct_name, member));
                            Type::Unknown
                        }
                    } else {
                        self.report_error(position, format!("Unknown struct type '{}'", struct_name));
                        Type::Unknown
                    }
                } else if target_ty != Type::Unknown {
                    self.report_error(position, format!("Cannot access member '{}' on non-struct type {}", member, target_ty.to_string()));
                    Type::Unknown
                } else {
                    Type::Unknown
                }
            }
            ExpressionNode::ArrayAccess { target, index, position, .. } => {
                let target_ty = self.visit_expression(target);
                let index_ty = self.visit_expression(index);
                
                if index_ty != Type::Int && index_ty != Type::Unknown {
                    self.report_error(position, format!("Array index must be of type int, got {}", index_ty.to_string()));
                }
                
                let t_str = target_ty.to_string();
                if t_str.ends_with("[]") {
                    Type::from_string(&t_str[..t_str.len()-2])
                } else if target_ty != Type::Unknown {
                    self.report_error(position, format!("Cannot index non-array type {}", t_str));
                    Type::Unknown
                } else {
                    Type::Unknown
                }
            }
        };

        expr.set_type_info(ty.to_string());
        
        // Attempt constant folding after type info is set
        if let ExpressionNode::Binary { left, operator, right, position, .. } = expr {
            if let (ExpressionNode::Literal { value: l_val, .. }, ExpressionNode::Literal { value: r_val, .. }) = (&**left, &**right) {
                if let Some(folded) = self.fold_binary(l_val, *operator, r_val) {
                    *expr = ExpressionNode::Literal {
                        value: folded,
                        position: position.clone(),
                        type_info: Some(ty.to_string()),
                    };
                }
            }
        }

        ty
    }

    fn fold_binary(&self, left: &crate::lexer::token::LiteralValue, op: TokenType, right: &crate::lexer::token::LiteralValue) -> Option<crate::lexer::token::LiteralValue> {
        use crate::lexer::token::LiteralValue;
        match (left, right) {
            (LiteralValue::Integer(l), LiteralValue::Integer(r)) => {
                match op {
                    TokenType::Plus => Some(LiteralValue::Integer(l + r)),
                    TokenType::Minus => Some(LiteralValue::Integer(l - r)),
                    TokenType::Star => Some(LiteralValue::Integer(l * r)),
                    TokenType::Slash if *r != 0 => Some(LiteralValue::Integer(l / r)),
                    TokenType::EqualEqual => Some(LiteralValue::Boolean(l == r)),
                    TokenType::NotEqual => Some(LiteralValue::Boolean(l != r)),
                    TokenType::Less => Some(LiteralValue::Boolean(l < r)),
                    TokenType::LessEqual => Some(LiteralValue::Boolean(l <= r)),
                    TokenType::Greater => Some(LiteralValue::Boolean(l > r)),
                    TokenType::GreaterEqual => Some(LiteralValue::Boolean(l >= r)),
                    _ => None,
                }
            }
            (LiteralValue::Boolean(l), LiteralValue::Boolean(r)) => {
                match op {
                    TokenType::AndAnd => Some(LiteralValue::Boolean(*l && *r)),
                    TokenType::OrOr => Some(LiteralValue::Boolean(*l || *r)),
                    TokenType::EqualEqual => Some(LiteralValue::Boolean(l == r)),
                    TokenType::NotEqual => Some(LiteralValue::Boolean(l != r)),
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

impl StatementNode {
    pub fn position(&self) -> &Position {
        match self {
            StatementNode::Block { position, .. } => position,
            StatementNode::ExprStmt { position, .. } => position,
            StatementNode::IfStmt { position, .. } => position,
            StatementNode::WhileStmt { position, .. } => position,
            StatementNode::ForStmt { position, .. } => position,
            StatementNode::ReturnStmt { position, .. } => position,
            StatementNode::VarDeclStmt { position, .. } => position,
            StatementNode::Empty { position } => position,
        }
    }
}