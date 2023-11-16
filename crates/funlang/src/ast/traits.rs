use funlang_error::ErrorCascade;

use crate::errors::ParserError;

pub trait Evaluable<R> {
    fn evaluate(&self) -> Result<R, ErrorCascade<ParserError>>;
}
