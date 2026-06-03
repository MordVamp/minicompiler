use crate::ir::ir_instructions::{IRInstruction, Operand};
use crate::ir::basic_block::BasicBlock;
use std::collections::{HashMap, HashSet};

use crate::ir::ir_generator::FunctionMetadata;

pub struct IROptimizer {
    pub blocks: HashMap<String, BasicBlock>,
    pub functions: Vec<FunctionMetadata>,
    pub inline_counter: usize,
}

impl IROptimizer {
    pub fn new(blocks: HashMap<String, BasicBlock>, functions: Vec<FunctionMetadata>) -> Self {
        Self { blocks, functions, inline_counter: 0 }
    }

    pub fn optimize(&mut self) {
        let mut changed = true;
        while changed {
            changed = false;
            if self.constant_propagation() { changed = true; }
            if self.function_inlining() { changed = true; }
            if self.copy_propagation() { changed = true; }
            if self.constant_folding() { changed = true; }
            if self.algebraic_simplification() { changed = true; }
            if self.common_subexpression_elimination() { changed = true; }
            if self.loop_invariant_code_motion() { changed = true; }
            if self.control_flow_optimization() { changed = true; }
            if self.block_merging() { changed = true; }
            if self.local_dead_store_elimination() { changed = true; }
            if self.global_dead_store_elimination() { changed = true; }
        }
        // DCE runs only once at the end as a cleanup pass.
        // It must NOT run iteratively because the x86 backend still reads stack slots
        // even for variables that SCCP replaced with literals in instructions.
        // Iterative DCE would remove the initializing Moves, leaving stack slots uninitialized.
        self.dead_code_elimination();
    }

    fn get_sorted_block_names(&self) -> Vec<String> {
        let mut keys: Vec<String> = self.blocks.keys().cloned().collect();
        keys.sort_by(|a, b| {
            let get_num = |s: &str| -> Option<usize> {
                s.rsplit('_').next().and_then(|num| num.parse().ok())
            };
            let num_a = get_num(a).unwrap_or(0);
            let num_b = get_num(b).unwrap_or(0);
            if num_a != num_b {
                num_a.cmp(&num_b)
            } else {
                a.cmp(b)
            }
        });
        keys
    }

    fn function_inlining(&mut self) -> bool {
        let mut changed = false;
        
        let mut inlineable: HashMap<String, Vec<IRInstruction>> = HashMap::new();
        for func in &self.functions {
            let func_label = format!("func_{}", func.name);
            if let Some(block) = self.blocks.get(&func_label) {
                if block.successors.is_empty() { // Single block
                    inlineable.insert(func.name.clone(), block.instructions.clone());
                }
            }
        }

        let block_names: Vec<String> = self.blocks.keys().cloned().collect();
        for name in block_names {
            let mut block = self.blocks.remove(&name).unwrap();
            let mut new_insts = Vec::new();
            
            for inst in block.instructions {
                if let IRInstruction::Call { result, callee, num_args } = &inst {
                    if let Some(callee_insts) = inlineable.get(callee) {
                        let mut param_ops = Vec::new();
                        let mut params_found = 0;
                        for j in (0..new_insts.len()).rev() {
                            if let IRInstruction::Param { value } = &new_insts[j] {
                                param_ops.push(value.clone());
                                params_found += 1;
                                if params_found == *num_args { break; }
                            }
                        }
                        
                        if params_found == *num_args {
                            param_ops.reverse();
                            let mut keep_insts = Vec::new();
                            let mut skip = *num_args;
                            for j in (0..new_insts.len()).rev() {
                                if skip > 0 && matches!(new_insts[j], IRInstruction::Param { .. }) {
                                    skip -= 1;
                                } else {
                                    keep_insts.push(new_insts[j].clone());
                                }
                            }
                            keep_insts.reverse();
                            new_insts = keep_insts;

                            let param_names = self.functions.iter().find(|f| &f.name == callee).unwrap().parameters.clone();
                            let mut replace_map = HashMap::new();
                            for (p_idx, p_name) in param_names.iter().enumerate() {
                                if p_idx < param_ops.len() {
                                    replace_map.insert(Operand::Var { name: p_name.clone(), version: 0 }, param_ops[p_idx].clone());
                                }
                            }

                            self.inline_counter += 1;
                            for c_inst in callee_insts {
                                let mut cloned = c_inst.clone();
                                
                                let map_op = |op: &mut Operand| {
                                    if let Some(repl) = replace_map.get(op) {
                                        *op = repl.clone();
                                    } else {
                                        match op {
                                            Operand::Var { name, .. } => { *name = format!("inln{}_{}", self.inline_counter, name); }
                                            Operand::Temp { id, .. } => { *id += self.inline_counter * 10000; }
                                            _ => {}
                                        }
                                    }
                                };

                                match &mut cloned {
                                    IRInstruction::Add { result, left, right } | IRInstruction::Sub { result, left, right } |
                                    IRInstruction::Mul { result, left, right } | IRInstruction::Div { result, left, right } |
                                    IRInstruction::Mod { result, left, right } |
                                    IRInstruction::And { result, left, right } | IRInstruction::Or { result, left, right } |
                                    IRInstruction::Xor { result, left, right } |
                                    IRInstruction::Equal { result, left, right } | IRInstruction::NotEqual { result, left, right } |
                                    IRInstruction::Less { result, left, right } | IRInstruction::LessEqual { result, left, right } |
                                    IRInstruction::Greater { result, left, right } | IRInstruction::GreaterEqual { result, left, right } => {
                                        map_op(result); map_op(left); map_op(right);
                                    }
                                    IRInstruction::Move { result, source } | IRInstruction::Not { result, operand: source } |
                                    IRInstruction::Neg { result, operand: source } | IRInstruction::Store { address: result, source } |
                                    IRInstruction::Load { result, address: source } | IRInstruction::GetElementPtr { result, base: source, .. } => {
                                        map_op(result); map_op(source);
                                    }
                                    IRInstruction::Return { value } => {
                                        if let (Some(r), Some(v)) = (result, value) {
                                            map_op(v);
                                            new_insts.push(IRInstruction::Move { result: r.clone(), source: v.clone() });
                                        }
                                        continue; 
                                    }
                                    _ => {}
                                }
                                new_insts.push(cloned);
                            }
                            changed = true;
                            continue;
                        }
                    }
                }
                new_insts.push(inst);
            }
            block.instructions = new_insts;
            self.blocks.insert(name, block);
        }
        changed
    }

    fn constant_propagation(&mut self) -> bool {
        let mut changed = false;
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
                if Self::replace_operands_static(inst, &constants) {
                    changed = true;
                }
            }
        }
        changed
    }

    fn copy_propagation(&mut self) -> bool {
        let mut changed = false;
        let mut copies: HashMap<Operand, Operand> = HashMap::new();

        for block in self.blocks.values() {
            for inst in &block.instructions {
                if let IRInstruction::Move { result, source } = inst {
                    if matches!(source, Operand::Var { .. } | Operand::Temp { .. }) {
                        copies.insert(result.clone(), source.clone());
                    }
                }
            }
        }

        for block in self.blocks.values_mut() {
            for inst in &mut block.instructions {
                if Self::replace_operands_static(inst, &copies) {
                    changed = true;
                }
            }
        }
        changed
    }

    fn common_subexpression_elimination(&mut self) -> bool {
        let mut changed = false;
        let mut expressions: HashMap<String, Operand> = HashMap::new();

        for block in self.blocks.values_mut() {
            for inst in &mut block.instructions {
                let (op_str, res, should_replace, existing) = match inst {
                    IRInstruction::Add { result, left, right } => {
                        let op = format!("ADD {:?} {:?}", left, right);
                        let existing = expressions.get(&op).cloned();
                        (Some(op), Some(result.clone()), existing.is_some(), existing)
                    }
                    IRInstruction::Sub { result, left, right } => {
                        let op = format!("SUB {:?} {:?}", left, right);
                        let existing = expressions.get(&op).cloned();
                        (Some(op), Some(result.clone()), existing.is_some(), existing)
                    }
                    IRInstruction::Mul { result, left, right } => {
                        let op = format!("MUL {:?} {:?}", left, right);
                        let existing = expressions.get(&op).cloned();
                        (Some(op), Some(result.clone()), existing.is_some(), existing)
                    }
                    IRInstruction::Div { result, left, right } => {
                        let op = format!("DIV {:?} {:?}", left, right);
                        let existing = expressions.get(&op).cloned();
                        (Some(op), Some(result.clone()), existing.is_some(), existing)
                    }
                    _ => (None, None, false, None)
                };

                if let (Some(op), Some(res)) = (op_str, res) {
                    if should_replace {
                        *inst = IRInstruction::Move { result: res, source: existing.unwrap() };
                        changed = true;
                    } else {
                        expressions.insert(op, res);
                    }
                }
            }
        }
        changed
    }

    fn constant_folding(&mut self) -> bool {
        let mut changed = false;
        for block in self.blocks.values_mut() {
            for inst in &mut block.instructions {
                let folded = match inst {
                    IRInstruction::Add { left: Operand::Literal { value: l }, right: Operand::Literal { value: r }, .. } => {
                        if let (Ok(lv), Ok(rv)) = (l.parse::<i64>(), r.parse::<i64>()) {
                            Some(Operand::Literal { value: (lv + rv).to_string() })
                        } else { None }
                    }
                    IRInstruction::Sub { left: Operand::Literal { value: l }, right: Operand::Literal { value: r }, .. } => {
                        if let (Ok(lv), Ok(rv)) = (l.parse::<i64>(), r.parse::<i64>()) {
                            Some(Operand::Literal { value: (lv - rv).to_string() })
                        } else { None }
                    }
                    IRInstruction::Mul { left: Operand::Literal { value: l }, right: Operand::Literal { value: r }, .. } => {
                        if let (Ok(lv), Ok(rv)) = (l.parse::<i64>(), r.parse::<i64>()) {
                            Some(Operand::Literal { value: (lv * rv).to_string() })
                        } else { None }
                    }
                    IRInstruction::Div { left: Operand::Literal { value: l }, right: Operand::Literal { value: r }, .. } => {
                        if let (Ok(lv), Ok(rv)) = (l.parse::<i64>(), r.parse::<i64>()) {
                            if rv != 0 { Some(Operand::Literal { value: (lv / rv).to_string() }) } else { None }
                        } else { None }
                    }
                    IRInstruction::Equal { left: Operand::Literal { value: l }, right: Operand::Literal { value: r }, .. } => {
                        Some(Operand::Literal { value: if l == r { "1".to_string() } else { "0".to_string() } })
                    }
                    IRInstruction::NotEqual { left: Operand::Literal { value: l }, right: Operand::Literal { value: r }, .. } => {
                        Some(Operand::Literal { value: if l != r { "1".to_string() } else { "0".to_string() } })
                    }
                    IRInstruction::Less { left: Operand::Literal { value: l }, right: Operand::Literal { value: r }, .. } => {
                        if let (Ok(lv), Ok(rv)) = (l.parse::<i64>(), r.parse::<i64>()) {
                            Some(Operand::Literal { value: if lv < rv { "1".to_string() } else { "0".to_string() } })
                        } else { None }
                    }
                    IRInstruction::LessEqual { left: Operand::Literal { value: l }, right: Operand::Literal { value: r }, .. } => {
                        if let (Ok(lv), Ok(rv)) = (l.parse::<i64>(), r.parse::<i64>()) {
                            Some(Operand::Literal { value: if lv <= rv { "1".to_string() } else { "0".to_string() } })
                        } else { None }
                    }
                    IRInstruction::Greater { left: Operand::Literal { value: l }, right: Operand::Literal { value: r }, .. } => {
                        if let (Ok(lv), Ok(rv)) = (l.parse::<i64>(), r.parse::<i64>()) {
                            Some(Operand::Literal { value: if lv > rv { "1".to_string() } else { "0".to_string() } })
                        } else { None }
                    }
                    IRInstruction::GreaterEqual { left: Operand::Literal { value: l }, right: Operand::Literal { value: r }, .. } => {
                        if let (Ok(lv), Ok(rv)) = (l.parse::<i64>(), r.parse::<i64>()) {
                            Some(Operand::Literal { value: if lv >= rv { "1".to_string() } else { "0".to_string() } })
                        } else { None }
                    }
                    IRInstruction::Phi { result, sources } => {
                        let mut folded_val = None;
                        let mut all_same = true;
                        for (op, _) in sources {
                            if op == result { continue; }
                            if let Operand::Literal { .. } = op {
                                if folded_val.is_none() { folded_val = Some(op.clone()); }
                                else if folded_val != Some(op.clone()) { all_same = false; break; }
                            } else {
                                all_same = false; break;
                            }
                        }
                        if all_same && folded_val.is_some() { folded_val } else { None }
                    }
                    _ => None,
                };

                if let Some(val) = folded {
                    if let Some(res) = Self::get_instruction_result_static(inst) {
                        *inst = IRInstruction::Move { result: res.clone(), source: val };
                        changed = true;
                    }
                }
            }
        }
        changed
    }

    fn algebraic_simplification(&mut self) -> bool {
        let mut changed = false;
        for block in self.blocks.values_mut() {
            for inst in &mut block.instructions {
                let simplified = match inst {
                    IRInstruction::Add { left, right, .. } => {
                        if let Operand::Literal { value } = right {
                            if value == "0" { Some(left.clone()) } else { None }
                        } else if let Operand::Literal { value } = left {
                            if value == "0" { Some(right.clone()) } else { None }
                        } else { None }
                    }
                    IRInstruction::Sub { left, right, .. } => {
                        if let Operand::Literal { value } = right {
                            if value == "0" { Some(left.clone()) } else { None }
                        } else { None }
                    }
                    IRInstruction::Mul { left, right, .. } => {
                        if let Operand::Literal { value } = right {
                            if value == "1" { Some(left.clone()) } 
                            else if value == "0" { Some(Operand::Literal { value: "0".to_string() }) }
                            else { None }
                        } else if let Operand::Literal { value } = left {
                            if value == "1" { Some(right.clone()) }
                            else if value == "0" { Some(Operand::Literal { value: "0".to_string() }) }
                            else { None }
                        } else { None }
                    }
                    IRInstruction::Div { left, right, .. } => {
                        if let Operand::Literal { value } = right {
                            if value == "1" { Some(left.clone()) } else { None }
                        } else { None }
                    }
                    _ => None,
                };

                if let Some(val) = simplified {
                    if let Some(res) = Self::get_instruction_result_static(inst) {
                        *inst = IRInstruction::Move { result: res.clone(), source: val };
                        changed = true;
                    }
                }
            }
        }
        changed
    }

    fn replace_operands_static(inst: &mut IRInstruction, constants: &HashMap<Operand, Operand>) -> bool {
        let mut changed = false;
        let mut replace = |op: &mut Operand| {
            if let Some(val) = constants.get(op) {
                if op != val {
                    *op = val.clone();
                    changed = true;
                }
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
            IRInstruction::Phi { sources, .. } => {
                for (op, _) in sources.iter_mut() {
                    let mut s = op.clone();
                    replace(&mut s);
                    *op = s;
                }
            }
            _ => {}
        }
        changed
    }

    /// Derive the set of successors of a block from its instructions directly,
    /// without needing pre-computed block.successors.
    fn get_instruction_successors(block: &BasicBlock) -> HashSet<String> {
        let mut succs = HashSet::new();
        for inst in &block.instructions {
            match inst {
                IRInstruction::Jump { label: Operand::Label { name } } |
                IRInstruction::JumpIfTrue { label: Operand::Label { name }, .. } |
                IRInstruction::JumpIfFalse { label: Operand::Label { name }, .. } => {
                    succs.insert(name.clone());
                }
                _ => {}
            }
        }
        succs
    }

    /// Loop Invariant Code Motion (LICM):
    /// Detects back-edges in the CFG to identify loop headers.
    /// Moves pure constant `Move` instructions from loop headers into the loop pre-header
    /// (the unique predecessor that is not a back-edge source).
    /// Works by analyzing Jump instructions directly — does NOT rely on pre-computed successors.
    fn loop_invariant_code_motion(&mut self) -> bool {
        let mut changed = false;

        // Build predecessor map from Jump instructions directly
        let mut preds: HashMap<String, Vec<String>> = HashMap::new();
        let sorted = self.get_sorted_block_names();

        // For each block, compute successors from instructions (incl. fall-throughs)
        for (idx, name) in sorted.iter().enumerate() {
            let block = match self.blocks.get(name) {
                Some(b) => b,
                None => continue,
            };
            let mut succs: Vec<String> = Self::get_instruction_successors(block).into_iter().collect();

            // Add fall-through if no terminal instruction
            let has_terminal = block.instructions.last().map(|last| {
                matches!(last,
                    IRInstruction::Jump { .. } | IRInstruction::Return { .. } |
                    IRInstruction::JumpIfTrue { .. } | IRInstruction::JumpIfFalse { .. }
                )
            }).unwrap_or(false);

            if !has_terminal && idx + 1 < sorted.len() {
                succs.push(sorted[idx + 1].clone());
            }

            for succ in succs {
                preds.entry(succ).or_default().push(name.clone());
            }
        }

        // Identify loop headers: blocks with a back-edge predecessor
        let header_candidates: Vec<String> = self.blocks.keys().cloned().collect();
        for header in header_candidates {
            let header_succs = match self.blocks.get(&header) {
                Some(b) => Self::get_instruction_successors(b),
                None => continue,
            };

            let header_preds = match preds.get(&header) {
                Some(p) => p.clone(),
                None => continue,
            };

            // Back-edge preds: preds that the header also jumps back to
            let back_edge_preds: Vec<String> = header_preds.iter()
                .filter(|p| header_succs.contains(*p) || *p == &header)
                .cloned().collect();

            if back_edge_preds.is_empty() { continue; }

            // Pre-header: unique non-back-edge predecessor
            let preheader_preds: Vec<String> = header_preds.iter()
                .filter(|p| !back_edge_preds.contains(p))
                .cloned().collect();

            if preheader_preds.len() != 1 { continue; }
            let preheader = preheader_preds[0].clone();

            // Collect constant Move indices to hoist from header
            let hoist_indices: Vec<usize> = {
                let header_block = match self.blocks.get(&header) {
                    Some(b) => b,
                    None => continue,
                };
                header_block.instructions.iter().enumerate()
                    .filter_map(|(i, inst)| {
                        match inst {
                            IRInstruction::Move { source: Operand::Literal { .. }, .. } => Some(i),
                            _ => None,
                        }
                    })
                    .collect()
            };

            if hoist_indices.is_empty() { continue; }

            // Remove from header (in reverse order to keep indices stable)
            let mut to_hoist = Vec::new();
            {
                let header_block = self.blocks.get_mut(&header).unwrap();
                for &i in hoist_indices.iter().rev() {
                    to_hoist.push(header_block.instructions.remove(i));
                }
            }
            to_hoist.reverse();

            // Insert into preheader before terminal instruction
            {
                let preheader_block = self.blocks.get_mut(&preheader).unwrap();
                let insert_pos = if let Some(last) = preheader_block.instructions.last() {
                    let is_terminal = matches!(last,
                        IRInstruction::Jump { .. } | IRInstruction::Return { .. } |
                        IRInstruction::JumpIfTrue { .. } | IRInstruction::JumpIfFalse { .. }
                    );
                    if is_terminal { preheader_block.instructions.len() - 1 }
                    else { preheader_block.instructions.len() }
                } else {
                    0
                };
                for (offset, inst) in to_hoist.into_iter().enumerate() {
                    preheader_block.instructions.insert(insert_pos + offset, inst);
                }
            }
            changed = true;
        }

        changed
    }

    fn block_merging(&mut self) -> bool {
        let mut preds: HashMap<String, Vec<String>> = HashMap::new();
        for (name, block) in &self.blocks {
            for succ in &block.successors {
                preds.entry(succ.clone()).or_default().push(name.clone());
            }
        }
        let keys: Vec<String> = self.blocks.keys().cloned().collect();
        for name in keys {
            let merge_target = if let Some(block) = self.blocks.get(&name) {
                if block.successors.len() == 1 {
                    let succ_name = &block.successors[0];
                    if let Some(succ_preds) = preds.get(succ_name) {
                        if succ_preds.len() == 1 && succ_preds[0] == name && succ_name != &name && !succ_name.starts_with("func_") {
                            Some(succ_name.clone())
                        } else { None }
                    } else { None }
                } else { None }
            } else { None };

            if let Some(succ_name) = merge_target {
                let mut succ_insts = self.blocks.get(&succ_name).unwrap().instructions.clone();
                let succ_succs = self.blocks.get(&succ_name).unwrap().successors.clone();
                
                let block_mut = self.blocks.get_mut(&name).unwrap();
                if let Some(IRInstruction::Jump { .. }) = block_mut.instructions.last() {
                    block_mut.instructions.pop();
                }
                block_mut.instructions.append(&mut succ_insts);
                block_mut.successors = succ_succs;
                
                self.blocks.remove(&succ_name);
                return true; // Restart optimization loop
            }
        }
        false
    }

    fn local_dead_store_elimination(&mut self) -> bool {
        let mut changed = false;
        for block in self.blocks.values_mut() {
            let mut stores: HashMap<String, usize> = HashMap::new();
            let mut to_remove = HashSet::new();
            
            for (i, inst) in block.instructions.iter().enumerate() {
                match inst {
                    IRInstruction::Store { address: Operand::Var { name, version }, .. } => {
                        let key = format!("v_{}_{}", name, version);
                        if let Some(prev_idx) = stores.get(&key) {
                            to_remove.insert(*prev_idx);
                            changed = true;
                        }
                        stores.insert(key, i);
                    }
                    IRInstruction::Store { address: Operand::Temp { id, version }, .. } => {
                        let key = format!("t_{}_{}", id, version);
                        if let Some(prev_idx) = stores.get(&key) {
                            to_remove.insert(*prev_idx);
                            changed = true;
                        }
                        stores.insert(key, i);
                    }
                    IRInstruction::Load { address: Operand::Var { name, version }, .. } => {
                        let key = format!("v_{}_{}", name, version);
                        stores.remove(&key);
                    }
                    IRInstruction::Load { address: Operand::Temp { id, version }, .. } => {
                        let key = format!("t_{}_{}", id, version);
                        stores.remove(&key);
                    }
                    IRInstruction::Call { .. } => {
                        stores.clear();
                    }
                    _ => {}
                }
            }
            
            if !to_remove.is_empty() {
                let mut new_insts = Vec::new();
                for (i, inst) in block.instructions.drain(..).enumerate() {
                    if !to_remove.contains(&i) {
                        new_insts.push(inst);
                    }
                }
                block.instructions = new_insts;
            }
        }
        changed
    }

    fn global_dead_store_elimination(&mut self) -> bool {
        let mut changed = false;
        
        let mut preds: HashMap<String, Vec<String>> = HashMap::new();
        for (name, block) in &self.blocks {
            for succ in &block.successors {
                preds.entry(succ.clone()).or_default().push(name.clone());
            }
        }
        
        let mut blocks_reading_memory = HashSet::new();
        for (name, block) in &self.blocks {
            for inst in &block.instructions {
                if matches!(inst, IRInstruction::Load { .. } | IRInstruction::Call { .. }) {
                    blocks_reading_memory.insert(name.clone());
                    break;
                }
            }
        }
        
        let mut reachability = blocks_reading_memory.clone();
        let mut worklist: Vec<String> = reachability.iter().cloned().collect();
        while let Some(node) = worklist.pop() {
            if let Some(p) = preds.get(&node) {
                for pred in p {
                    if reachability.insert(pred.clone()) {
                        worklist.push(pred.clone());
                    }
                }
            }
        }
        
        for (name, block) in self.blocks.iter_mut() {
            let mut any_succ_reaches = false;
            for succ in &block.successors {
                if reachability.contains(succ) {
                    any_succ_reaches = true;
                    break;
                }
            }
            
            if !any_succ_reaches && !blocks_reading_memory.contains(name) {
                let old_len = block.instructions.len();
                block.instructions.retain(|inst| !matches!(inst, IRInstruction::Store { .. }));
                if block.instructions.len() != old_len {
                    changed = true;
                }
            } else if !any_succ_reaches && blocks_reading_memory.contains(name) {
                let mut last_read_idx = None;
                for (i, inst) in block.instructions.iter().enumerate() {
                    if matches!(inst, IRInstruction::Load { .. } | IRInstruction::Call { .. }) {
                        last_read_idx = Some(i);
                    }
                }
                
                if let Some(idx) = last_read_idx {
                    let mut to_remove = HashSet::new();
                    for (i, inst) in block.instructions.iter().enumerate() {
                        if i > idx && matches!(inst, IRInstruction::Store { .. }) {
                            to_remove.insert(i);
                        }
                    }
                    if !to_remove.is_empty() {
                        let mut new_insts = Vec::new();
                        for (i, inst) in block.instructions.drain(..).enumerate() {
                            if !to_remove.contains(&i) {
                                new_insts.push(inst);
                            }
                        }
                        block.instructions = new_insts;
                        changed = true;
                    }
                }
            }
        }
        
        changed
    }

    fn control_flow_optimization(&mut self) -> bool {
        let mut changed_global = false;
        let mut changed = true;
        while changed {
            changed = false;
            // Fold constant branches
            for block in self.blocks.values_mut() {
                let mut i = 0;
                while i < block.instructions.len() {
                    let mut remove = false;
                    let mut replace_with_jump = None;
                    match &block.instructions[i] {
                        IRInstruction::JumpIfTrue { condition: Operand::Literal { value }, label } => {
                            if value == "0" { remove = true; changed = true; }
                            else if value == "1" { replace_with_jump = Some(label.clone()); changed = true; }
                        }
                        IRInstruction::JumpIfFalse { condition: Operand::Literal { value }, label } => {
                            if value == "1" { remove = true; changed = true; }
                            else if value == "0" { replace_with_jump = Some(label.clone()); changed = true; }
                        }
                        _ => {}
                    }
                    if remove {
                        block.instructions.remove(i);
                        continue;
                    } else if let Some(lbl) = replace_with_jump {
                        block.instructions[i] = IRInstruction::Jump { label: lbl };
                        block.instructions.truncate(i + 1); // Code after unconditional jump is dead
                    }
                    i += 1;
                }
            }

            // Rebuild successors first (with fall-throughs!)
            let sorted_names = self.get_sorted_block_names();
            for name in &sorted_names {
                if let Some(block) = self.blocks.get_mut(name) {
                    let orig_successors = block.successors.clone();
                    block.successors.clear();
                    
                    // Add explicit jumps and find potential fall-throughs
                    let mut has_unconditional_jump_or_return = false;
                    let mut conditional_targets = Vec::new();
                    
                    for inst in &block.instructions {
                        match inst {
                            IRInstruction::Jump { label: Operand::Label { name } } => {
                                has_unconditional_jump_or_return = true;
                                if !block.successors.contains(name) {
                                    block.successors.push(name.clone());
                                }
                            }
                            IRInstruction::Return { .. } => {
                                has_unconditional_jump_or_return = true;
                            }
                            IRInstruction::JumpIfTrue { label: Operand::Label { name }, .. } |
                            IRInstruction::JumpIfFalse { label: Operand::Label { name }, .. } => {
                                if !block.successors.contains(name) {
                                    block.successors.push(name.clone());
                                }
                                conditional_targets.push(name.clone());
                            }
                            _ => {}
                        }
                    }
                    
                    // If the block is not terminated by an unconditional jump or return,
                    // we must retain any original successors that are not the explicit conditional target(s)
                    // as they represent the valid fall-through path.
                    if !has_unconditional_jump_or_return {
                        for orig_succ in &orig_successors {
                            if !block.successors.contains(orig_succ) {
                                block.successors.push(orig_succ.clone());
                            }
                        }
                    }
                }
            }

            // Recompute reachability starting from all function entries and 'entry'
            let mut reachability = HashSet::new();
            let mut worklist = Vec::new();
            for name in self.blocks.keys() {
                if name.starts_with("func_") || name == "entry" {
                    worklist.push(name.clone());
                    reachability.insert(name.clone());
                }
            }

            while let Some(node) = worklist.pop() {
                if let Some(bb) = self.blocks.get(&node) {
                    for succ in &bb.successors {
                        if reachability.insert(succ.clone()) {
                            worklist.push(succ.clone());
                        }
                    }
                }
            }

            let all_keys: Vec<_> = self.blocks.keys().cloned().collect();
            for k in all_keys {
                if !reachability.contains(&k) {
                    self.blocks.remove(&k);
                    changed = true;
                }
            }

            // Clean up PHI nodes
            for bb in self.blocks.values_mut() {
                for inst in &mut bb.instructions {
                    if let IRInstruction::Phi { sources, result } = inst {
                        sources.retain(|(_, pred)| reachability.contains(pred));
                        if sources.len() == 1 {
                            let src = sources[0].0.clone();
                            *inst = IRInstruction::Move { result: result.clone(), source: src };
                            changed = true;
                        }
                    }
                }
            }

            // Rebuild successors and predecessors
            let mut all_preds: HashMap<String, Vec<String>> = HashMap::new();
            for (name, block) in &self.blocks {
                for succ in &block.successors {
                    all_preds.entry(succ.clone()).or_default().push(name.clone());
                }
            }
            for (name, block) in self.blocks.iter_mut() {
                block.predecessors = all_preds.remove(name).unwrap_or_default();
            }
            if changed { changed_global = true; }
        }
        changed_global
    }

    fn dead_code_elimination(&mut self) -> bool {
        let mut used_operands = HashSet::new();

        for block in self.blocks.values() {
            for inst in &block.instructions {
                Self::collect_used_operands_static(inst, &mut used_operands);
            }
        }

        let mut changed = false;
        for block in self.blocks.values_mut() {
            let old_len = block.instructions.len();
            block.instructions.retain(|inst| {
                if let Some(result) = Self::get_instruction_result_static(inst) {
                    if !used_operands.contains(result) && !Self::has_side_effects_static(inst) {
                        return false;
                    }
                }
                true
            });
            if block.instructions.len() != old_len {
                changed = true;
            }
        }
        changed
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
            IRInstruction::Phi { sources, .. } => {
                for (op, _) in sources {
                    used.insert(op.clone());
                }
            }
            IRInstruction::Store { address, source } => {
                used.insert(address.clone());
                used.insert(source.clone());
            }
            IRInstruction::Load { address, .. } => {
                used.insert(address.clone());
            }
            IRInstruction::GetElementPtr { base, offset, .. } => {
                used.insert(base.clone());
                used.insert(offset.clone());
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
