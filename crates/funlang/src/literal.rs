use std::ops::Add;

use crate::errors::operation_error::OperationError;

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralData {
    Identifier(String),
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
            LiteralData::Identifier(_) => Err(OperationError::InvalidIdentifierAddition),
            LiteralData::Number(addend1) => match rhs {
                LiteralData::Bool(_) => Err(OperationError::InvalidBooleanAddition),
                LiteralData::Identifier(_) => Err(OperationError::InvalidIdentifierAddition),
                LiteralData::Number(addend2) => Ok(LiteralData::Number(addend1 + addend2)),
                LiteralData::String(addend2) => {
                    let addend2 = match addend2.parse::<f32>() {
                        Ok(parsed_value) => Ok(parsed_value),
                        Err(_) => Err(OperationError::InvalidNumber),
                    }?;
                    Ok(LiteralData::Number(addend1 + addend2))
                }
                LiteralData::Null => Ok(LiteralData::Number(addend1)),
            },
            LiteralData::String(addend1) => match rhs {
                LiteralData::Bool(_) => Err(OperationError::InvalidBooleanAddition),
                LiteralData::Identifier(_) => Err(OperationError::InvalidIdentifierAddition),
                LiteralData::Number(addend2) => {
                    let addend1 = match addend1.parse::<f32>() {
                        Ok(parsed_value) => Ok(parsed_value),
                        Err(_) => Err(OperationError::InvalidNumber),
                    }?;
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
