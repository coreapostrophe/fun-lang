#[macro_export]
macro_rules! source {
    ($line_number:expr, $line_offset:expr) => {
        crate::error::Source::new($line_number, $line_offset)
    };
}

#[macro_export]
macro_rules! literal_number {
    ($value:expr) => {
        crate::token::TokenType::Literal(
            crate::token::LiteralData::Number($value)
        )
    };
}

#[macro_export]
macro_rules! literal_string {
    ($value:expr) => {
        crate::token::TokenType::Literal(
            crate::token::LiteralData::String($value)
        )
    };
}

#[macro_export]
macro_rules! literal_identifier {
    ($value:expr) => {
        crate::token::TokenType::Literal(
            crate::token::LiteralData::Identifier($value)
        )
    };
}