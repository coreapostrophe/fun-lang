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
                LiteralData::String(_) => Err(error!(OperationError::MismatchedType(
                    "number".to_string(),
                    "string".to_string()
                ))),
                LiteralData::Null => Ok(LiteralData::Number(addend1)),
            },
            LiteralData::String(addend1) => match rhs {
                LiteralData::Bool(_) => Err(error!(OperationError::InvalidBooleanOperation)),
                LiteralData::Number(addend2) => {
                    Ok(LiteralData::String(format!("{}{}", addend1, addend2)))
                }
                LiteralData::String(addend2) => Ok(LiteralData::String(addend1 + &addend2)),
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
            LiteralData::String(_) => Err(error!(OperationError::InvalidStringSubtraction)),
            LiteralData::Number(minuend) => match rhs {
                LiteralData::Bool(_) => Err(error!(OperationError::InvalidBooleanOperation)),
                LiteralData::String(_) => Err(error!(OperationError::InvalidStringSubtraction)),
                LiteralData::Number(subtrahend) => Ok(LiteralData::Number(minuend - subtrahend)),
                LiteralData::Null => Ok(LiteralData::Number(minuend)),
            },
            LiteralData::Null => Err(error!(OperationError::InvalidNullSubtraction)),
        }
    }
}

impl Mul for LiteralData {
    type Output = Result<LiteralData, ErrorCascade<OperationError>>;
    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            LiteralData::Bool(_) => Err(error!(OperationError::InvalidBooleanOperation)),
            LiteralData::String(_) => Err(error!(OperationError::InvalidStringMultiplication)),
            LiteralData::Number(multiplicand) => match rhs {
                LiteralData::Bool(_) => Err(error!(OperationError::InvalidBooleanOperation)),
                LiteralData::String(_) => Err(error!(OperationError::InvalidStringMultiplication)),
                LiteralData::Number(multiplier) => {
                    Ok(LiteralData::Number(multiplicand * multiplier))
                }
                LiteralData::Null => Err(error!(OperationError::InvalidNullMultiplication)),
            },
            LiteralData::Null => Err(error!(OperationError::InvalidNullMultiplication)),
        }
    }
}

impl Div for LiteralData {
    type Output = Result<LiteralData, ErrorCascade<OperationError>>;
    fn div(self, rhs: Self) -> Self::Output {
        match self {
            LiteralData::Bool(_) => Err(error!(OperationError::InvalidBooleanOperation)),
            LiteralData::String(_) => Err(error!(OperationError::InvalidStringDivision)),
            LiteralData::Number(dividend) => match rhs {
                LiteralData::Bool(_) => Err(error!(OperationError::InvalidBooleanOperation)),
                LiteralData::String(_) => Err(error!(OperationError::InvalidStringDivision)),
                LiteralData::Number(divisor) => Ok(LiteralData::Number(dividend / divisor)),
                LiteralData::Null => Err(error!(OperationError::InvalidNullDivision)),
            },
            LiteralData::Null => Err(error!(OperationError::InvalidNullDivision)),
        }
    }
}
