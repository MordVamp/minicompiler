use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Operand {
    Temp { id: usize, version: usize },
    Var { name: String, version: usize },
    Literal { value: String },
    Label { name: String },
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operand::Temp { id, version } => write!(f, "t{}_{}", id, version),
            Operand::Var { name, version } => write!(f, "{}_{}", name, version),
            Operand::Literal { value } => write!(f, "{}", value),
            Operand::Label { name } => write!(f, "{}", name),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IRInstruction {
    Add { result: Operand, left: Operand, right: Operand },
    Sub { result: Operand, left: Operand, right: Operand },
    Mul { result: Operand, left: Operand, right: Operand },
    Div { result: Operand, left: Operand, right: Operand },
    Mod { result: Operand, left: Operand, right: Operand },
    
    // Logical/Bitwise
    And { result: Operand, left: Operand, right: Operand },
    Or { result: Operand, left: Operand, right: Operand },
    Xor { result: Operand, left: Operand, right: Operand },
    Not { result: Operand, operand: Operand },
    
    // Comparison
    Equal { result: Operand, left: Operand, right: Operand },
    NotEqual { result: Operand, left: Operand, right: Operand },
    Less { result: Operand, left: Operand, right: Operand },
    LessEqual { result: Operand, left: Operand, right: Operand },
    Greater { result: Operand, left: Operand, right: Operand },
    GreaterEqual { result: Operand, left: Operand, right: Operand },
    
    Neg { result: Operand, operand: Operand },

    // Memory Operations
    Load { result: Operand, address: Operand },
    Store { address: Operand, source: Operand },
    Alloca { result: Operand, size: usize },
    GetElementPtr { result: Operand, base: Operand, offset: Operand }, // For structs/arrays

    Move { result: Operand, source: Operand },
    
    Jump { label: Operand },
    JumpIfTrue { condition: Operand, label: Operand },
    JumpIfFalse { condition: Operand, label: Operand },
    
    Call { result: Option<Operand>, callee: String, num_args: usize },
    Param { value: Operand },
    Return { value: Option<Operand> },
    
    Phi { result: Operand, sources: Vec<(Operand, String)> }, // (Operand, BlockLabel)
}

impl fmt::Display for IRInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IRInstruction::Add { result, left, right } => write!(f, "{} = ADD {}, {}", result, left, right),
            IRInstruction::Sub { result, left, right } => write!(f, "{} = SUB {}, {}", result, left, right),
            IRInstruction::Mul { result, left, right } => write!(f, "{} = MUL {}, {}", result, left, right),
            IRInstruction::Div { result, left, right } => write!(f, "{} = DIV {}, {}", result, left, right),
            IRInstruction::Mod { result, left, right } => write!(f, "{} = MOD {}, {}", result, left, right),
            
            IRInstruction::And { result, left, right } => write!(f, "{} = AND {}, {}", result, left, right),
            IRInstruction::Or { result, left, right } => write!(f, "{} = OR {}, {}", result, left, right),
            IRInstruction::Xor { result, left, right } => write!(f, "{} = XOR {}, {}", result, left, right),
            IRInstruction::Not { result, operand } => write!(f, "{} = NOT {}", result, operand),
            
            IRInstruction::Equal { result, left, right } => write!(f, "{} = EQ {}, {}", result, left, right),
            IRInstruction::NotEqual { result, left, right } => write!(f, "{} = NEQ {}, {}", result, left, right),
            IRInstruction::Less { result, left, right } => write!(f, "{} = LESS {}, {}", result, left, right),
            IRInstruction::LessEqual { result, left, right } => write!(f, "{} = LEQ {}, {}", result, left, right),
            IRInstruction::Greater { result, left, right } => write!(f, "{} = GREATER {}, {}", result, left, right),
            IRInstruction::GreaterEqual { result, left, right } => write!(f, "{} = GEQ {}, {}", result, left, right),
            
            IRInstruction::Neg { result, operand } => write!(f, "{} = NEG {}", result, operand),
            
            IRInstruction::Load { result, address } => write!(f, "{} = LOAD [{}]", result, address),
            IRInstruction::Store { address, source } => write!(f, "STORE [{}], {}", address, source),
            IRInstruction::Alloca { result, size } => write!(f, "{} = ALLOCA {}", result, size),
            IRInstruction::GetElementPtr { result, base, offset } => write!(f, "{} = GEP {}, {}", result, base, offset),

            IRInstruction::Move { result, source } => write!(f, "{} = MOVE {}", result, source),
            
            IRInstruction::Jump { label } => write!(f, "JUMP {}", label),
            IRInstruction::JumpIfTrue { condition, label } => write!(f, "JUMP_IF {} {}", condition, label),
            IRInstruction::JumpIfFalse { condition, label } => write!(f, "JUMP_IF_NOT {} {}", condition, label),
            
            IRInstruction::Call { result, callee, num_args } => {
                if let Some(r) = result {
                    write!(f, "{} = CALL {}, {}", r, callee, num_args)
                } else {
                    write!(f, "CALL {}, {}", callee, num_args)
                }
            }
            IRInstruction::Param { value } => write!(f, "PARAM {}", value),
            IRInstruction::Return { value } => {
                if let Some(v) = value {
                    write!(f, "RETURN {}", v)
                } else {
                    write!(f, "RETURN")
                }
            }
            IRInstruction::Phi { result, sources } => {
                let s_str = sources.iter().map(|(op, blk)| format!("{}[{}]", op, blk)).collect::<Vec<_>>().join(", ");
                write!(f, "{} = PHI {}", result, s_str)
            }
        }
    }
}
