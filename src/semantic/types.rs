#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Float,
    Bool,
    Void,
    String,
    Function { params: Vec<Type>, ret: Box<Type> },
    Struct(String),
    Unknown,
}

impl Type {
    pub fn from_string(s: &str) -> Self {
        match s {
            "int" => Type::Int,
            "float" => Type::Float,
            "bool" | "boolean" => Type::Bool,
            "void" => Type::Void,
            "string" => Type::String,
            _ => Type::Struct(s.to_string()),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Type::Int => "int".to_string(),
            Type::Float => "float".to_string(),
            Type::Bool => "bool".to_string(),
            Type::Void => "void".to_string(),
            Type::String => "string".to_string(),
            Type::Unknown => "unknown".to_string(),
            Type::Struct(name) => name.clone(),
            Type::Function { params, ret } => {
                let p_str = params.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", ");
                format!("fn({}) -> {}", p_str, ret.to_string())
            }
        }
    }
}
