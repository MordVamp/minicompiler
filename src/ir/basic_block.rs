use crate::ir::ir_instructions::IRInstruction;

#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub label: String,
    pub instructions: Vec<IRInstruction>,
    pub predecessors: Vec<String>,
    pub successors: Vec<String>,
}

impl BasicBlock {
    pub fn new(label: String) -> Self {
        Self {
            label,
            instructions: Vec::new(),
            predecessors: Vec::new(),
            successors: Vec::new(),
        }
    }

    pub fn add_instruction(&mut self, inst: IRInstruction) {
        self.instructions.push(inst);
    }

    pub fn to_string(&self) -> String {
        let mut out = format!("{}:\n", self.label);
        for inst in &self.instructions {
            out.push_str(&format!("  {}\n", inst));
        }
        out
    }
}
