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
