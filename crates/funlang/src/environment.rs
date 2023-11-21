use std::collections::HashMap;

use crate::ast::expr::Expr;

#[derive(Debug, Clone)]
pub struct Environment {
    variables: HashMap<String, Expr>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn set_enclosing(mut self, environment: Environment) -> Self {
        self.enclosing = Some(Box::new(environment));
        self
    }

    pub fn define(&mut self, name: &str, value: Expr) {
        self.variables.insert(name.to_string(), value);
    }

    pub fn mut_variable(&mut self, name: &str) -> Option<&mut Expr> {
        self.variables.get_mut(name).or(match self.enclosing {
            Some(ref mut environment) => environment.mut_variable(name),
            None => None,
        })
    }

    pub fn variable(&self, name: &str) -> Option<&Expr> {
        self.variables.get(name).or(match self.enclosing {
            Some(ref environment) => environment.variable(name),
            None => None,
        })
    }
}
