use funlang_error::ErrorCascade;

use crate::errors::InterpreterError;

pub trait Evaluable<R> {
    fn evaluate(&self) -> Result<R, ErrorCascade<InterpreterError>>;
}

pub trait Executable {
    fn execute(&self) -> Result<(), ErrorCascade<InterpreterError>>;
}
