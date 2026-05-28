use crate::ir::ir_instructions::{IRInstruction, Operand};
use crate::ir::basic_block::BasicBlock;
use std::collections::HashMap;

/// Callee-saved registers — safe to use without worrying about ABI caller-save rules.
/// We save/restore them in the function prologue/epilogue if used.
pub const ALLOCATABLE: &[&str] = &["rbx", "r12", "r13", "r14", "r15"];

#[derive(Clone, Debug)]
pub struct LiveInterval {
    pub operand: Operand,
    pub start: usize, // linearised instruction index of first def
    pub end: usize,   // linearised instruction index of last use
    pub crosses_call: bool,
}

/// Result of register allocation for one function.
pub struct RegAlloc {
    /// Operand key → physical register name (only for allocated operands).
    pub reg_map: HashMap<String, &'static str>,
    /// Callee-saved registers actually used (need save/restore).
    pub used_regs: Vec<&'static str>,
}

impl RegAlloc {
    pub fn reg_for(&self, op: &Operand) -> Option<&'static str> {
        let key = match op {
            Operand::Var { name, .. } => format!("v_{}", name),
            Operand::Temp { id, version } => format!("t_{}_{}", id, version),
            _ => return None,
        };
        self.reg_map.get(&key).copied()
    }

    /// Returns the set of StackFrame-style keys (`var_X` / `temp_X`) for all
    /// operands that have been assigned a physical register.
    /// Used by the liveness-aware stack allocator to skip already-allocated ops.
    pub fn sf_reg_keys(&self) -> std::collections::HashSet<String> {
        self.reg_map.keys().map(|k| {
            if let Some(name) = k.strip_prefix("v_") {
                format!("var_{}", name)
            } else if let Some(rest) = k.strip_prefix("t_") {
                // rest is "id_version" — StackFrame uses only id
                let id = rest.split('_').next().unwrap_or(rest);
                format!("temp_{}", id)
            } else {
                k.clone()
            }
        }).collect()
    }
}

pub struct RegisterAllocator;

impl RegisterAllocator {
    /// Run Linear Scan allocation on the given blocks (already in RPO).
    pub fn allocate(ordered_blocks: &[(&String, &BasicBlock)]) -> RegAlloc {
        let (instructions, call_indices) = Self::linearize(ordered_blocks);
        let intervals = Self::compute_intervals(&instructions, &call_indices);
        Self::linear_scan(intervals)
    }

    // ── Linearise all instructions in RPO order ─────────────────────────────

    fn linearize<'a>(
        blocks: &[(&String, &'a BasicBlock)],
    ) -> (Vec<&'a IRInstruction>, Vec<usize>) {
        let mut insts = Vec::new();
        let mut call_indices = Vec::new();
        for (_, blk) in blocks {
            for inst in &blk.instructions {
                let idx = insts.len();
                if matches!(inst, IRInstruction::Call { .. }) {
                    call_indices.push(idx);
                }
                insts.push(inst);
            }
        }
        (insts, call_indices)
    }

    // ── Compute live intervals ───────────────────────────────────────────────

    fn compute_intervals(
        insts: &[&IRInstruction],
        call_indices: &[usize],
    ) -> Vec<LiveInterval> {
        // operand key → (interval, seen_def)
        let mut map: HashMap<String, LiveInterval> = HashMap::new();

        for (idx, inst) in insts.iter().enumerate() {
            // Definition: sets start
            if let Some(def) = Self::def(inst) {
                let key = Self::key(&def);
                let iv = map.entry(key).or_insert(LiveInterval {
                    operand: def.clone(),
                    start: idx,
                    end: idx,
                    crosses_call: false,
                });
                iv.start = iv.start.min(idx);
                iv.end = iv.end.max(idx);
            }
            // Uses: extend end
            for used in Self::uses(inst) {
                let key = Self::key(&used);
                let start_idx = if matches!(used, Operand::Var { version: 0, .. }) { 0 } else { idx };
                let iv = map.entry(key.clone()).or_insert(LiveInterval {
                    operand: used.clone(),
                    start: start_idx,
                    end: idx,
                    crosses_call: false,
                });
                iv.end = iv.end.max(idx);
            }
        }

        // Mark intervals that cross a call site
        for iv in map.values_mut() {
            for &ci in call_indices {
                if iv.start <= ci && ci <= iv.end {
                    iv.crosses_call = true;
                    break;
                }
            }
        }

        let mut result: Vec<LiveInterval> = map.into_values().collect();
        result.sort_by_key(|iv| iv.start);
        result
    }

    // ── Linear Scan ─────────────────────────────────────────────────────────

    fn linear_scan(intervals: Vec<LiveInterval>) -> RegAlloc {
        // active: (end, operand_key, reg_idx)
        let mut active: Vec<(usize, String, usize)> = Vec::new();
        let mut free: Vec<usize> = (0..ALLOCATABLE.len()).collect();
        let mut reg_map: HashMap<String, &'static str> = HashMap::new();
        let mut used_regs: Vec<&'static str> = Vec::new();

        for iv in &intervals {
            // Skip Literals, Labels, and intervals that cross calls
            // (call-crossing vars need callee-save semantics more carefully handled)
            match &iv.operand {
                Operand::Literal { .. } | Operand::Label { .. } => continue,
                _ => {}
            }
            if iv.crosses_call {
                continue; // keep on stack for simplicity
            }

            // Expire intervals whose end < current start
            active.retain(|(end, _, ri)| {
                if *end < iv.start {
                    free.push(*ri);
                    false
                } else {
                    true
                }
            });

            let key = Self::key(&iv.operand);

            if free.is_empty() {
                // Spill: choose interval with largest end (not current)
                if let Some(pos) = active.iter().position(|(end, _, _)| {
                    *end == active.iter().map(|(e, _, _)| *e).max().unwrap_or(0)
                }) {
                    if active[pos].0 > iv.end {
                        // Spill that one, give its register to current
                        let (_, spill_key, ri) = active.remove(pos);
                        // Remove spilled operand from reg_map
                        reg_map.remove(&spill_key);
                        let reg = ALLOCATABLE[ri];
                        reg_map.insert(key.clone(), reg);
                        active.push((iv.end, key, ri));
                        if !used_regs.contains(&reg) { used_regs.push(reg); }
                    }
                    // else: current interval is longer — leave it on stack
                }
            } else {
                let ri = free.remove(0);
                let reg = ALLOCATABLE[ri];
                reg_map.insert(key.clone(), reg);
                active.push((iv.end, key, ri));
                if !used_regs.contains(&reg) { used_regs.push(reg); }
            }
        }

        RegAlloc { reg_map, used_regs }
    }

    // ── Helpers ─────────────────────────────────────────────────────────────

    fn key(op: &Operand) -> String {
        match op {
            Operand::Var { name, .. } => format!("v_{name}"),
            Operand::Temp { id, version } => format!("t_{id}_{version}"),
            Operand::Literal { value } => format!("l_{value}"),
            Operand::Label { name } => format!("lbl_{name}"),
        }
    }

    fn def(inst: &IRInstruction) -> Option<Operand> {
        match inst {
            IRInstruction::Add { result, .. } | IRInstruction::Sub { result, .. }
            | IRInstruction::Mul { result, .. } | IRInstruction::Div { result, .. }
            | IRInstruction::Move { result, .. } | IRInstruction::Not { result, .. }
            | IRInstruction::Neg { result, .. } | IRInstruction::Equal { result, .. }
            | IRInstruction::NotEqual { result, .. } | IRInstruction::Less { result, .. }
            | IRInstruction::LessEqual { result, .. } | IRInstruction::Greater { result, .. }
            | IRInstruction::GreaterEqual { result, .. }
            | IRInstruction::Load { result, .. } | IRInstruction::GetElementPtr { result, .. }
            | IRInstruction::Alloca { result, .. }
            | IRInstruction::Phi { result, .. } => Some(result.clone()),
            IRInstruction::Call { result: Some(r), .. } => Some(r.clone()),
            _ => None,
        }
    }

    fn uses(inst: &IRInstruction) -> Vec<Operand> {
        let mut v = Vec::new();
        match inst {
            IRInstruction::Add { left, right, .. } | IRInstruction::Sub { left, right, .. }
            | IRInstruction::Mul { left, right, .. } | IRInstruction::Div { left, right, .. }
            | IRInstruction::Equal { left, right, .. } | IRInstruction::NotEqual { left, right, .. }
            | IRInstruction::Less { left, right, .. } | IRInstruction::LessEqual { left, right, .. }
            | IRInstruction::Greater { left, right, .. } | IRInstruction::GreaterEqual { left, right, .. } => {
                v.push(left.clone()); v.push(right.clone());
            }
            IRInstruction::Move { source, .. } | IRInstruction::Not { operand: source, .. }
            | IRInstruction::Neg { operand: source, .. }
            | IRInstruction::JumpIfTrue { condition: source, .. }
            | IRInstruction::JumpIfFalse { condition: source, .. }
            | IRInstruction::Return { value: Some(source) }
            | IRInstruction::Param { value: source } => { v.push(source.clone()); }
            IRInstruction::Store { address, source } => {
                v.push(address.clone()); v.push(source.clone());
            }
            IRInstruction::Load { address, .. } => { v.push(address.clone()); }
            IRInstruction::GetElementPtr { base, offset, .. } => {
                v.push(base.clone()); v.push(offset.clone());
            }
            IRInstruction::Phi { sources, .. } => {
                for (op, _) in sources { v.push(op.clone()); }
            }
            _ => {}
        }
        v
    }
}
