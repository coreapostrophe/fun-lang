use std::ops::{Add, Div, Mul, Sub};

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
            LiteralData::Bool(_) => Err(error!(OperationError::InvalidBooleanOperation)),
            LiteralData::Number(addend1) => match rhs {
                LiteralData::Bool(_) => Err(error!(OperationError::InvalidBooleanOperation)),
                LiteralData::Number(addend2) => Ok(LiteralData::Number(addend1 + addend2)),
                LiteralData::String(addend2) => {
                    let addend2 =
                        parse_string_to_num!(addend2, error!(OperationError::InvalidParsedNumber))?;
                    Ok(LiteralData::Number(addend1 + addend2))
                }
                LiteralData::Null => Ok(LiteralData::Number(addend1)),
            },
            LiteralData::String(addend1) => match rhs {
                LiteralData::Bool(_) => Err(error!(OperationError::InvalidBooleanOperation)),
                LiteralData::Number(addend2) => {
                    let addend1 =
                        parse_string_to_num!(addend1, error!(OperationError::InvalidNumber))?;
                    Ok(LiteralData::String(format!("{}{}", addend1, addend2)))
                }
                LiteralData::String(addend2) => {
                    Ok(LiteralData::String(addend1 + &addend2))
                }
                LiteralData::Null => Ok(LiteralData::String(addend1)),
            },
            LiteralData::Null => Ok(rhs),
        }
    }
}

impl Sub for LiteralData {
    type Output = Result<LiteralData, ErrorCascade<OperationError>>;
    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            LiteralData::Bool(_) => Err(error!(OperationError::InvalidBooleanOperation)),
            LiteralData::String(_) => Err(error!(OperationError::InvalidStringOperation)),
            LiteralData::Number(minuend) => match rhs {
                LiteralData::Bool(_) => Err(error!(OperationError::InvalidBooleanOperation)),
                LiteralData::String(_) => Err(error!(OperationError::InvalidStringOperation)),
                LiteralData::Number(subtrahend) => Ok(LiteralData::Number(minuend - subtrahend)),
                LiteralData::Null => Ok(LiteralData::Number(minuend)),
            },
            LiteralData::Null => Err(error!(OperationError::InvalidNullOperation)),
        }
    }
}

impl Mul for LiteralData {
    type Output = Result<LiteralData, ErrorCascade<OperationError>>;
    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            LiteralData::Bool(_) => Err(error!(OperationError::InvalidBooleanOperation)),
            LiteralData::String(multiplicand) => match rhs {
                LiteralData::Bool(_) => Err(error!(OperationError::InvalidStringOperation)),
                LiteralData::String(_) => Err(error!(OperationError::InvalidStringOperation)),
                LiteralData::Number(multiplier) => {
                    Ok(LiteralData::String(multiplicand.repeat(multiplier as usize)))
                },
                LiteralData::Null => todo!(),
            },
            LiteralData::Number(_) => todo!(),
            LiteralData::Null => Err(error!(OperationError::InvalidNullOperation)),
        }
    }
}

impl Div for LiteralData {
    type Output = Result<LiteralData, ErrorCascade<OperationError>>;
    fn div(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
