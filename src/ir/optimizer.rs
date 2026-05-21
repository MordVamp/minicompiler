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
        let mut changed = true;
        while changed {
            changed = false;
            if self.constant_propagation() { changed = true; }
            if self.constant_folding() { changed = true; }
            if self.algebraic_simplification() { changed = true; }
            if self.loop_invariant_code_motion() { changed = true; }
            if self.control_flow_optimization() { changed = true; }
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
