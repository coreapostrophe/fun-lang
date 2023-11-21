#[macro_export]
macro_rules! token_lit_number {
    ($value:expr) => {
        crate::token::Token::new(TokenType::Number)
            .set_literal_data(crate::literal::LiteralData::Number($value))
    };
}

#[macro_export]
macro_rules! token_lit_string {
    ($value:expr) => {
        crate::token::Token::new(TokenType::String)
            .set_literal_data(crate::literal::LiteralData::String($value))
    };
}

#[macro_export]
macro_rules! parse_string_to_num {
    ($value:expr, $error:expr) => {
        match $value.parse::<f32>() {
            Ok(parsed_value) => Ok(parsed_value),
            Err(_) => Err($error),
        }
    };
}

#[macro_export]
macro_rules! error {
    ($error_type:expr) => {
        funlang_error::ErrorCascade::new($error_type)
    };
}
