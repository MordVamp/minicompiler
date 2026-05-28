use std::collections::{HashMap, HashSet};
use crate::ir::ir_instructions::{IRInstruction, Operand};

pub struct StackFrame {
    pub offsets: HashMap<String, i32>,
    pub next_offset: i32,
}

impl StackFrame {
    pub fn new() -> Self {
        Self { offsets: HashMap::new(), next_offset: -8 }
    }

    pub fn op_key(op: &Operand) -> String {
        match op {
            Operand::Var { name, .. } => format!("var_{}", name),
            Operand::Temp { id, .. } => format!("temp_{}", id),
            _ => op.to_string(),
        }
    }

    /// Returns a previously assigned offset, or allocates a new 8-byte slot.
    pub fn get_offset(&mut self, operand: &Operand) -> i32 {
        let key = Self::op_key(operand);
        if let Some(&off) = self.offsets.get(&key) { return off; }
        let off = self.next_offset;
        self.offsets.insert(key, off);
        self.next_offset -= 8;
        off
    }

    /// Reserves a contiguous `size * 8`-byte block for an array.
    pub fn allocate_array(&mut self, operand: &Operand, size: usize) -> i32 {
        let key = Self::op_key(operand);
        if let Some(&off) = self.offsets.get(&key) { return off; }
        let alloc = (size as i32) * 8;
        let base = self.next_offset - alloc + 8;
        self.offsets.insert(key, base);
        self.next_offset -= alloc;
        base
    }

    pub fn reset(&mut self) {
        self.offsets.clear();
        self.next_offset = -8;
    }

    /// Minimum stack bytes needed, rounded up to 16.
    pub fn aligned_size(&self) -> i32 {
        // Use the deepest *actually assigned* offset, not next_offset.
        // Reused slots don't inflate the frame — only slots that received a fresh
        // allocation pushed next_offset further, but the reused slots are already
        // counted by the deepest assignment.
        let deepest = self.offsets.values().copied().min().unwrap_or(0);
        if deepest >= 0 { return 0; }
        ((-deepest + 15) / 16) * 16
    }

    // ─── Liveness-aware allocation ───────────────────────────────────────────

    /// Single-pass liveness-aware stack slot allocator.
    ///
    /// Algorithm:
    ///   1. Allocate arrays first (fixed contiguous blocks).
    ///   2. Compute [first_use, last_use] instruction indices for every spilled
    ///      scalar operand (Var / Temp not already in a physical register).
    ///   3. Greedy linear-scan: operands whose live ranges don't overlap share
    ///      the same [rbp-N] slot, drastically shrinking the stack frame.
    ///
    /// `instructions` — flat RPO instruction sequence for the function.
    /// `reg_keys`     — op_keys already assigned to physical registers (skip).
    /// `heap_arrays`  — op_keys of arrays that will be malloc'd (skip stack allocation).
    pub fn allocate_with_liveness(
        &mut self,
        instructions: &[IRInstruction],
        reg_keys: &HashSet<String>,
        heap_arrays: &HashSet<String>,
    ) {
        // ── Phase 1: arrays get fixed contiguous regions (only if not on heap) ──
        for inst in instructions {
            if let IRInstruction::Alloca { result, size } = inst {
                let key = Self::op_key(result);
                if !heap_arrays.contains(&key) {
                    self.allocate_array(result, *size);
                }
            }
        }

        // ── Phase 2: compute live intervals for spilled scalars ───────────────
        let mut first_use: HashMap<String, usize> = HashMap::new();
        let mut last_use: HashMap<String, usize> = HashMap::new();

        for (idx, inst) in instructions.iter().enumerate() {
            for op in scalar_operands(inst) {
                let key = Self::op_key(&op);
                // skip arrays, registers, and non Var/Temp operands
                if self.offsets.contains_key(&key) || reg_keys.contains(&key) {
                    continue;
                }
                if !matches!(op, Operand::Var { .. } | Operand::Temp { .. }) {
                    continue;
                }
                first_use.entry(key.clone()).or_insert(idx);
                last_use.insert(key, idx);
            }
        }

        // ── Phase 3: greedy slot reuse ────────────────────────────────────────
        // Sort by interval start.
        let mut intervals: Vec<(String, usize, usize)> = first_use
            .iter()
            .filter_map(|(k, &s)| last_use.get(k).map(|&e| (k.clone(), s, e)))
            .collect();
        intervals.sort_by_key(|(_, s, _)| *s);

        // free_slots: (end_idx, offset) — the slot is reusable from end_idx+1
        let mut free_slots: Vec<(usize, i32)> = Vec::new();

        for (key, start, end) in &intervals {
            // Find a slot freed strictly before this interval starts.
            let reuse = free_slots.iter().position(|(freed, _)| *freed < *start);
            let offset = if let Some(i) = reuse {
                let (_, slot_off) = free_slots.remove(i);
                slot_off
            } else {
                let o = self.next_offset;
                self.next_offset -= 8;
                o
            };
            self.offsets.insert(key.clone(), offset);
            free_slots.push((*end, offset));
        }
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────────

/// Collect all Var/Temp operands from an instruction (scalars only — excludes
/// the *base* of GetElementPtr because that's an array address, not a scalar).
fn scalar_operands(inst: &IRInstruction) -> Vec<Operand> {
    let mut v = Vec::new();
    match inst {
        IRInstruction::Add { result, left, right }
        | IRInstruction::Sub { result, left, right }
        | IRInstruction::Mul { result, left, right }
        | IRInstruction::Div { result, left, right }
        | IRInstruction::Mod { result, left, right }
        | IRInstruction::And { result, left, right }
        | IRInstruction::Or  { result, left, right }
        | IRInstruction::Xor { result, left, right }
        | IRInstruction::Equal        { result, left, right }
        | IRInstruction::NotEqual     { result, left, right }
        | IRInstruction::Less         { result, left, right }
        | IRInstruction::LessEqual    { result, left, right }
        | IRInstruction::Greater      { result, left, right }
        | IRInstruction::GreaterEqual { result, left, right } => {
            v.push(result.clone()); v.push(left.clone()); v.push(right.clone());
        }
        IRInstruction::Move { result, source } => {
            v.push(result.clone()); v.push(source.clone());
        }
        IRInstruction::Not { result, operand } | IRInstruction::Neg { result, operand } => {
            v.push(result.clone()); v.push(operand.clone());
        }
        IRInstruction::Call { result, .. } => {
            if let Some(r) = result { v.push(r.clone()); }
        }
        IRInstruction::Phi { result, sources } => {
            v.push(result.clone());
            for (op, _) in sources { v.push(op.clone()); }
        }
        IRInstruction::Load { result, address } => {
            v.push(result.clone()); v.push(address.clone());
        }
        IRInstruction::Store { address, source } => {
            v.push(address.clone()); v.push(source.clone());
        }
        IRInstruction::GetElementPtr { result, offset, .. } => {
            // NOTE: `base` intentionally excluded — it is an array address,
            // already handled by allocate_array in Phase 1.
            v.push(result.clone()); v.push(offset.clone());
        }
        IRInstruction::JumpIfTrue  { condition, .. }
        | IRInstruction::JumpIfFalse { condition, .. } => {
            v.push(condition.clone());
        }
        IRInstruction::Return { value: Some(op) } | IRInstruction::Param { value: op } => {
            v.push(op.clone());
        }
        IRInstruction::Alloca { .. }
        | IRInstruction::Return { value: None }
        | IRInstruction::Jump { .. } => {}
    }
    v
}