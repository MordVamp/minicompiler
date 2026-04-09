use crate::parser::ast::*;
use crate::lexer::token::TokenType;
use crate::ir::ir_instructions::{IRInstruction, Operand};
use crate::ir::basic_block::BasicBlock;
use std::collections::HashMap;

pub struct IRGenerator {
    pub blocks: HashMap<String, BasicBlock>,
    current_block: String,
    temp_counter: usize,
    label_counter: usize,
}

impl IRGenerator {
    pub fn new() -> Self {
        let mut blocks = HashMap::new();
        blocks.insert("entry".to_string(), BasicBlock::new("entry".to_string()));
        Self {
            blocks,
            current_block: "entry".to_string(),
            temp_counter: 0,
            label_counter: 0,
        }
    }

    fn new_temp(&mut self) -> Operand {
        self.temp_counter += 1;
        Operand::Temp { id: self.temp_counter, version: 0 }
    }

    fn new_label(&mut self, prefix: &str) -> String {
        self.label_counter += 1;
        format!("{}_{}", prefix, self.label_counter)
    }

    fn switch_block(&mut self, label: String) {
        if !self.blocks.contains_key(&label) {
            self.blocks.insert(label.clone(), BasicBlock::new(label.clone()));
        }
        self.current_block = label;
    }

    fn emit(&mut self, inst: IRInstruction) {
        if let Some(bb) = self.blocks.get_mut(&self.current_block) {
            bb.add_instruction(inst);
        }
    }

    pub fn generate(&mut self, program: &ProgramNode) {
        for decl in &program.declarations {
            self.visit_declaration(decl);
        }
    }

    fn visit_declaration(&mut self, decl: &DeclarationNode) {
        match decl {
            DeclarationNode::FunctionDecl { name, body, .. } => {
                let func_label = format!("func_{}", name);
                self.switch_block(func_label);
                self.visit_statement(body);
            }
            DeclarationNode::StructDecl { .. } => {}
            DeclarationNode::VarDecl { name, initializer, .. } => {
                if let Some(init) = initializer {
                    let src = self.visit_expression(init);
                    self.emit(IRInstruction::Move {
                        result: Operand::Var { name: name.clone(), version: 0 },
                        source: src,
                    });
                }
            }
        }
    }

    fn visit_statement(&mut self, stmt: &StatementNode) {
        match stmt {
            StatementNode::Block { statements, .. } => {
                for s in statements {
                    self.visit_statement(s);
                }
            }
            StatementNode::ExprStmt { expression, .. } => {
                self.visit_expression(expression);
            }
            StatementNode::IfStmt { condition, then_branch, else_branch, .. } => {
                let cond_op = self.visit_expression(condition);
                let then_lbl = self.new_label("then");
                let end_lbl = self.new_label("end_if");
                let else_lbl = if else_branch.is_some() { self.new_label("else") } else { end_lbl.clone() };

                self.emit(IRInstruction::JumpIfFalse { condition: cond_op, label: Operand::Label { name: else_lbl.clone() } });
                
                self.switch_block(then_lbl.clone());
                self.visit_statement(then_branch);
                self.emit(IRInstruction::Jump { label: Operand::Label { name: end_lbl.clone() } });

                if let Some(eb) = else_branch {
                    self.switch_block(else_lbl.clone());
                    self.visit_statement(eb);
                    self.emit(IRInstruction::Jump { label: Operand::Label { name: end_lbl.clone() } });
                }
                
                self.switch_block(end_lbl);
            }
            StatementNode::ReturnStmt { value, .. } => {
                if let Some(v) = value {
                    let val_op = self.visit_expression(v);
                    self.emit(IRInstruction::Return { value: Some(val_op) });
                } else {
                    self.emit(IRInstruction::Return { value: None });
                }
            }
            StatementNode::VarDeclStmt { decl, .. } => {
                self.visit_declaration(decl);
            }
            _ => { /* While/For mapping to blocks omitted for brevity but follows similar jump logic */ }
        }
    }

    fn visit_expression(&mut self, expr: &ExpressionNode) -> Operand {
        match expr {
            ExpressionNode::Literal { value, .. } => {
                Operand::Literal { value: value.to_string() }
            }
            ExpressionNode::Identifier { name, .. } => {
                Operand::Var { name: name.clone(), version: 0 }
            }
            ExpressionNode::Binary { left, operator, right, .. } => {
                let l_op = self.visit_expression(left);
                let r_op = self.visit_expression(right);
                let result = self.new_temp();
                
                let inst = match operator {
                    TokenType::Plus => IRInstruction::Add { result: result.clone(), left: l_op, right: r_op },
                    TokenType::Minus => IRInstruction::Sub { result: result.clone(), left: l_op, right: r_op },
                    TokenType::Star => IRInstruction::Mul { result: result.clone(), left: l_op, right: r_op },
                    TokenType::Slash => IRInstruction::Div { result: result.clone(), left: l_op, right: r_op },
                    _ => IRInstruction::Add { result: result.clone(), left: l_op, right: r_op }, // Fallback for simplicity
                };
                self.emit(inst);
                result
            }
            ExpressionNode::Assignment { target, value, .. } => {
                let v_op = self.visit_expression(value);
                self.emit(IRInstruction::Move {
                    result: Operand::Var { name: target.clone(), version: 0 },
                    source: v_op.clone(),
                });
                v_op
            }
            ExpressionNode::Call { callee, arguments, .. } => {
                for arg in arguments {
                    let a_op = self.visit_expression(arg);
                    self.emit(IRInstruction::Param { value: a_op });
                }
                let result = self.new_temp();
                self.emit(IRInstruction::Call { result: Some(result.clone()), callee: callee.clone(), num_args: arguments.len() });
                result
            }
            // Other expressions mapped to temps
            _ => self.new_temp(),
        }
    }
}
