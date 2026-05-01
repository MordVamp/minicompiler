pub const INTEGER_ARG_REGISTERS: &[&str] = &["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
pub const RETURN_REGISTER: &str = "rax";

pub fn get_arg_register(index: usize) -> Option<&'static str> {
    INTEGER_ARG_REGISTERS.get(index).copied()
}
