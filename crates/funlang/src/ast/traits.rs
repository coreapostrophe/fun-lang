use funlang_error::ErrorCascade;

use crate::{environment::Environment, errors::InterpreterError};

pub trait Evaluable<R> {
    fn evaluate(&self, environment: &mut Environment) -> Result<R, ErrorCascade<InterpreterError>>;
}

pub trait Executable<R> {
    fn execute(&self, environment: &mut Environment) -> Result<R, ErrorCascade<InterpreterError>>;
}
