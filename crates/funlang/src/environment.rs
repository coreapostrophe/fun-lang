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

    pub fn define(&mut self, name: String, value: Expr) {
        self.variables.insert(name, value);
    }

    pub fn get_variable(&self, name: &str) -> Option<&Expr> {
        self.variables.get(name)
    }
}
