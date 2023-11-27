use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Sub},
};

use funlang_error::ErrorCascade;

use crate::{error, errors::InterpreterError, functions::Function, parse_string_to_num};

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralData {
    String(String),
    Number(f32),
    Bool(bool),
    Function(Function),
    None,
}

impl LiteralData {
    fn parse_num(&self) -> Result<f32, ErrorCascade<InterpreterError>> {
        match self {
            Self::Bool(bool_value) => Ok(if bool_value.clone() { 1.0 } else { 0.0 }),
            Self::Number(number_value) => Ok(number_value.clone()),
            Self::String(string_value) => {
                let parsed_string_value = parse_string_to_num!(
                    string_value,
                    error!(InterpreterError::InvalidParsedNumber(string_value.clone()))
                )?;
                Ok(parsed_string_value)
            }
            Self::None => Ok(0.0),
            Self::Function(_) => Ok(1.0),
        }
    }

    pub fn is_truthy(&self) -> Result<bool, ErrorCascade<InterpreterError>> {
        Ok(self.parse_num()? != 0.0)
    }

    pub fn is_falsy(&self) -> Result<bool, ErrorCascade<InterpreterError>> {
        Ok(!self.is_truthy()?)
    }
}

impl Add for LiteralData {
    type Output = Result<Self, ErrorCascade<InterpreterError>>;
    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Self::String(ref addend1) => match rhs {
                Self::Function(ref addend2) => Ok(Self::String(format!("{}{}", addend1, addend2))),
                Self::Bool(ref addend2) => Ok(Self::String(format!("{}{}", addend1, addend2))),
                Self::String(ref addend2) => Ok(Self::String(format!("{}{}", addend1, addend2))),
                Self::Number(ref addend2) => Ok(Self::String(format!("{}{}", addend1, addend2))),
                Self::None => Ok(Self::String(format!("{}null", addend1))),
            },
            _ => Ok(Self::Number(self.parse_num()? + rhs.parse_num()?)),
        }
    }
}

impl Sub for LiteralData {
    type Output = Result<Self, ErrorCascade<InterpreterError>>;
    fn sub(self, rhs: Self) -> Self::Output {
        Ok(Self::Number(self.parse_num()? - rhs.parse_num()?))
    }
}

impl Mul for LiteralData {
    type Output = Result<Self, ErrorCascade<InterpreterError>>;
    fn mul(self, rhs: Self) -> Self::Output {
        Ok(Self::Number(self.parse_num()? * rhs.parse_num()?))
    }
}

impl Div for LiteralData {
    type Output = Result<Self, ErrorCascade<InterpreterError>>;
    fn div(self, rhs: Self) -> Self::Output {
        Ok(Self::Number(self.parse_num()? / rhs.parse_num()?))
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

impl Display for LiteralData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Bool(bool_value) => write!(f, "{}", bool_value),
            Self::String(string_value) => write!(f, "{}", string_value),
            Self::Number(number_value) => write!(f, "{}", number_value),
            Self::None => write!(f, "None"),
            Self::Function(function_value) => write!(f, "{}", function_value),
        }
    }
}
