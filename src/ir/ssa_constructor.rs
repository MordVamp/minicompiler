use crate::ir::basic_block::BasicBlock;
use crate::ir::ir_instructions::{IRInstruction, Operand};
use std::collections::HashMap;

pub struct SSAConstructor {
    pub blocks: HashMap<String, BasicBlock>,
    counters: HashMap<String, usize>,
}

impl SSAConstructor {
    pub fn new(blocks: HashMap<String, BasicBlock>) -> Self {
        Self {
            blocks,
            counters: HashMap::new(),
        }
    }

    fn new_version(&mut self, name: &str) -> usize {
        let count = self.counters.entry(name.to_string()).or_insert(0);
        *count += 1;
        *count
    }

    fn current_version(&self, name: &str) -> usize {
        *self.counters.get(name).unwrap_or(&0)
    }

    /// Convert the CFG into Static Single Assignment form.
    /// Note: A full implementation computes Dominance Frontiers and inserts PHI nodes.
    /// This implementation performs linear basic block SSA versioning.
    pub fn construct(&mut self) {
        let mut keys: Vec<String> = self.blocks.keys().cloned().collect();
        keys.sort(); // Process in basic deterministic order
        
        // Simple pessimistic versioning pass (simulating SSA across bb without full DF)
        for key in keys {
            let mut bb = self.blocks.remove(&key).unwrap();
            let mut new_instructions = Vec::new();

            for inst in bb.instructions {
                let ssa_inst = match inst {
                    IRInstruction::Move { result, source } => {
                        let src_ssa = self.version_operand_read(&source);
                        let res_ssa = self.version_operand_write(&result);
                        IRInstruction::Move { result: res_ssa, source: src_ssa }
                    }
                    IRInstruction::Add { result, left, right } => {
                        let l = self.version_operand_read(&left);
                        let r = self.version_operand_read(&right);
                        let res = self.version_operand_write(&result);
                        IRInstruction::Add { result: res, left: l, right: r }
                    }
                    IRInstruction::Sub { result, left, right } => {
                        let l = self.version_operand_read(&left);
                        let r = self.version_operand_read(&right);
                        let res = self.version_operand_write(&result);
                        IRInstruction::Sub { result: res, left: l, right: r }
                    }
                    IRInstruction::Mul { result, left, right } => {
                        let l = self.version_operand_read(&left);
                        let r = self.version_operand_read(&right);
                        let res = self.version_operand_write(&result);
                        IRInstruction::Mul { result: res, left: l, right: r }
                    }
                    IRInstruction::Div { result, left, right } => {
                        let l = self.version_operand_read(&left);
                        let r = self.version_operand_read(&right);
                        let res = self.version_operand_write(&result);
                        IRInstruction::Div { result: res, left: l, right: r }
                    }
                    IRInstruction::Param { value } => {
                        IRInstruction::Param { value: self.version_operand_read(&value) }
                    }
                    IRInstruction::Call { result, callee, num_args } => {
                        if let Some(r) = result {
                            IRInstruction::Call { result: Some(self.version_operand_write(&r)), callee, num_args }
                        } else {
                            IRInstruction::Call { result: None, callee, num_args }
                        }
                    }
                    // For brevity, other instructions map identically.
                    _ => inst,
                };
                new_instructions.push(ssa_inst);
            }

            bb.instructions = new_instructions;
            self.blocks.insert(key, bb);
        }
    }

    fn version_operand_read(&self, op: &Operand) -> Operand {
        match op {
            Operand::Var { name, .. } => Operand::Var { name: name.clone(), version: self.current_version(name) },
            Operand::Temp { id, .. } => Operand::Temp { id: *id, version: self.current_version(&format!("t{}", id)) },
            _ => op.clone(),
        }
    }

    fn version_operand_write(&mut self, op: &Operand) -> Operand {
        match op {
            Operand::Var { name, .. } => Operand::Var { name: name.clone(), version: self.new_version(name) },
            Operand::Temp { id, .. } => Operand::Temp { id: *id, version: self.new_version(&format!("t{}", id)) },
            _ => op.clone(),
        }
    }
}
