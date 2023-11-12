use std::ops::Add;

use funlang_error::ErrorCascade;

use crate::{error, errors::OperationError, parse_string_to_num};

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralData {
    String(String),
    Number(f32),
    Bool(bool),
    Null,
}

impl Add for LiteralData {
    type Output = Result<LiteralData, ErrorCascade<OperationError>>;
    fn add(self, rhs: Self) -> Self::Output {
        match self {
            LiteralData::Bool(_) => Err(error!(OperationError::InvalidBooleanAddition)),
            LiteralData::Number(addend1) => match rhs {
                LiteralData::Bool(_) => Err(error!(OperationError::InvalidBooleanAddition)),
                LiteralData::Number(addend2) => Ok(LiteralData::Number(addend1 + addend2)),
                LiteralData::String(addend2) => {
                    let addend2 =
                        parse_string_to_num!(addend2, error!(OperationError::InvalidNumber))?;
                    Ok(LiteralData::Number(addend1 + addend2))
                }
                LiteralData::Null => Ok(LiteralData::Number(addend1)),
            },
            LiteralData::String(addend1) => match rhs {
                LiteralData::Bool(_) => Err(error!(OperationError::InvalidBooleanAddition)),
                LiteralData::Number(addend2) => {
                    let addend1 =
                        parse_string_to_num!(addend1, error!(OperationError::InvalidNumber))?;
                    Ok(LiteralData::String(format!("{}{}", addend1, addend2)))
                }
                LiteralData::String(addend2) => {
                    Ok(LiteralData::String(format!("{}{}", addend1, addend2)))
                }
                LiteralData::Null => Ok(LiteralData::String(addend1)),
            },
            LiteralData::Null => Ok(rhs),
        }
    }
}
