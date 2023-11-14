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
                        parse_string_to_num!(addend1, error!(OperationError::InvalidParsedNumber))?;
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
            LiteralData::String(minuend) => {
                let minuend =
                    parse_string_to_num!(minuend, error!(OperationError::InvalidParsedNumber))?;
                match rhs {
                    LiteralData::Bool(_) => Err(error!(OperationError::InvalidBooleanOperation)),
                    LiteralData::String(subtrahend) => {
                        let subtrahend = parse_string_to_num!(
                            subtrahend,
                            error!(OperationError::InvalidParsedNumber)
                        )?;
                        Ok(LiteralData::Number(minuend - subtrahend))
                    }
                    LiteralData::Number(subtrahend) => {
                        Ok(LiteralData::Number(minuend - subtrahend))
                    }
                    LiteralData::Null => Err(error!(OperationError::InvalidNullSubtraction)),
                }
            }
            LiteralData::Number(minuend) => match rhs {
                LiteralData::Bool(_) => Err(error!(OperationError::InvalidBooleanOperation)),
                LiteralData::String(subtrahend) => {
                    let subtrahend = parse_string_to_num!(
                        subtrahend,
                        error!(OperationError::InvalidParsedNumber)
                    )?;
                    Ok(LiteralData::Number(minuend - subtrahend))
                }
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
            LiteralData::String(multiplicand) => match rhs {
                LiteralData::Bool(_) => Err(error!(OperationError::InvalidBooleanOperation)),
                LiteralData::String(_) => Err(error!(OperationError::InvalidBooleanOperation)),
                LiteralData::Number(multiplier) => Ok(LiteralData::String(
                    multiplicand.repeat(multiplier as usize),
                )),
                LiteralData::Null => Err(error!(OperationError::InvalidNullMultiplication)),
            },
            LiteralData::Number(multiplicand) => match rhs {
                LiteralData::Bool(_) => Err(error!(OperationError::InvalidBooleanOperation)),
                LiteralData::String(multiplier) => {
                    let multiplier = parse_string_to_num!(
                        multiplier,
                        error!(OperationError::InvalidParsedNumber)
                    )?;
                    Ok(LiteralData::Number(multiplicand * multiplier))
                }
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
            LiteralData::Bool(_) => todo!(),
            LiteralData::String(_) => todo!(),
            LiteralData::Number(_) => todo!(),
            LiteralData::Null => todo!(),
        }
    }
}
