use std::collections::HashMap;
use crate::ir::ir_instructions::Operand;

pub struct StackFrame {
    pub offsets: HashMap<String, i32>,
    pub next_offset: i32,
}

impl StackFrame {
    pub fn new() -> Self {
        Self {
            offsets: HashMap::new(),
            next_offset: -8,
        }
    }

    pub fn get_offset(&mut self, operand: &Operand) -> i32 {
        let key = match operand {
            Operand::Var { name, .. } => format!("var_{}", name),
            Operand::Temp { id, .. } => format!("temp_{}", id),
            _ => operand.to_string(),
        };
        if let Some(&offset) = self.offsets.get(&key) {
            offset
        } else {
            let offset = self.next_offset;
            self.offsets.insert(key, offset);
            self.next_offset -= 8;
            offset
        }
    }

    pub fn allocate_array(&mut self, operand: &Operand, size: usize) -> i32 {
        let key = match operand {
            Operand::Var { name, .. } => format!("var_{}", name),
            Operand::Temp { id, .. } => format!("temp_{}", id),
            _ => operand.to_string(),
        };
        if let Some(&offset) = self.offsets.get(&key) {
            return offset;
        }
        
        let alloc_size = (size as i32) * 8;
        let base_offset = self.next_offset - alloc_size + 8;
        self.offsets.insert(key, base_offset);
        self.next_offset -= alloc_size;
        base_offset
    }

    pub fn reset(&mut self) {
        self.offsets.clear();
        self.next_offset = -8;
    }

    pub fn aligned_size(&self) -> i32 {
        ((-self.next_offset + 15) / 16) * 16
    }
}
