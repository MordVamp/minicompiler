use crate::ir::ir_instructions::{IRInstruction, Operand};
use crate::ir::basic_block::BasicBlock;
use std::collections::{HashMap, HashSet};

pub struct IROptimizer {
    pub blocks: HashMap<String, BasicBlock>,
}

impl IROptimizer {
    pub fn new(blocks: HashMap<String, BasicBlock>) -> Self {
        Self { blocks }
    }

    pub fn optimize(&mut self) {
        self.constant_propagation();
        self.dead_code_elimination();
    }

    fn constant_propagation(&mut self) {
        let mut constants: HashMap<Operand, Operand> = HashMap::new();

        for block in self.blocks.values() {
            for inst in &block.instructions {
                if let IRInstruction::Move { result, source: Operand::Literal { value } } = inst {
                    constants.insert(result.clone(), Operand::Literal { value: value.clone() });
                }
            }
        }

        for block in self.blocks.values_mut() {
            for inst in &mut block.instructions {
                Self::replace_operands_static(inst, &constants);
            }
        }
    }

    fn replace_operands_static(inst: &mut IRInstruction, constants: &HashMap<Operand, Operand>) {
        let replace = |op: &mut Operand| {
            if let Some(val) = constants.get(op) {
                *op = val.clone();
            }
        };

        match inst {
            IRInstruction::Add { left, right, .. } | IRInstruction::Sub { left, right, .. } |
            IRInstruction::Mul { left, right, .. } | IRInstruction::Div { left, right, .. } |
            IRInstruction::And { left, right, .. } | IRInstruction::Or { left, right, .. } |
            IRInstruction::Equal { left, right, .. } | IRInstruction::NotEqual { left, right, .. } |
            IRInstruction::Less { left, right, .. } | IRInstruction::LessEqual { left, right, .. } |
            IRInstruction::Greater { left, right, .. } | IRInstruction::GreaterEqual { left, right, .. } => {
                let (mut l, mut r) = (left.clone(), right.clone());
                replace(&mut l);
                replace(&mut r);
                *left = l;
                *right = r;
            }
            IRInstruction::Move { source, .. } | IRInstruction::Not { operand: source, .. } |
            IRInstruction::Neg { operand: source, .. } | IRInstruction::JumpIfTrue { condition: source, .. } |
            IRInstruction::JumpIfFalse { condition: source, .. } | IRInstruction::Return { value: Some(source) } |
            IRInstruction::Param { value: source } => {
                let mut s = source.clone();
                replace(&mut s);
                *source = s;
            }
            _ => {}
        }
    }

    fn dead_code_elimination(&mut self) {
        let mut used_operands = HashSet::new();

        for block in self.blocks.values() {
            for inst in &block.instructions {
                Self::collect_used_operands_static(inst, &mut used_operands);
            }
        }

        for block in self.blocks.values_mut() {
            block.instructions.retain(|inst| {
                if let Some(result) = Self::get_instruction_result_static(inst) {
                    if !used_operands.contains(result) && !Self::has_side_effects_static(inst) {
                        return false;
                    }
                }
                true
            });
        }
    }

    fn collect_used_operands_static(inst: &IRInstruction, used: &mut HashSet<Operand>) {
        match inst {
            IRInstruction::Add { left, right, .. } | IRInstruction::Sub { left, right, .. } |
            IRInstruction::Mul { left, right, .. } | IRInstruction::Div { left, right, .. } |
            IRInstruction::And { left, right, .. } | IRInstruction::Or { left, right, .. } |
            IRInstruction::Equal { left, right, .. } | IRInstruction::NotEqual { left, right, .. } |
            IRInstruction::Less { left, right, .. } | IRInstruction::LessEqual { left, right, .. } |
            IRInstruction::Greater { left, right, .. } | IRInstruction::GreaterEqual { left, right, .. } => {
                used.insert(left.clone());
                used.insert(right.clone());
            }
            IRInstruction::Move { source, .. } | IRInstruction::Not { operand: source, .. } |
            IRInstruction::Neg { operand: source, .. } | IRInstruction::JumpIfTrue { condition: source, .. } |
            IRInstruction::JumpIfFalse { condition: source, .. } | IRInstruction::Return { value: Some(source) } |
            IRInstruction::Param { value: source } => {
                used.insert(source.clone());
            }
            _ => {}
        }
    }

    fn get_instruction_result_static<'a>(inst: &'a IRInstruction) -> Option<&'a Operand> {
        match inst {
            IRInstruction::Add { result, .. } | IRInstruction::Sub { result, .. } |
            IRInstruction::Mul { result, .. } | IRInstruction::Div { result, .. } |
            IRInstruction::Move { result, .. } | IRInstruction::Not { result, .. } |
            IRInstruction::Neg { result, .. } | IRInstruction::Equal { result, .. } |
            IRInstruction::NotEqual { result, .. } | IRInstruction::Less { result, .. } |
            IRInstruction::LessEqual { result, .. } | IRInstruction::Greater { result, .. } |
            IRInstruction::GreaterEqual { result, .. } | IRInstruction::Call { result: Some(result), .. } |
            IRInstruction::Alloca { result, .. } | IRInstruction::Load { result, .. } |
            IRInstruction::GetElementPtr { result, .. } => Some(result),
            _ => None,
        }
    }

    fn has_side_effects_static(inst: &IRInstruction) -> bool {
        match inst {
            IRInstruction::Call { .. } | IRInstruction::Store { .. } | IRInstruction::Return { .. } => true,
            _ => false,
        }
    }
}
