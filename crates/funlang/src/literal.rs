use std::ops::Add;

use crate::{errors::OperationError, parse_string_to_num};

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralData {
    String(String),
    Number(f32),
    Bool(bool),
    Null,
}

impl Add for LiteralData {
    type Output = Result<LiteralData, OperationError>;
    fn add(self, rhs: Self) -> Self::Output {
        match self {
            LiteralData::Bool(_) => Err(OperationError::InvalidBooleanAddition),
            LiteralData::Number(addend1) => match rhs {
                LiteralData::Bool(_) => Err(OperationError::InvalidBooleanAddition),
                LiteralData::Number(addend2) => Ok(LiteralData::Number(addend1 + addend2)),
                LiteralData::String(addend2) => {
                    let addend2 = parse_string_to_num!(addend2, OperationError::InvalidNumber)?;
                    Ok(LiteralData::Number(addend1 + addend2))
                }
                LiteralData::Null => Ok(LiteralData::Number(addend1)),
            },
            LiteralData::String(addend1) => match rhs {
                LiteralData::Bool(_) => Err(OperationError::InvalidBooleanAddition),
                LiteralData::Number(addend2) => {
                    let addend1 = parse_string_to_num!(addend1, OperationError::InvalidNumber)?;
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
