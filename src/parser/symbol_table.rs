use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Variable,
    Function,
    Struct,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub data_type: String,
    pub line: usize,
    pub column: usize,
}

pub struct SymbolTable {
    scopes: Vec<HashMap<String, Symbol>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    pub fn define(&mut self, name: String, kind: SymbolKind, data_type: String, line: usize, column: usize) -> Result<(), String> {
        let current = self.scopes.last_mut().unwrap();
        if current.contains_key(&name) {
            return Err(format!("Identifier '{}' already defined in this scope at line {}, col {}.", name, line, column));
        }
        current.insert(name.clone(), Symbol {
            name,
            kind,
            data_type,
            line,
            column,
        });
        Ok(())
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(symbol) = scope.get(name) {
                return Some(symbol);
            }
        }
        None
    }

    pub fn exists_in_current_scope(&self, name: &str) -> bool {
        self.scopes.last().unwrap().contains_key(name)
    }
}
