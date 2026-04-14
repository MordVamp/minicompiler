use std::collections::HashMap;
use crate::semantic::types::Type;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolKind {
    Variable,
    Function,
    Struct,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub typ: Type,
    pub line: usize,
    pub column: usize,
}

pub struct SymbolTable {
    scopes: Vec<HashMap<String, Symbol>>,
    archived_scopes: Vec<(usize, HashMap<String, Symbol>)>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
            archived_scopes: Vec::new(),
        }
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        if self.scopes.len() > 1 {
            let level = self.scopes.len() - 1;
            let scope = self.scopes.pop().unwrap();
            self.archived_scopes.push((level, scope));
        }
    }

    pub fn define(&mut self, name: String, kind: SymbolKind, typ: Type, line: usize, column: usize) -> Result<(), String> {
        let current = self.scopes.last_mut().unwrap();
        if current.contains_key(&name) {
            return Err(format!("Identifier '{}' already defined in this scope.", name));
        }
        current.insert(name.clone(), Symbol {
            name,
            kind,
            typ,
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

    pub fn lookup_local(&self, name: &str) -> Option<&Symbol> {
        self.scopes.last().unwrap().get(name)
    }

    // Export internal state for debugging / visualization (Sprint 3 Req)
    pub fn dump(&self) -> String {
        let mut out = String::new();
        out.push_str("--- Symbol Table Dump ---\n");
        
        // Show archived scopes (local variables from exited blocks/functions)
        for (i, (level, scope)) in self.archived_scopes.iter().enumerate() {
            out.push_str(&format!("Archived Scope {} (Depth {}):\n", i, level));
            for (name, symbol) in scope {
                out.push_str(&format!("  {} : {:?} of type {}\n", name, symbol.kind, symbol.typ.to_string()));
            }
        }

        // Show remaining active scopes (usually just global level 0 at the end)
        for (i, scope) in self.scopes.iter().enumerate() {
            out.push_str(&format!("Active Scope level {}:\n", i));
            for (name, symbol) in scope {
                out.push_str(&format!("  {} : {:?} of type {}\n", name, symbol.kind, symbol.typ.to_string()));
            }
        }
        out.push_str("-------------------------\n");
        out
    }
}
