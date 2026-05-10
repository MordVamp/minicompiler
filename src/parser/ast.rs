use crate::lexer::token::{LiteralValue, TokenType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ASTNode {
    Program(ProgramNode),
    Declaration(DeclarationNode),
    Statement(StatementNode),
    Expression(ExpressionNode),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramNode {
    pub declarations: Vec<DeclarationNode>,
    pub position: Position,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeclarationNode {
    FunctionDecl {
        name: String,
        return_type: String, // e.g. "int", "void"
        parameters: Vec<ParamNode>,
        body: Box<StatementNode>, // BlockStmt
        position: Position,
    },
    StructDecl {
        name: String,
        fields: Vec<StatementNode>, // VarDeclStmt
        position: Position,
    },
    VarDecl {
        var_type: String,
        name: String,
        initializer: Option<ExpressionNode>,
        position: Position,
    },
    ArrayDecl {
        var_type: String,
        name: String,
        size: Option<ExpressionNode>,
        initializer: Option<Vec<ExpressionNode>>,
        position: Position,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamNode {
    pub var_type: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StatementNode {
    Block {
        statements: Vec<StatementNode>,
        position: Position,
    },
    ExprStmt {
        expression: ExpressionNode,
        position: Position,
    },
    IfStmt {
        condition: ExpressionNode,
        then_branch: Box<StatementNode>,
        else_branch: Option<Box<StatementNode>>,
        position: Position,
    },
    WhileStmt {
        condition: ExpressionNode,
        body: Box<StatementNode>,
        position: Position,
    },
    ForStmt {
        init: Option<Box<StatementNode>>, // VarDecl or ExprStmt
        condition: Option<ExpressionNode>,
        update: Option<ExpressionNode>,
        body: Box<StatementNode>,
        position: Position,
    },
    ReturnStmt {
        value: Option<ExpressionNode>,
        position: Position,
    },
    VarDeclStmt {
        decl: DeclarationNode, // Wrap a VarDecl inside statement
        position: Position,
    },
    Empty {
        position: Position,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExpressionNode {
    Literal {
        value: LiteralValue,
        position: Position,
        type_info: Option<String>,
    },
    Identifier {
        name: String,
        position: Position,
        type_info: Option<String>,
    },
    Binary {
        left: Box<ExpressionNode>,
        operator: TokenType,
        right: Box<ExpressionNode>,
        position: Position,
        type_info: Option<String>,
    },
    Unary {
        operator: TokenType,
        operand: Box<ExpressionNode>,
        position: Position,
        type_info: Option<String>,
    },
    Postfix {
        target: Box<ExpressionNode>,
        operator: TokenType,
        position: Position,
        type_info: Option<String>,
    },
    Call {
        callee: String,
        arguments: Vec<ExpressionNode>,
        position: Position,
        type_info: Option<String>,
    },
    Assignment {
        target: Box<ExpressionNode>,
        operator: TokenType,
        value: Box<ExpressionNode>,
        position: Position,
        type_info: Option<String>,
    },
    MemberAccess {
        target: Box<ExpressionNode>,
        member: String,
        position: Position,
        type_info: Option<String>,
    },
    ArrayAccess {
        target: Box<ExpressionNode>,
        index: Box<ExpressionNode>,
        position: Position,
        type_info: Option<String>,
    },
}

// ---------------------------------------------------------
// Visitor Output formatters
// ---------------------------------------------------------

impl ProgramNode {
    /// Pretty string representation
    pub fn to_pretty_string(&self) -> String {
        let mut out = String::from("Program:\n");
        for decl in &self.declarations {
            out.push_str(&decl.to_pretty_string(1));
        }
        out
    }

    /// JSON representation
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".to_string())
    }

    /// GraphViz DOT representation
    pub fn to_dot(&self) -> String {
        let mut out = String::from("digraph AST {\n  node [shape=box];\n");
        let root_id = "prog";
        out.push_str(&format!("  {} [label=\"Program\"];\n", root_id));
        let mut id_counter = 0;
        for decl in &self.declarations {
            decl.to_dot(&mut out, root_id, &mut id_counter);
        }
        out.push_str("}\n");
        out
    }
}

// Trait or impls for specific AST nodes formatting
impl DeclarationNode {
    fn to_pretty_string(&self, indent: usize) -> String {
        let ind = "  ".repeat(indent);
        match self {
            DeclarationNode::FunctionDecl { name, return_type, parameters, body, .. } => {
                let params = parameters.iter().map(|p| format!("{} {}", p.var_type, p.name)).collect::<Vec<_>>().join(", ");
                let mut out = format!("{}FunctionDecl: {} -> {}\n{}  Parameters: [{}]\n{}  Body:\n", 
                    ind, name, return_type, ind, params, ind);
                out.push_str(&body.to_pretty_string(indent + 2));
                out
            }
            DeclarationNode::StructDecl { name, fields, .. } => {
                let mut out = format!("{}StructDecl: {}\n", ind, name);
                for f in fields {
                    out.push_str(&f.to_pretty_string(indent + 1));
                }
                out
            }
            DeclarationNode::VarDecl { var_type, name, initializer, .. } => {
                let mut out = format!("{}VarDecl: {} {}", ind, var_type, name);
                if let Some(init) = initializer {
                    out.push_str(&format!(" = {}", init.to_pretty_string(0)));
                }
                out.push('\n');
                out
            }
            DeclarationNode::ArrayDecl { var_type, name, size, initializer, .. } => {
                let mut out = format!("{}ArrayDecl: {} {}", ind, var_type, name);
                if let Some(sz) = size {
                    out.push_str(&format!("[{}]", sz.to_pretty_string(0)));
                } else {
                    out.push_str("[]");
                }
                if let Some(init) = initializer {
                    let elems = init.iter().map(|e| e.to_pretty_string(0)).collect::<Vec<_>>().join(", ");
                    out.push_str(&format!(" = {{{}}}", elems));
                }
                out.push('\n');
                out
            }
        }
    }

    fn to_dot(&self, out: &mut String, parent_id: &str, id: &mut usize) {
        let my_id = format!("node{}", *id);
        *id += 1;
        out.push_str(&format!("  {} -> {};\n", parent_id, my_id));
        match self {
            DeclarationNode::FunctionDecl { name, return_type, body, .. } => {
                out.push_str(&format!("  {} [label=\"Func: {}->{}\", color=blue];\n", my_id, name, return_type));
                body.to_dot(out, &my_id, id);
            }
            DeclarationNode::StructDecl { name, fields, .. } => {
                out.push_str(&format!("  {} [label=\"Struct: {}\", color=blue];\n", my_id, name));
                for f in fields {
                    f.to_dot(out, &my_id, id);
                }
            }
            DeclarationNode::VarDecl { var_type, name, initializer, .. } => {
                out.push_str(&format!("  {} [label=\"{{VarDecl | {}: {}}}\", color=lightblue, style=filled];\n", my_id, name, var_type));
                if let Some(init) = initializer {
                    init.to_dot(out, &my_id, id);
                }
            }
            DeclarationNode::ArrayDecl { var_type, name, size, initializer, .. } => {
                out.push_str(&format!("  {} [label=\"{{ArrayDecl | {}: {}[]}}\", color=lightblue, style=filled];\n", my_id, name, var_type));
                if let Some(sz) = size {
                    sz.to_dot(out, &my_id, id);
                }
                if let Some(init) = initializer {
                    for e in init {
                        e.to_dot(out, &my_id, id);
                    }
                }
            }
        }
    }
}

impl StatementNode {
    fn to_pretty_string(&self, indent: usize) -> String {
        let ind = "  ".repeat(indent);
        match self {
            StatementNode::Block { statements, .. } => {
                let mut out = format!("{}Block:\n", ind);
                for stmt in statements {
                    out.push_str(&stmt.to_pretty_string(indent + 1));
                }
                out
            }
            StatementNode::ExprStmt { expression, .. } => {
                format!("{}ExprStmt: {}\n", ind, expression.to_pretty_string(0))
            }
            StatementNode::IfStmt { condition, then_branch, else_branch, .. } => {
                let mut out = format!("{}If: {}\n{}  Then:\n", ind, condition.to_pretty_string(0), ind);
                out.push_str(&then_branch.to_pretty_string(indent + 2));
                if let Some(els) = else_branch {
                    out.push_str(&format!("{}  Else:\n", ind));
                    out.push_str(&els.to_pretty_string(indent + 2));
                }
                out
            }
            StatementNode::WhileStmt { condition, body, .. } => {
                let mut out = format!("{}While: {}\n{}  Body:\n", ind, condition.to_pretty_string(0), ind);
                out.push_str(&body.to_pretty_string(indent + 2));
                out
            }
            StatementNode::ForStmt { init, condition, update, body, .. } => {
                let s_init = init.as_ref().map_or("()".to_string(), |s| s.to_pretty_string(0).trim().to_string());
                let s_cond = condition.as_ref().map_or("".to_string(), |e| e.to_pretty_string(0));
                let s_upd = update.as_ref().map_or("".to_string(), |e| e.to_pretty_string(0));
                let mut out = format!("{}For: {}; {}; {}\n{}  Body:\n", ind, s_init, s_cond, s_upd, ind);
                out.push_str(&body.to_pretty_string(indent + 2));
                out
            }
            StatementNode::ReturnStmt { value, .. } => {
                if let Some(v) = value {
                    format!("{}Return: {}\n", ind, v.to_pretty_string(0))
                } else {
                    format!("{}Return\n", ind)
                }
            }
            StatementNode::VarDeclStmt { decl, .. } => {
                decl.to_pretty_string(indent)
            }
            StatementNode::Empty { .. } => format!("{}EmptyStmt\n", ind),
        }
    }

    fn to_dot(&self, out: &mut String, parent_id: &str, id: &mut usize) {
        let my_id = format!("node{}", *id);
        *id += 1;
        out.push_str(&format!("  {} -> {};\n", parent_id, my_id));
        match self {
            StatementNode::Block { statements, .. } => {
                out.push_str(&format!("  {} [label=\"Block\"];\n", my_id));
                for s in statements {
                    s.to_dot(out, &my_id, id);
                }
            }
            StatementNode::ExprStmt { expression, .. } => {
                out.push_str(&format!("  {} [label=\"ExprStmt\"];\n", my_id));
                expression.to_dot(out, &my_id, id);
            }
            StatementNode::IfStmt { condition, then_branch, else_branch, .. } => {
                out.push_str(&format!("  {} [label=\"If\"];\n", my_id));
                condition.to_dot(out, &my_id, id);
                then_branch.to_dot(out, &my_id, id);
                if let Some(els) = else_branch {
                    els.to_dot(out, &my_id, id);
                }
            }
            StatementNode::WhileStmt { condition, body, .. } => {
                out.push_str(&format!("  {} [label=\"While\"];\n", my_id));
                condition.to_dot(out, &my_id, id);
                body.to_dot(out, &my_id, id);
            }
            StatementNode::ForStmt { init, condition, update, body, .. } => {
                out.push_str(&format!("  {} [label=\"For\"];\n", my_id));
                if let Some(i) = init {
                    i.to_dot(out, &my_id, id);
                }
                if let Some(c) = condition {
                    c.to_dot(out, &my_id, id);
                }
                if let Some(u) = update {
                    u.to_dot(out, &my_id, id);
                }
                body.to_dot(out, &my_id, id);
            }
            StatementNode::ReturnStmt { value, .. } => {
                out.push_str(&format!("  {} [label=\"Return\"];\n", my_id));
                if let Some(v) = value {
                    v.to_dot(out, &my_id, id);
                }
            }
            StatementNode::VarDeclStmt { decl, .. } => {
                decl.to_dot(out, parent_id, id); // inline into parent
            }
            StatementNode::Empty { .. } => {
                out.push_str(&format!("  {} [label=\"Empty\"];\n", my_id));
            }
        }
    }
}

impl ExpressionNode {
    fn op_to_str(op: &TokenType) -> &'static str {
        match op {
            TokenType::Plus => "+",
            TokenType::Minus => "-",
            TokenType::Star => "*",
            TokenType::Slash => "/",
            TokenType::Percent => "%",
            TokenType::Equal => "=",
            TokenType::EqualEqual => "==",
            TokenType::NotEqual => "!=",
            TokenType::Less => "<",
            TokenType::LessEqual => "<=",
            TokenType::Greater => ">",
            TokenType::GreaterEqual => ">=",
            TokenType::AndAnd => "&&",
            TokenType::OrOr => "||",
            TokenType::Bang => "!",
            TokenType::PlusEqual => "+=",
            TokenType::MinusEqual => "-=",
            TokenType::StarEqual => "*=",
            TokenType::SlashEqual => "/=",
            TokenType::PlusPlus => "++",
            TokenType::MinusMinus => "--",
            _ => "?",
        }
    }

    fn to_pretty_string(&self, _indent: usize) -> String {
        let t_str = if let Some(t) = self.type_info() { format!(" [type: {}]", t) } else { "".to_string() };
        let base = match self {
            ExpressionNode::Literal { value, .. } => format!("{}", value),
            ExpressionNode::Identifier { name, .. } => name.clone(),
            ExpressionNode::Binary { left, operator, right, .. } => {
                format!("({} {} {})", left.to_pretty_string(0), Self::op_to_str(operator), right.to_pretty_string(0))
            }
            ExpressionNode::Unary { operator, operand, .. } => {
                format!("({}{})", Self::op_to_str(operator), operand.to_pretty_string(0))
            }
            ExpressionNode::Call { callee, arguments, .. } => {
                let args = arguments.iter().map(|a| a.to_pretty_string(0)).collect::<Vec<_>>().join(", ");
                format!("{}({})", callee, args)
            }
            ExpressionNode::Assignment { target, operator, value, .. } => {
                format!("({} {} {})", target.to_pretty_string(0), Self::op_to_str(operator), value.to_pretty_string(0))
            }
            ExpressionNode::Postfix { target, operator, .. } => {
                format!("({}{})", target.to_pretty_string(0), Self::op_to_str(operator))
            }
            ExpressionNode::MemberAccess { target, member, .. } => {
                format!("{}.{}", target.to_pretty_string(0), member)
            }
            ExpressionNode::ArrayAccess { target, index, .. } => {
                format!("{}[{}]", target.to_pretty_string(0), index.to_pretty_string(0))
            }
        };
        format!("{}{}", base, t_str)
    }

    fn to_dot(&self, out: &mut String, parent_id: &str, id: &mut usize) {
        let my_id = format!("node{}", *id);
        *id += 1;
        out.push_str(&format!("  {} -> {};\n", parent_id, my_id));
        match self {
            ExpressionNode::Literal { value, .. } => {
                // escape quotes for DOT
                let val_str = format!("{}", value).replace("\"", "\\\"");
                out.push_str(&format!("  {} [label=\"{}\", color=green];\n", my_id, val_str));
            }
            ExpressionNode::Identifier { name, .. } => {
                out.push_str(&format!("  {} [label=\"{}\", color=purple];\n", my_id, name));
            }
            ExpressionNode::Binary { left, operator, right, .. } => {
                out.push_str(&format!("  {} [label=\"{}\", color=orange];\n", my_id, operator));
                left.to_dot(out, &my_id, id);
                right.to_dot(out, &my_id, id);
            }
            ExpressionNode::Unary { operator, operand, .. } => {
                out.push_str(&format!("  {} [label=\"Unary {}\", color=orange];\n", my_id, operator));
                operand.to_dot(out, &my_id, id);
            }
            ExpressionNode::Call { callee, arguments, .. } => {
                out.push_str(&format!("  {} [label=\"Call {}\"];\n", my_id, callee));
                for a in arguments {
                    a.to_dot(out, &my_id, id);
                }
            }
            ExpressionNode::Assignment { target, operator, value, .. } => {
                out.push_str(&format!("  {} [label=\"Assign: {}\", color=red];\n", my_id, Self::op_to_str(operator)));
                target.to_dot(out, &my_id, id);
                value.to_dot(out, &my_id, id);
            }
            ExpressionNode::Postfix { target, operator, .. } => {
                out.push_str(&format!("  {} [label=\"Postfix {}\", color=orange];\n", my_id, Self::op_to_str(operator)));
                target.to_dot(out, &my_id, id);
            }
            ExpressionNode::MemberAccess { target, member, .. } => {
                out.push_str(&format!("  {} [label=\"MemberAccess: .{}\"];\n", my_id, member));
                target.to_dot(out, &my_id, id);
            }
            ExpressionNode::ArrayAccess { target, index, .. } => {
                out.push_str(&format!("  {} [label=\"ArrayAccess: []\"];\n", my_id));
                target.to_dot(out, &my_id, id);
                index.to_dot(out, &my_id, id);
            }
        }
    }
}

impl ExpressionNode {
    pub fn position(&self) -> &Position {
        match self {
            ExpressionNode::Literal { position, .. } => position,
            ExpressionNode::Identifier { position, .. } => position,
            ExpressionNode::Binary { position, .. } => position,
            ExpressionNode::Unary { position, .. } => position,
            ExpressionNode::Call { position, .. } => position,
            ExpressionNode::Assignment { position, .. } => position,
            ExpressionNode::Postfix { position, .. } => position,
            ExpressionNode::MemberAccess { position, .. } => position,
            ExpressionNode::ArrayAccess { position, .. } => position,
        }
    }

    pub fn type_info(&self) -> Option<&String> {
        match self {
            ExpressionNode::Literal { type_info, .. } => type_info.as_ref(),
            ExpressionNode::Identifier { type_info, .. } => type_info.as_ref(),
            ExpressionNode::Binary { type_info, .. } => type_info.as_ref(),
            ExpressionNode::Unary { type_info, .. } => type_info.as_ref(),
            ExpressionNode::Call { type_info, .. } => type_info.as_ref(),
            ExpressionNode::Assignment { type_info, .. } => type_info.as_ref(),
            ExpressionNode::Postfix { type_info, .. } => type_info.as_ref(),
            ExpressionNode::MemberAccess { type_info, .. } => type_info.as_ref(),
            ExpressionNode::ArrayAccess { type_info, .. } => type_info.as_ref(),
        }
    }

    pub fn set_type_info(&mut self, t: String) {
        match self {
            ExpressionNode::Literal { type_info, .. } => *type_info = Some(t),
            ExpressionNode::Identifier { type_info, .. } => *type_info = Some(t),
            ExpressionNode::Binary { type_info, .. } => *type_info = Some(t),
            ExpressionNode::Unary { type_info, .. } => *type_info = Some(t),
            ExpressionNode::Call { type_info, .. } => *type_info = Some(t),
            ExpressionNode::Assignment { type_info, .. } => *type_info = Some(t),
            ExpressionNode::Postfix { type_info, .. } => *type_info = Some(t),
            ExpressionNode::MemberAccess { type_info, .. } => *type_info = Some(t),
            ExpressionNode::ArrayAccess { type_info, .. } => *type_info = Some(t),
        }
    }
}
