use crate::ir::ir_instructions::Operand;

pub struct ExpressionGenerator;

impl ExpressionGenerator {
    pub fn load_operand(reg: &str, op: &Operand, offset_provider: &mut dyn FnMut(&Operand) -> i32) -> String {
        match op {
            Operand::Literal { value } => format!("  mov {}, {}\n", reg, value),
            Operand::Label { name } => format!("  mov {}, {}\n", reg, name),
            _ => {
                let offset = offset_provider(op);
                format!("  mov {}, [rbp{}]\n", reg, if offset >= 0 { format!("+{}", offset) } else { offset.to_string() })
            }
        }
    }

    pub fn store_operand(op: &Operand, reg: &str, offset_provider: &mut dyn FnMut(&Operand) -> i32) -> String {
        let offset = offset_provider(op);
        format!("  mov [rbp{}], {}\n", if offset >= 0 { format!("+{}", offset) } else { offset.to_string() }, reg)
    }

    pub fn generate_comparison(set_inst: &str, result: &Operand, left: &Operand, right: &Operand, offset_provider: &mut dyn FnMut(&Operand) -> i32) -> String {
        let mut output = Self::load_operand("rax", left, offset_provider);
        match right {
            Operand::Literal { value } => {
                output.push_str(&format!("  cmp rax, {}\n", value));
            }
            _ => {
                let offset = offset_provider(right);
                output.push_str(&format!("  cmp rax, [rbp{}]\n", if offset >= 0 { format!("+{}", offset) } else { offset.to_string() }));
            }
        }
        output.push_str(&format!("  {} al\n", set_inst));
        output.push_str("  movzx rax, al\n");
        output.push_str(&Self::store_operand(result, "rax", offset_provider));
        output
    }
}
