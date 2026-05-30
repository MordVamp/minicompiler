use crate::parser::ast::*;
use crate::lexer::token::TokenType;
use crate::ir::ir_instructions::{IRInstruction, Operand};
use crate::ir::basic_block::BasicBlock;
use std::collections::HashMap;

#[derive(Clone)]
pub struct FunctionMetadata {
    pub name: String,
    pub parameters: Vec<String>,
}

pub struct IRGenerator {
    pub blocks: HashMap<String, BasicBlock>,
    pub functions: Vec<FunctionMetadata>,
    current_block: String,
    temp_counter: usize,
    pub label_counter: usize,
    struct_fields: std::collections::HashMap<String, Vec<String>>,
    pub strings: Vec<(String, String)>,
}

impl IRGenerator {
    pub fn new() -> Self {
        let mut blocks = HashMap::new();
        blocks.insert("entry".to_string(), BasicBlock::new("entry".to_string()));
        Self {
            blocks,
            functions: Vec::new(),
            current_block: "entry".to_string(),
            temp_counter: 0,
            label_counter: 0,
            struct_fields: std::collections::HashMap::new(),
            strings: Vec::new(),
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
        let prev = self.current_block.clone();
        if !self.blocks.contains_key(&label) {
            self.blocks.insert(label.clone(), BasicBlock::new(label.clone()));
        }
        self.current_block = label.clone();
        
        let mut has_fallthrough = false;
        if let Some(pbb) = self.blocks.get(&prev) {
            if let Some(last) = pbb.instructions.last() {
                match last {
                    IRInstruction::Jump { .. } | IRInstruction::Return { .. }
                    | IRInstruction::JumpIfTrue { .. } | IRInstruction::JumpIfFalse { .. } => has_fallthrough = false,
                    _ => has_fallthrough = true,
                }
            } else {
                has_fallthrough = true;
            }
        }

        // Link the new block to its predecessor if it's not a function start and there's a valid fall-through
        if !label.starts_with("func_") && prev != "entry" && has_fallthrough {
            if let Some(bb) = self.blocks.get_mut(&label) {
                if !bb.predecessors.contains(&prev) {
                    bb.predecessors.push(prev.clone());
                }
            }
            if let Some(pbb) = self.blocks.get_mut(&prev) {
                if !pbb.successors.contains(&label) {
                    pbb.successors.push(label);
                }
            }
        }
    }

    fn emit(&mut self, inst: IRInstruction) {
        // Track jump targets to build CFG edges
        let target = match &inst {
            IRInstruction::Jump { label: Operand::Label { name } } => Some(name.clone()),
            IRInstruction::JumpIfTrue { label: Operand::Label { name }, .. } => Some(name.clone()),
            IRInstruction::JumpIfFalse { label: Operand::Label { name }, .. } => Some(name.clone()),
            _ => None,
        };

        if let Some(t_lbl) = target {
            let current = self.current_block.clone();
            // Link current -> target
            if let Some(pbb) = self.blocks.get_mut(&current) {
                if !pbb.successors.contains(&t_lbl) {
                    pbb.successors.push(t_lbl.clone());
                }
            }
            // Ensure target block exists and link target -> current (pred)
            if !self.blocks.contains_key(&t_lbl) {
                self.blocks.insert(t_lbl.clone(), BasicBlock::new(t_lbl.clone()));
            }
            if let Some(tbb) = self.blocks.get_mut(&t_lbl) {
                if !tbb.predecessors.contains(&current) {
                    tbb.predecessors.push(current);
                }
            }
        }

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
            DeclarationNode::FunctionDecl { name, body, parameters, .. } => {
                let func_label = format!("func_{}", name);
                let param_names = parameters.iter().map(|p| p.name.clone()).collect();
                self.functions.push(FunctionMetadata { name: name.clone(), parameters: param_names });
                self.switch_block(func_label);
                self.visit_statement(body);
            }
            DeclarationNode::StructDecl { name, fields, .. } => {
                let mut f_names = Vec::new();
                for f in fields {
                    if let StatementNode::VarDeclStmt { decl: DeclarationNode::VarDecl { name: fnm, .. }, .. } = f {
                        f_names.push(fnm.clone());
                    }
                }
                self.struct_fields.insert(name.clone(), f_names);
            }
            DeclarationNode::VarDecl { name, initializer, .. } => {
                if let Some(init) = initializer {
                    let src = self.visit_expression(init);
                    self.emit(IRInstruction::Move {
                        result: Operand::Var { name: name.clone(), version: 0 },
                        source: src,
                    });
                }
            }
            DeclarationNode::ArrayDecl { name, size, initializer, .. } => {
                let mut alloc_size = 0;
                if let Some(sz_expr) = size {
                    if let ExpressionNode::Literal { value: crate::lexer::token::LiteralValue::Integer(v), .. } = sz_expr {
                        alloc_size = *v as usize;
                    }
                } else if let Some(init) = initializer {
                    alloc_size = init.len();
                }
                
                self.emit(IRInstruction::Alloca {
                    result: Operand::Var { name: name.clone(), version: 0 },
                    size: alloc_size,
                });

                if let Some(init) = initializer {
                    for (i, expr) in init.iter().enumerate() {
                        let src = self.visit_expression(expr);
                        let ptr = self.new_temp();
                        self.emit(IRInstruction::GetElementPtr { 
                            result: ptr.clone(), 
                            base: Operand::Var { name: name.clone(), version: 0 }, 
                            offset: Operand::Literal { value: i.to_string() } 
                        });
                        self.emit(IRInstruction::Store { address: ptr, source: src });
                    }
                }
            }
        }
    }

    fn visit_statement(&mut self, stmt: &StatementNode) {
        let pos = stmt.position();
        self.emit(IRInstruction::DebugLoc { line: pos.line, col: pos.column });

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

                // Add then_lbl as the fallthrough successor without emitting an explicit Jump.
                // Insert at position 0 so the RPO DFS (which iterates successors in reverse)
                // visits then_lbl last in DFS, placing it FIRST in the final RPO output —
                // immediately after the conditional block so x86 fallthrough works correctly.
                let cond_block = self.current_block.clone();
                if !self.blocks.contains_key(&then_lbl) {
                    self.blocks.insert(then_lbl.clone(), BasicBlock::new(then_lbl.clone()));
                }
                if let Some(pbb) = self.blocks.get_mut(&cond_block) {
                    if !pbb.successors.contains(&then_lbl) {
                        pbb.successors.insert(0, then_lbl.clone()); // at front so RPO visits it right after cond_block
                    }
                }
                if let Some(tbb) = self.blocks.get_mut(&then_lbl) {
                    if !tbb.predecessors.contains(&cond_block) {
                        tbb.predecessors.push(cond_block);
                    }
                }

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
            StatementNode::WhileStmt { condition, body, .. } => {
                // Loop Peeling: Unroll the very first iteration of the loop!
                let peel_body = self.new_label("while_peel");
                let start_label = self.new_label("while_cond");
                let body_label = self.new_label("while_body");
                let end_label = self.new_label("while_end");

                // Evaluate condition for the peeled iteration
                let cond_op_peel = self.visit_expression(condition);
                self.emit(IRInstruction::JumpIfTrue { condition: cond_op_peel, label: Operand::Label { name: peel_body.clone() } });
                self.emit(IRInstruction::Jump { label: Operand::Label { name: end_label.clone() } });

                // The peeled iteration body
                self.switch_block(peel_body);
                self.visit_statement(body);
                self.emit(IRInstruction::Jump { label: Operand::Label { name: start_label.clone() } });
                
                // Normal loop condition
                self.switch_block(start_label.clone());
                let cond_op = self.visit_expression(condition);
                self.emit(IRInstruction::JumpIfTrue { condition: cond_op, label: Operand::Label { name: body_label.clone() } });
                self.emit(IRInstruction::Jump { label: Operand::Label { name: end_label.clone() } });

                // Normal loop body
                self.switch_block(body_label);
                self.visit_statement(body);
                self.emit(IRInstruction::Jump { label: Operand::Label { name: start_label } });

                self.switch_block(end_label);
            }
            StatementNode::ForStmt { init, condition, update, body, .. } => {
                let cond_label = self.new_label("for_cond");
                let body_label = self.new_label("for_body");
                let update_label = self.new_label("for_update");
                let end_label = self.new_label("for_end");

                if let Some(i) = init {
                    self.visit_statement(i);
                }

                self.emit(IRInstruction::Jump { label: Operand::Label { name: cond_label.clone() } });
                
                self.switch_block(cond_label.clone());
                if let Some(c) = condition {
                    let cond_op = self.visit_expression(c);
                    self.emit(IRInstruction::JumpIfTrue { condition: cond_op, label: Operand::Label { name: body_label.clone() } });
                    self.emit(IRInstruction::Jump { label: Operand::Label { name: end_label.clone() } });
                } else {
                    self.emit(IRInstruction::Jump { label: Operand::Label { name: body_label.clone() } });
                }

                self.switch_block(body_label);
                self.visit_statement(body);
                self.emit(IRInstruction::Jump { label: Operand::Label { name: update_label.clone() } });

                self.switch_block(update_label);
                if let Some(u) = update {
                    self.visit_expression(u);
                }
                self.emit(IRInstruction::Jump { label: Operand::Label { name: cond_label } });

                self.switch_block(end_label);
            }
            StatementNode::Empty { .. } => {}
        }
    }

    fn visit_expression(&mut self, expr: &ExpressionNode) -> Operand {
        match expr {
            ExpressionNode::Literal { value, .. } => {
                if let crate::lexer::token::LiteralValue::String(s) = value {
                    let label = self.new_label("str");
                    // NasM strings can be created exactly like the string contents, but we should make sure we quote them or define them byte by byte
                    self.strings.push((label.clone(), format!("`{}`", s.replace("`", "\\`"))));
                    Operand::Label { name: label }
                } else if let crate::lexer::token::LiteralValue::Boolean(b) = value {
                    Operand::Literal { value: if *b { "1".to_string() } else { "0".to_string() } }
                } else {
                    Operand::Literal { value: value.to_string() }
                }
            }
            ExpressionNode::Identifier { name, .. } => {
                Operand::Var { name: name.clone(), version: 0 }
            }
            ExpressionNode::Binary { left, operator, right, .. } => {
                if *operator == TokenType::AndAnd {
                    let false_lbl = self.new_label("and_false");
                    let end_lbl = self.new_label("and_end");
                    let result = self.new_temp();
                    
                    let l_op = self.visit_expression(left);
                    self.emit(IRInstruction::JumpIfFalse { condition: l_op, label: Operand::Label { name: false_lbl.clone() } });
                    
                    let r_op = self.visit_expression(right);
                    self.emit(IRInstruction::Move { result: result.clone(), source: r_op });
                    self.emit(IRInstruction::Jump { label: Operand::Label { name: end_lbl.clone() } });
                    
                    self.switch_block(false_lbl);
                    self.emit(IRInstruction::Move { result: result.clone(), source: Operand::Literal { value: "0".to_string() } });
                    self.emit(IRInstruction::Jump { label: Operand::Label { name: end_lbl.clone() } });
                    
                    self.switch_block(end_lbl);
                    return result;
                } else if *operator == TokenType::OrOr {
                    let true_lbl = self.new_label("or_true");
                    let end_lbl = self.new_label("or_end");
                    let result = self.new_temp();
                    
                    let l_op = self.visit_expression(left);
                    self.emit(IRInstruction::JumpIfTrue { condition: l_op, label: Operand::Label { name: true_lbl.clone() } });
                    
                    let r_op = self.visit_expression(right);
                    self.emit(IRInstruction::Move { result: result.clone(), source: r_op });
                    self.emit(IRInstruction::Jump { label: Operand::Label { name: end_lbl.clone() } });
                    
                    self.switch_block(true_lbl);
                    self.emit(IRInstruction::Move { result: result.clone(), source: Operand::Literal { value: "1".to_string() } });
                    self.emit(IRInstruction::Jump { label: Operand::Label { name: end_lbl.clone() } });
                    
                    self.switch_block(end_lbl);
                    return result;
                }

                let l_op = self.visit_expression(left);
                let r_op = self.visit_expression(right);
                let result = self.new_temp();
                
                let inst = match operator {
                    TokenType::Plus => IRInstruction::Add { result: result.clone(), left: l_op, right: r_op },
                    TokenType::Minus => IRInstruction::Sub { result: result.clone(), left: l_op, right: r_op },
                    TokenType::Star => IRInstruction::Mul { result: result.clone(), left: l_op, right: r_op },
                    TokenType::Slash => IRInstruction::Div { result: result.clone(), left: l_op, right: r_op },
                    TokenType::Percent => IRInstruction::Mod { result: result.clone(), left: l_op, right: r_op },
                    TokenType::EqualEqual => IRInstruction::Equal { result: result.clone(), left: l_op, right: r_op },
                    TokenType::NotEqual => IRInstruction::NotEqual { result: result.clone(), left: l_op, right: r_op },
                    TokenType::Less => IRInstruction::Less { result: result.clone(), left: l_op, right: r_op },
                    TokenType::LessEqual => IRInstruction::LessEqual { result: result.clone(), left: l_op, right: r_op },
                    TokenType::Greater => IRInstruction::Greater { result: result.clone(), left: l_op, right: r_op },
                    TokenType::GreaterEqual => IRInstruction::GreaterEqual { result: result.clone(), left: l_op, right: r_op },
                    _ => IRInstruction::Move { result: result.clone(), source: l_op },
                };
                self.emit(inst);
                result
            }
            ExpressionNode::Unary { operator, operand, .. } => {
                let op = self.visit_expression(operand);
                let result = self.new_temp();
                let inst = match operator {
                    TokenType::Minus => IRInstruction::Neg { result: result.clone(), operand: op },
                    TokenType::Bang => IRInstruction::Not { result: result.clone(), operand: op },
                    _ => IRInstruction::Move { result: result.clone(), source: op },
                };
                self.emit(inst);
                result
            }
            ExpressionNode::Assignment { target, value, .. } => {
                let v_op = self.visit_expression(value);
                
                if let ExpressionNode::Identifier { name, .. } = &**target {
                    self.emit(IRInstruction::Move {
                        result: Operand::Var { name: name.clone(), version: 0 },
                        source: v_op.clone(),
                    });
                } else if let ExpressionNode::ArrayAccess { target: arr, index, .. } = &**target {
                    let base = self.visit_expression(arr);
                    let offset = self.visit_expression(index);
                    let ptr = self.new_temp();
                    self.emit(IRInstruction::GetElementPtr { 
                        result: ptr.clone(), 
                        base, 
                        offset 
                    });
                    self.emit(IRInstruction::Store { address: ptr, source: v_op.clone() });
                } else if let ExpressionNode::MemberAccess { target: obj, member, .. } = &**target {
                    let base = self.visit_expression(obj);
                    let struct_name = obj.type_info().cloned().unwrap_or_default();
                    let offset_val = if let Some(fields) = self.struct_fields.get(&struct_name) {
                        fields.iter().position(|f| f == member).unwrap_or(0)
                    } else {
                        0
                    };
                    let ptr = self.new_temp();
                    self.emit(IRInstruction::GetElementPtr { 
                        result: ptr.clone(), 
                        base, 
                        offset: Operand::Literal { value: offset_val.to_string() } 
                    });
                    self.emit(IRInstruction::Store { address: ptr, source: v_op.clone() });
                }
                
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
            ExpressionNode::MemberAccess { target, member, .. } => {
                let base = self.visit_expression(target);
                let struct_name = target.type_info().cloned().unwrap_or_default();
                let offset = if let Some(fields) = self.struct_fields.get(&struct_name) {
                    fields.iter().position(|f| f == member).unwrap_or(0)
                } else {
                    0
                };
                
                let ptr = self.new_temp();
                self.emit(IRInstruction::GetElementPtr { 
                    result: ptr.clone(), 
                    base, 
                    offset: Operand::Literal { value: offset.to_string() } 
                });
                
                let result = self.new_temp();
                self.emit(IRInstruction::Load { result: result.clone(), address: ptr });
                result
            }
            ExpressionNode::ArrayAccess { target, index, .. } => {
                let base = self.visit_expression(target);
                let offset = self.visit_expression(index);
                let ptr = self.new_temp();
                self.emit(IRInstruction::GetElementPtr { 
                    result: ptr.clone(), 
                    base, 
                    offset 
                });
                
                let result = self.new_temp();
                self.emit(IRInstruction::Load { result: result.clone(), address: ptr });
                result
            }
            // Other expressions mapped to temps
            _ => self.new_temp(),
        }
    }
}
