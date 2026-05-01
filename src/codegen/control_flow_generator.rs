use crate::ir::ir_instructions::Operand;
use crate::codegen::label_manager::LabelManager;

pub struct ControlFlowGenerator;

impl ControlFlowGenerator {
    pub fn generate_jump(label: &Operand) -> String {
        if let Operand::Label { name } = label {
            format!("  jmp {}\n", LabelManager::block_label(name))
        } else {
            String::new()
        }
    }

    pub fn generate_jump_if(condition_val: &str, label: &Operand, is_true: bool) -> String {
        let inst = if is_true { "jne" } else { "je" };
        if let Operand::Label { name } = label {
            format!("  cmp {}, 0\n  {} {}\n", condition_val, inst, LabelManager::block_label(name))
        } else {
            String::new()
        }
    }
}
