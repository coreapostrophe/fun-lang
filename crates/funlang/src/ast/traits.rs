use funlang_error::ErrorCascade;

use crate::{environment::Environment, errors::InterpreterError};

pub trait Evaluable<R> {
    fn evaluate(&self, environment: &Environment) -> Result<R, ErrorCascade<InterpreterError>>;
}

pub trait Executable {
    fn execute(&self, environment: &Environment) -> Result<(), ErrorCascade<InterpreterError>>;
}
