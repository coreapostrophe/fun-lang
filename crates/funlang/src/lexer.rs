use crate::{
    error::InterpreterError,
    literal_identifier, literal_number, literal_string, source,
    token::{Token, TokenType},
};

#[derive(Debug)]
pub struct Lexer {
    source: Option<String>,
    tokens: Vec<Token>,
    start_index: usize,
    crawled_index: usize,
    current_line_number: u32,
    current_line_offset: u32,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            source: None,
            tokens: vec![],
            start_index: 0,
            crawled_index: 0,
            current_line_number: 1,
            current_line_offset: 1,
        }
    }

    fn unwrap_source(&self) -> Result<&String, InterpreterError> {
        self.source.as_ref().ok_or(InterpreterError::UnprovidedSource)
    }

    fn is_at_end(&self) -> Result<bool, InterpreterError> {
        Ok(self.crawled_index >= self.unwrap_source()?.len())
    }

    fn advance(&mut self, value: usize) {
        self.crawled_index += value;
        self.current_line_offset += value as u32;
    }

    fn peek(&mut self, lookahead_offset: usize) -> Result<char, InterpreterError> {
        match self
            .unwrap_source()?
            .chars()
            .nth(self.crawled_index + lookahead_offset)
        {
            Some(c) => Ok(c),
            None => Ok('\0'),
        }
    }

    fn match_next(&mut self, expected: char) -> Result<bool, InterpreterError> {
        let result = match self.unwrap_source()?.chars().nth(self.crawled_index + 1) {
            Some(next_char) => next_char == expected,
            None => false,
        };
        if result {
            self.advance(1);
        }
        Ok(result)
    }

    fn string(&mut self) -> Result<TokenType, InterpreterError> {
        let mut is_closed = false;
        'crawler: while !self.is_at_end()? {
            self.advance(1);
            if self.peek(0)? == '\n' {
                self.current_line_number += 1;
            }
            if self.peek(0)? == '"' {
                is_closed = true;
                break 'crawler;
            }
        }

        if !is_closed {
            Err(InterpreterError::UnterminatedString(source!(
                self.current_line_number,
                self.current_line_offset
            )))
        } else {
            let literal_value = &self.unwrap_source()?[(self.start_index + 1)..self.crawled_index];
            Ok(literal_string!(literal_value.to_string()))
        }
    }

    fn number(&mut self) -> Result<TokenType, InterpreterError> {
        while self.peek(1)?.is_digit(10) {
            self.advance(1);
        }
        if self.peek(1)? == '.' && self.peek(2)?.is_digit(10) {
            self.advance(2);
            while self.peek(1)?.is_digit(10) {
                self.advance(1);
            }
        }
        let literal_value = &self.unwrap_source()?[self.start_index..self.crawled_index + 1];
        let parsed_literal_value = match literal_value.parse::<f32>() {
            Ok(value) => Ok(value),
            Err(_) => Err(InterpreterError::InvalidCharacterIndex(source!(
                self.current_line_number,
                self.current_line_offset
            ))),
        }?;
        Ok(literal_number!(parsed_literal_value))
    }

    fn identifier(&mut self) -> Result<TokenType, InterpreterError> {
        while self.peek(1)?.is_alphanumeric() {
            self.advance(1);
        }

        let literal_value = &self.unwrap_source()?[self.start_index..self.crawled_index + 1];
        let parsed_keyword = TokenType::get_keyword(literal_value)
            .unwrap_or(literal_identifier!(literal_value.to_string()));

        Ok(parsed_keyword)
    }

    fn scan_token(&mut self) -> Result<(), InterpreterError> {
        let c = self.peek(0)?;

        let token_type = match c {
            '(' => Ok(Some(TokenType::LeftParen)),
            ')' => Ok(Some(TokenType::RightParen)),
            '[' => Ok(Some(TokenType::LeftBracket)),
            ']' => Ok(Some(TokenType::RightBracket)),
            '{' => Ok(Some(TokenType::LeftBrace)),
            '}' => Ok(Some(TokenType::RightBrace)),
            '.' => Ok(Some(TokenType::Dot)),
            ',' => Ok(Some(TokenType::Comma)),
            '-' => Ok(Some(TokenType::Minus)),
            '+' => Ok(Some(TokenType::Plus)),
            ';' => Ok(Some(TokenType::Semicolon)),
            '*' => Ok(Some(TokenType::Star)),
            '!' => {
                if self.match_next('=')? {
                    Ok(Some(TokenType::BangEqual))
                } else {
                    Ok(Some(TokenType::Bang))
                }
            }
            '=' => {
                if self.match_next('=')? {
                    Ok(Some(TokenType::EqualEqual))
                } else {
                    Ok(Some(TokenType::Equal))
                }
            }
            '<' => {
                if self.match_next('=')? {
                    Ok(Some(TokenType::LessEqual))
                } else {
                    Ok(Some(TokenType::Less))
                }
            }
            '>' => {
                if self.match_next('=')? {
                    Ok(Some(TokenType::GreaterEqual))
                } else {
                    Ok(Some(TokenType::Greater))
                }
            }
            '/' => {
                if self.match_next('/')? {
                    while self.peek(0)? != '\n' && !self.is_at_end()? {
                        self.advance(1);
                    }
                    Ok(None)
                } else {
                    Ok(Some(TokenType::Slash))
                }
            }
            ' ' => Ok(None),
            '\r' => Ok(None),
            '\t' => Ok(None),
            '\n' => {
                self.current_line_number += 1;
                self.current_line_offset = 1;
                Ok(None)
            }
            '"' => Ok(Some(self.string()?)),
            c => {
                if c.is_digit(10) {
                    Ok(Some(self.number()?))
                } else if c.is_alphabetic() {
                    Ok(Some(self.identifier()?))
                } else {
                    Err(InterpreterError::UnexpectedCharacter(source!(
                        self.current_line_number,
                        self.current_line_offset
                    )))
                }
            }
        }?;

        match token_type {
            Some(token_type) => self.tokens.push(Token::new(token_type)),
            None => (),
        }

        self.advance(1);

        Ok(())
    }

    fn clear_state(&mut self) {
        self.start_index = 0;
        self.crawled_index = 0;
        self.current_line_number = 1;
        self.current_line_offset = 1;
    }

    pub fn tokenize(&mut self, source: &str) -> Result<(), InterpreterError> {
        self.clear_state();
        self.source = Some(source.to_string());

        while !self.is_at_end()? {
            self.start_index = self.crawled_index;
            self.scan_token()?;
        }

        self.tokens.push(Token::new(TokenType::EOF));

        Ok(())
    }
}

#[cfg(test)]
mod lexer_tests {
    use super::*;
    use crate::{literal_identifier, literal_number, literal_string};

    #[test]
    fn parses_single_character_lexemes() {
        let mut scanner = Lexer::new();
        let result = scanner.tokenize("[](){},.-+;*/");

        assert!(result.is_ok());
        assert_eq!(
            format!("{:?}", scanner.tokens),
            format!(
                "{:?}",
                vec![
                    Token::new(TokenType::LeftBracket),
                    Token::new(TokenType::RightBracket),
                    Token::new(TokenType::LeftParen),
                    Token::new(TokenType::RightParen),
                    Token::new(TokenType::LeftBrace),
                    Token::new(TokenType::RightBrace),
                    Token::new(TokenType::Comma),
                    Token::new(TokenType::Dot),
                    Token::new(TokenType::Minus),
                    Token::new(TokenType::Plus),
                    Token::new(TokenType::Semicolon),
                    Token::new(TokenType::Star),
                    Token::new(TokenType::Slash),
                    Token::new(TokenType::EOF)
                ]
            )
        )
    }

    #[test]
    fn parses_one_or_two_character_lexemes() {
        let mut scanner = Lexer::new();
        let result = scanner.tokenize("!!====<<=>>=");

        assert!(result.is_ok());
        assert_eq!(
            format!("{:?}", scanner.tokens),
            format!(
                "{:?}",
                vec![
                    Token::new(TokenType::Bang),
                    Token::new(TokenType::BangEqual),
                    Token::new(TokenType::EqualEqual),
                    Token::new(TokenType::Equal),
                    Token::new(TokenType::Less),
                    Token::new(TokenType::LessEqual),
                    Token::new(TokenType::Greater),
                    Token::new(TokenType::GreaterEqual),
                    Token::new(TokenType::EOF),
                ]
            )
        )
    }

    #[test]
    fn ignores_comments() {
        let mut scanner = Lexer::new();
        let result = scanner.tokenize("+//++++++\n+");

        assert!(result.is_ok());
        assert_eq!(
            format!("{:?}", scanner.tokens),
            format!(
                "{:?}",
                vec![
                    Token::new(TokenType::Plus),
                    Token::new(TokenType::Plus),
                    Token::new(TokenType::EOF),
                ]
            )
        )
    }

    #[test]
    fn ignores_white_space() {
        let mut scanner = Lexer::new();
        let result = scanner.tokenize("+ \t\r\n+");

        assert!(result.is_ok());
        assert_eq!(
            format!("{:?}", scanner.tokens),
            format!(
                "{:?}",
                vec![
                    Token::new(TokenType::Plus),
                    Token::new(TokenType::Plus),
                    Token::new(TokenType::EOF),
                ]
            )
        )
    }

    #[test]
    fn parses_string_literals() {
        let mut scanner = Lexer::new();
        let result = scanner.tokenize("+\"Example string\"+");

        assert!(result.is_ok());
        assert_eq!(
            format!("{:?}", scanner.tokens),
            format!(
                "{:?}",
                vec![
                    Token::new(TokenType::Plus),
                    Token::new(literal_string!("Example string".to_string())),
                    Token::new(TokenType::Plus),
                    Token::new(TokenType::EOF),
                ]
            )
        )
    }

    #[test]
    fn parses_number_literals() {
        let mut scanner = Lexer::new();
        let result = scanner.tokenize("+1232.23+");

        assert!(result.is_ok());
        assert_eq!(
            format!("{:?}", scanner.tokens),
            format!(
                "{:?}",
                vec![
                    Token::new(TokenType::Plus),
                    Token::new(literal_number!(1232.23_f32)),
                    Token::new(TokenType::Plus),
                    Token::new(TokenType::EOF),
                ]
            )
        )
    }

    #[test]
    fn parses_identifiers() {
        let mut scanner = Lexer::new();
        let result = scanner.tokenize("+abcd1234+");

        assert!(result.is_ok());
        assert_eq!(
            format!("{:?}", scanner.tokens),
            format!(
                "{:?}",
                vec![
                    Token::new(TokenType::Plus),
                    Token::new(literal_identifier!("abcd1234".to_string())),
                    Token::new(TokenType::Plus),
                    Token::new(TokenType::EOF),
                ]
            )
        )
    }

    #[test]
    fn parses_keywords() {
        let mut scanner = Lexer::new();
        let result = scanner.tokenize("h+and+h");

        assert!(result.is_ok());
        assert_eq!(
            format!("{:?}", scanner.tokens),
            format!(
                "{:?}",
                vec![
                    Token::new(literal_identifier!("h".to_string())),
                    Token::new(TokenType::Plus),
                    Token::new(TokenType::And),
                    Token::new(TokenType::Plus),
                    Token::new(literal_identifier!("h".to_string())),
                    Token::new(TokenType::EOF),
                ]
            )
        )
    }
}
