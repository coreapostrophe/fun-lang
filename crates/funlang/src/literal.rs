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
            LiteralData::Bool(addend1) => match rhs {
                LiteralData::Bool(addend2) => {
                    let addend1 = if addend1 { 1.0 } else { 0.0 };
                    let addend2 = if addend2 { 1.0 } else { 0.0 };
                    Ok(LiteralData::Number(addend1 + addend2))
                }
                LiteralData::String(addend2) => {
                    Ok(LiteralData::String(format!("{}{}", addend1, addend2)))
                }
                LiteralData::Number(addend2) => {
                    let addend1 = if addend1 { 1.0 } else { 0.0 };
                    Ok(LiteralData::Number(addend1 + addend2))
                }
                LiteralData::Null => {
                    let addend1 = if addend1 { 1.0 } else { 0.0 };
                    Ok(LiteralData::Number(addend1))
                }
            },
            LiteralData::Number(addend1) => match rhs {
                LiteralData::Bool(addend2) => {
                    let addend2 = if addend2 { 1.0 } else { 0.0 };
                    Ok(LiteralData::Number(addend1 + addend2))
                }
                LiteralData::Number(addend2) => Ok(LiteralData::Number(addend1 + addend2)),
                LiteralData::String(addend2) => {
                    Ok(LiteralData::String(format!("{}{}", addend1, addend2)))
                }
                LiteralData::Null => Ok(LiteralData::Number(addend1)),
            },
            LiteralData::String(addend1) => match rhs {
                LiteralData::Bool(addend2) => {
                    Ok(LiteralData::String(format!("{}{}", addend1, addend2)))
                }
                LiteralData::Number(addend2) => {
                    Ok(LiteralData::String(format!("{}{}", addend1, addend2)))
                }
                LiteralData::String(addend2) => Ok(LiteralData::String(addend1 + &addend2)),
                LiteralData::Null => Ok(LiteralData::String(format!("{}null", addend1))),
            },
            LiteralData::Null => Ok(rhs),
        }
    }
}

impl Sub for LiteralData {
    type Output = Result<LiteralData, ErrorCascade<OperationError>>;
    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            LiteralData::Bool(minuend) => match rhs {
                LiteralData::Bool(subtrahend) => {
                    let minuend = if minuend { 1.0 } else { 0.0 };
                    let subtrahend = if subtrahend { 1.0 } else { 0.0 };
                    Ok(LiteralData::Number(minuend - subtrahend))
                }
                LiteralData::Number(subtrahend) => {
                    let minuend = if minuend { 1.0 } else { 0.0 };
                    Ok(LiteralData::Number(minuend - subtrahend))
                }
                LiteralData::String(_) => Err(error!(OperationError::MismatchedType(
                    "bool".to_string(),
                    "string".to_string()
                ))),
                LiteralData::Null => {
                    let minuend = if minuend { 1.0 } else { 0.0 };
                    Ok(LiteralData::Number(minuend - 0.0))
                }
            },
            LiteralData::String(minuend) => match rhs {
                LiteralData::Bool(subtrahend) => {
                    let minuend = parse_string_to_num!(
                        minuend,
                        error!(OperationError::InvalidParsedNumber(minuend))
                    )?;
                    let subtrahend = if subtrahend { 1.0 } else { 0.0 };
                    Ok(LiteralData::Number(minuend - subtrahend))
                }
                LiteralData::Number(subtrahend) => {
                    let minuend = parse_string_to_num!(
                        minuend,
                        error!(OperationError::InvalidParsedNumber(minuend))
                    )?;
                    Ok(LiteralData::Number(minuend - subtrahend))
                }
                LiteralData::String(subtrahend) => {
                    let minuend = parse_string_to_num!(
                        minuend,
                        error!(OperationError::InvalidParsedNumber(minuend))
                    )?;
                    let subtrahend = parse_string_to_num!(
                        subtrahend,
                        error!(OperationError::InvalidParsedNumber(subtrahend))
                    )?;
                    Ok(LiteralData::Number(minuend - subtrahend))
                }
                LiteralData::Null => {
                    let minuend = parse_string_to_num!(
                        minuend,
                        error!(OperationError::InvalidParsedNumber(minuend))
                    )?;
                    Ok(LiteralData::Number(minuend))
                }
            },
            LiteralData::Number(minuend) => match rhs {
                LiteralData::Bool(subtrahend) => {
                    let subtrahend = if subtrahend { 1.0 } else { 0.0 };
                    Ok(LiteralData::Number(minuend - subtrahend))
                }
                LiteralData::String(subtrahend) => {
                    let subtrahend = parse_string_to_num!(
                        subtrahend,
                        error!(OperationError::InvalidParsedNumber(subtrahend))
                    )?;
                    Ok(LiteralData::Number(minuend - subtrahend))
                }
                LiteralData::Number(subtrahend) => Ok(LiteralData::Number(minuend - subtrahend)),
                LiteralData::Null => Ok(LiteralData::Number(minuend)),
            },
            LiteralData::Null => match rhs {
                LiteralData::Bool(subtrahend) => {
                    let subtrahend = if subtrahend { 1.0 } else { 0.0 };
                    Ok(LiteralData::Number(0.0 - subtrahend))
                }
                LiteralData::Number(subtrahend) => Ok(LiteralData::Number(0.0 - subtrahend)),
                LiteralData::String(subtrahend) => {
                    let subtrahend = parse_string_to_num!(
                        subtrahend,
                        error!(OperationError::InvalidParsedNumber(subtrahend))
                    )?;
                    Ok(LiteralData::Number(0.0 - subtrahend))
                }
                LiteralData::Null => Ok(LiteralData::Number(0.0))
            },
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

impl PartialOrd for LiteralData {
    fn gt(&self, _other: &Self) -> bool {
        todo!()
    }
    fn ge(&self, _other: &Self) -> bool {
        todo!()
    }
    fn le(&self, _other: &Self) -> bool {
        todo!()
    }
    fn lt(&self, _other: &Self) -> bool {
        todo!()
    }
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }
}
