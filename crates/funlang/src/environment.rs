use std::collections::HashMap;

use crate::ast::expr::Expr;

#[derive(Debug)]
pub struct Environment {
    variables: HashMap<String, Expr>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &str, value: Expr) {
        self.variables.insert(name.to_string(), value);
    }

    pub fn mut_variable(&mut self, name: &str) -> Option<&mut Expr> {
        self.variables.get_mut(name)
    }

    pub fn variable(&self, name: &str) -> Option<&Expr> {
        self.variables.get(name)
    }
}
