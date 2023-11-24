use std::{
    cell::RefCell,
    collections::HashMap,
    ops::Deref,
    rc::{Rc, Weak},
};

use funlang_error::ErrorCascade;

use crate::{ast::expr::Expr, error, errors::EnvironmentError};

#[derive(Debug)]
pub struct EnvironmentNode {
    variables: HashMap<String, Expr>,
    parent_scope: Weak<RefCell<EnvironmentNode>>,
}

impl EnvironmentNode {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            parent_scope: Weak::new(),
        }
    }

    pub fn define(&mut self, name: &str, value: Expr) {
        self.variables.insert(name.to_owned(), value);
    }

    pub fn assign(
        &mut self,
        name: &str,
        value: Expr,
    ) -> Result<(), ErrorCascade<EnvironmentError>> {
        let local_variable = self.variables.get_mut(name);
        let parent_scope = self.parent_scope.upgrade();

        match local_variable {
            Some(mutable_variable) => {
                *mutable_variable = value;
                Ok(())
            }
            None => match parent_scope {
                Some(parent_scope) => parent_scope.deref().borrow_mut().assign(name, value),
                None => Err(error!(EnvironmentError::InvalidVariable)),
            },
        }
    }

    pub fn variable(&self, name: &str) -> Option<Expr> {
        let local_variable = self.variables.get(name).cloned();
        let parent_scope = self.parent_scope.upgrade();

        local_variable.or(match parent_scope {
            Some(parent_scope) => parent_scope.deref().borrow().variable(name),
            None => None,
        })
    }
}

#[derive(Debug)]
pub struct Environment(pub Rc<RefCell<EnvironmentNode>>);

impl Environment {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(EnvironmentNode::new())))
    }

    pub fn create_scope(&mut self) -> Self {
        let new_environment = Self::new();
        new_environment.0.borrow_mut().parent_scope = Rc::downgrade(&self.0);
        new_environment
    }

    pub fn define(&mut self, name: &str, value: Expr) {
        self.0.borrow_mut().define(name, value);
    }

    pub fn assign(
        &mut self,
        name: &str,
        value: Expr,
    ) -> Result<(), ErrorCascade<EnvironmentError>> {
        self.0.borrow_mut().assign(name, value)
    }

    pub fn variable(&self, name: &str) -> Option<Expr> {
        self.0.borrow().variable(name)
    }
}

#[cfg(test)]
mod environment_tests {
    use crate::{ast::expr::LiteralExpr, literal::LiteralData};

    use super::*;

    #[test]
    fn fetches_variable() {
        let mut environment = Environment::new();

        environment.define(
            "hello",
            Expr::Literal(Box::new(LiteralExpr {
                literal: LiteralData::String("world".to_owned()),
            })),
        );

        assert_eq!(
            format!("{:?}", environment.variable("hello")),
            format!(
                "{:?}",
                Some(Expr::Literal(Box::new(LiteralExpr {
                    literal: LiteralData::String("world".to_owned()),
                })))
            )
        );
    }

    #[test]
    fn creates_scope() {
        let mut environment = Environment::new();

        environment.define(
            "hello",
            Expr::Literal(Box::new(LiteralExpr {
                literal: LiteralData::String("world".to_owned()),
            })),
        );

        let child_environment = environment.create_scope();

        assert_eq!(
            format!("{:?}", child_environment.variable("hello")),
            format!(
                "{:?}",
                Some(Expr::Literal(Box::new(LiteralExpr {
                    literal: LiteralData::String("world".to_owned()),
                })))
            )
        );
    }

    #[test]
    fn assigns_variable() {
        let mut environment = Environment::new();

        environment.define(
            "hello",
            Expr::Literal(Box::new(LiteralExpr {
                literal: LiteralData::String("world".to_owned()),
            })),
        );

        let mut child_environment = environment.create_scope();

        let assign_result = child_environment.assign(
            "hello",
            Expr::Literal(Box::new(LiteralExpr {
                literal: LiteralData::String("funlang".to_owned()),
            })),
        );
        assert!(assign_result.is_ok());

        assert_eq!(
            format!("{:?}", child_environment.variable("hello")),
            format!(
                "{:?}",
                Some(Expr::Literal(Box::new(LiteralExpr {
                    literal: LiteralData::String("funlang".to_owned()),
                })))
            )
        );
    }
}
