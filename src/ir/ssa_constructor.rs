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
        keys.sort(); 
        
        let mut block_exit_versions: HashMap<String, HashMap<String, usize>> = HashMap::new();

        for key in keys {
            let mut bb = self.blocks.remove(&key).unwrap();
            
            // 1. Insert PHI nodes if there are multiple predecessors with different versions
            if bb.predecessors.len() > 1 {
                let mut vars_to_phi = std::collections::HashSet::new();
                for pred in &bb.predecessors {
                    if let Some(versions) = block_exit_versions.get(pred) {
                        for (var, _) in versions {
                            vars_to_phi.insert(var.clone());
                        }
                    }
                }

                for var in vars_to_phi {
                    let mut sources = Vec::new();
                    let mut different = false;
                    let mut first_v = None;
                    
                    for pred in &bb.predecessors {
                        if let Some(versions) = block_exit_versions.get(pred) {
                            let v = *versions.get(&var).unwrap_or(&0);
                            sources.push((Operand::Var { name: var.clone(), version: v }, pred.clone()));
                            if first_v.is_none() { first_v = Some(v); }
                            else if first_v != Some(v) { different = true; }
                        }
                    }

                    if different {
                        let new_v = self.new_version(&var);
                        let phi_node = IRInstruction::Phi { 
                            result: Operand::Var { name: var.clone(), version: new_v },
                            sources 
                        };
                        bb.instructions.insert(0, phi_node);
                    }
                }
            } else if bb.predecessors.len() == 1 {
                // Inherit versions from the single predecessor
                if let Some(versions) = block_exit_versions.get(&bb.predecessors[0]) {
                    for (var, ver) in versions {
                        self.counters.insert(var.clone(), *ver);
                    }
                }
            }

            // 2. Linear versioning of instructions
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
                    IRInstruction::Mod { result, left, right } => {
                        let l = self.version_operand_read(&left);
                        let r = self.version_operand_read(&right);
                        let res = self.version_operand_write(&result);
                        IRInstruction::Mod { result: res, left: l, right: r }
                    }
                    IRInstruction::And { result, left, right } => {
                        let l = self.version_operand_read(&left);
                        let r = self.version_operand_read(&right);
                        let res = self.version_operand_write(&result);
                        IRInstruction::And { result: res, left: l, right: r }
                    }
                    IRInstruction::Or { result, left, right } => {
                        let l = self.version_operand_read(&left);
                        let r = self.version_operand_read(&right);
                        let res = self.version_operand_write(&result);
                        IRInstruction::Or { result: res, left: l, right: r }
                    }
                    IRInstruction::Xor { result, left, right } => {
                        let l = self.version_operand_read(&left);
                        let r = self.version_operand_read(&right);
                        let res = self.version_operand_write(&result);
                        IRInstruction::Xor { result: res, left: l, right: r }
                    }
                    IRInstruction::Not { result, operand } => {
                        let op = self.version_operand_read(&operand);
                        let res = self.version_operand_write(&result);
                        IRInstruction::Not { result: res, operand: op }
                    }
                    IRInstruction::Neg { result, operand } => {
                        let op = self.version_operand_read(&operand);
                        let res = self.version_operand_write(&result);
                        IRInstruction::Neg { result: res, operand: op }
                    }
                    IRInstruction::Equal { result, left, right } => {
                        let l = self.version_operand_read(&left);
                        let r = self.version_operand_read(&right);
                        let res = self.version_operand_write(&result);
                        IRInstruction::Equal { result: res, left: l, right: r }
                    }
                    IRInstruction::NotEqual { result, left, right } => {
                        let l = self.version_operand_read(&left);
                        let r = self.version_operand_read(&right);
                        let res = self.version_operand_write(&result);
                        IRInstruction::NotEqual { result: res, left: l, right: r }
                    }
                    IRInstruction::Less { result, left, right } => {
                        let l = self.version_operand_read(&left);
                        let r = self.version_operand_read(&right);
                        let res = self.version_operand_write(&result);
                        IRInstruction::Less { result: res, left: l, right: r }
                    }
                    IRInstruction::LessEqual { result, left, right } => {
                        let l = self.version_operand_read(&left);
                        let r = self.version_operand_read(&right);
                        let res = self.version_operand_write(&result);
                        IRInstruction::LessEqual { result: res, left: l, right: r }
                    }
                    IRInstruction::Greater { result, left, right } => {
                        let l = self.version_operand_read(&left);
                        let r = self.version_operand_read(&right);
                        let res = self.version_operand_write(&result);
                        IRInstruction::Greater { result: res, left: l, right: r }
                    }
                    IRInstruction::GreaterEqual { result, left, right } => {
                        let l = self.version_operand_read(&left);
                        let r = self.version_operand_read(&right);
                        let res = self.version_operand_write(&result);
                        IRInstruction::GreaterEqual { result: res, left: l, right: r }
                    }
                    IRInstruction::Load { result, address } => {
                        let addr = self.version_operand_read(&address);
                        let res = self.version_operand_write(&result);
                        IRInstruction::Load { result: res, address: addr }
                    }
                    IRInstruction::Store { address, source } => {
                        let addr = self.version_operand_read(&address);
                        let src = self.version_operand_read(&source);
                        IRInstruction::Store { address: addr, source: src }
                    }
                    IRInstruction::Alloca { result, size } => {
                        let res = self.version_operand_write(&result);
                        IRInstruction::Alloca { result: res, size }
                    }
                    IRInstruction::GetElementPtr { result, base, offset } => {
                        let b = self.version_operand_read(&base);
                        let o = self.version_operand_read(&offset);
                        let res = self.version_operand_write(&result);
                        IRInstruction::GetElementPtr { result: res, base: b, offset: o }
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
                    IRInstruction::Return { value } => {
                        if let Some(v) = value {
                            IRInstruction::Return { value: Some(self.version_operand_read(&v)) }
                        } else {
                            IRInstruction::Return { value: None }
                        }
                    }
                    IRInstruction::JumpIfTrue { condition, label } => {
                        IRInstruction::JumpIfTrue { condition: self.version_operand_read(&condition), label }
                    }
                    IRInstruction::JumpIfFalse { condition, label } => {
                        IRInstruction::JumpIfFalse { condition: self.version_operand_read(&condition), label }
                    }

                    _ => inst,
                };
                new_instructions.push(ssa_inst);
            }

            block_exit_versions.insert(key.clone(), self.counters.clone());
            bb.instructions = new_instructions;
            self.blocks.insert(key, bb);
        }

        // 3. Patch Phi sources for back-edges
        for bb in self.blocks.values_mut() {
            for inst in &mut bb.instructions {
                if let IRInstruction::Phi { ref mut sources, .. } = inst {
                    for (op, pred) in sources.iter_mut() {
                        if let Operand::Var { name, ref mut version } = op {
                            if let Some(versions) = block_exit_versions.get(pred) {
                                if let Some(v) = versions.get(name) {
                                    *version = *v;
                                }
                            }
                        }
                    }
                }
            }
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
