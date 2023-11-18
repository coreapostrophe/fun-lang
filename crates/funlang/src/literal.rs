use std::{
    cmp::Ordering,
    ops::{Add, Div, Mul, Sub},
};

use funlang_error::ErrorCascade;

use crate::{error, errors::InterpreterError, parse_string_to_num};

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralData {
    String(String),
    Number(f32),
    Bool(bool),
    None,
}

impl LiteralData {
    fn parse_num(&self) -> Result<f32, ErrorCascade<InterpreterError>> {
        match self {
            LiteralData::Bool(bool_value) => Ok(if bool_value.clone() { 1.0 } else { 0.0 }),
            LiteralData::Number(number_value) => Ok(number_value.clone()),
            LiteralData::String(string_value) => {
                let parsed_string_value = parse_string_to_num!(
                    string_value,
                    error!(InterpreterError::InvalidParsedNumber(string_value.clone()))
                )?;
                Ok(parsed_string_value)
            }
            LiteralData::None => Ok(0.0),
        }
    }
}

impl Add for LiteralData {
    type Output = Result<LiteralData, ErrorCascade<InterpreterError>>;
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
                LiteralData::None => Ok(LiteralData::String(format!("{}null", addend1))),
            },
            _ => Ok(LiteralData::Number(self.parse_num()? + rhs.parse_num()?)),
        }
    }
}

impl Sub for LiteralData {
    type Output = Result<LiteralData, ErrorCascade<InterpreterError>>;
    fn sub(self, rhs: Self) -> Self::Output {
        Ok(LiteralData::Number(self.parse_num()? - rhs.parse_num()?))
    }
}

impl Mul for LiteralData {
    type Output = Result<LiteralData, ErrorCascade<InterpreterError>>;
    fn mul(self, rhs: Self) -> Self::Output {
        Ok(LiteralData::Number(self.parse_num()? * rhs.parse_num()?))
    }
}

impl Div for LiteralData {
    type Output = Result<LiteralData, ErrorCascade<InterpreterError>>;
    fn div(self, rhs: Self) -> Self::Output {
        Ok(LiteralData::Number(self.parse_num()? / rhs.parse_num()?))
    }
}

impl PartialOrd for LiteralData {
    fn gt(&self, other: &Self) -> bool {
        match self.parse_num() {
            Ok(self_value) => match other.parse_num() {
                Ok(other_value) => self_value > other_value,
                _ => false,
            },
            _ => false,
        }
    }
    fn ge(&self, other: &Self) -> bool {
        match self.parse_num() {
            Ok(self_value) => match other.parse_num() {
                Ok(other_value) => self_value >= other_value,
                _ => false,
            },
            _ => false,
        }
    }
    fn le(&self, other: &Self) -> bool {
        match self.parse_num() {
            Ok(self_value) => match other.parse_num() {
                Ok(other_value) => self_value <= other_value,
                _ => false,
            },
            _ => false,
        }
    }
    fn lt(&self, other: &Self) -> bool {
        match self.parse_num() {
            Ok(self_value) => match other.parse_num() {
                Ok(other_value) => self_value < other_value,
                _ => false,
            },
            _ => false,
        }
    }
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.parse_num() {
            Ok(self_value) => match other.parse_num() {
                Ok(other_value) => {
                    if self_value == other_value {
                        Some(Ordering::Equal)
                    } else if self_value > other_value {
                        Some(Ordering::Equal)
                    } else {
                        Some(Ordering::Equal)
                    }
                }
                _ => None,
            },
            _ => None,
        }
    }
}
