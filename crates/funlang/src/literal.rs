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

impl LiteralData {
    fn parse_num(&self) -> Result<f32, ErrorCascade<OperationError>> {
        match self {
            LiteralData::Bool(bool_value) => Ok(if bool_value.clone() { 1.0 } else { 0.0 }),
            LiteralData::Number(number_value) => Ok(number_value.clone()),
            LiteralData::String(string_value) => {
                let parsed_string_value = parse_string_to_num!(
                    string_value,
                    error!(OperationError::InvalidParsedNumber(string_value.clone()))
                )?;
                Ok(parsed_string_value)
            }
            LiteralData::Null => Ok(0.0),
        }
    }
}

impl Add for LiteralData {
    type Output = Result<LiteralData, ErrorCascade<OperationError>>;
    fn add(self, rhs: Self) -> Self::Output {
        match self {
            LiteralData::String(ref addend1) => match rhs {
                LiteralData::Bool(ref addend2) => {
                    Ok(LiteralData::String(format!("{}{}", addend1, addend2)))
                }
                LiteralData::String(ref addend2) => {
                    Ok(LiteralData::String(format!("{}{}", addend1, addend2)))
                }
                LiteralData::Number(ref addend2) => {
                    Ok(LiteralData::String(format!("{}{}", addend1, addend2)))
                }
                LiteralData::Null => Ok(LiteralData::String(format!("{}null", addend1))),
            },
            _ => Ok(LiteralData::Number(self.parse_num()? + rhs.parse_num()?)),
        }
    }
}

impl Sub for LiteralData {
    type Output = Result<LiteralData, ErrorCascade<OperationError>>;
    fn sub(self, rhs: Self) -> Self::Output {
        Ok(LiteralData::Number(self.parse_num()? - rhs.parse_num()?))
    }
}

impl Mul for LiteralData {
    type Output = Result<LiteralData, ErrorCascade<OperationError>>;
    fn mul(self, rhs: Self) -> Self::Output {
        Ok(LiteralData::Number(self.parse_num()? * rhs.parse_num()?))
    }
}

impl Div for LiteralData {
    type Output = Result<LiteralData, ErrorCascade<OperationError>>;
    fn div(self, rhs: Self) -> Self::Output {
        Ok(LiteralData::Number(self.parse_num()? / rhs.parse_num()?))
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
