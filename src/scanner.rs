use crate::{
    error::{CompilerError, Source},
    source,
    token::{Token, TokenType},
};

#[derive(Debug)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start_index: usize,
    current_char_index: usize,
    current_line_number: u32,
    current_line_offset: u32,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            tokens: vec![],
            start_index: 0,
            current_char_index: 0,
            current_line_number: 1,
            current_line_offset: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current_char_index >= self.source.len()
    }

    fn advance(&mut self) {
        self.current_char_index += 1;
        self.current_line_offset += 1;
    }

    fn peek(&mut self, offset: usize) -> char {
        match self.source.chars().nth(self.current_char_index + offset) {
            Some(c) => c,
            None => '\0',
        }
    }

    fn match_next(&mut self, expected: char) -> bool {
        let result = match self.source.chars().nth(self.current_char_index + 1) {
            Some(next_char) => next_char == expected,
            None => false,
        };
        if result {
            self.advance();
        }
        result
    }

    fn scan_token(&mut self) -> Result<(), CompilerError> {
        let c = self.peek(0);

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
                if self.match_next('=') {
                    Ok(Some(TokenType::BangEqual))
                } else {
                    Ok(Some(TokenType::Bang))
                }
            }
            '=' => {
                if self.match_next('=') {
                    Ok(Some(TokenType::EqualEqual))
                } else {
                    Ok(Some(TokenType::Equal))
                }
            }
            '<' => {
                if self.match_next('=') {
                    Ok(Some(TokenType::LessEqual))
                } else {
                    Ok(Some(TokenType::Less))
                }
            }
            '>' => {
                if self.match_next('=') {
                    Ok(Some(TokenType::GreaterEqual))
                } else {
                    Ok(Some(TokenType::Greater))
                }
            }
            '/' => {
                if self.match_next('/') {
                    while self.peek(0) != '\n' && !self.is_at_end() {
                        self.advance();
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
            },
            _ => Err(CompilerError::UnexpectedCharacter(source!(
                self.current_line_number,
                self.current_line_offset
            ))),
        }?;

        match token_type {
            Some(token_type) => self.tokens.push(Token::new(token_type)),
            None => (),
        }

        self.advance();

        Ok(())
    }

    pub fn scan_tokens(&mut self) -> Result<(), CompilerError> {
        while !self.is_at_end() {
            self.start_index = self.current_char_index;
            self.scan_token()?;
        }

        self.tokens.push(Token::new(TokenType::EOF));

        Ok(())
    }
}

#[cfg(test)]
mod scanner_tests {
    use super::*;

    #[test]
    fn matches_single_character_lexemes() {
        let mut scanner = Scanner::new("[](){},.-+;*/");
        let result = scanner.scan_tokens();

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
    fn matches_one_or_two_character_lexemes() {
        let mut scanner = Scanner::new("!!====<<=>>=");
        let result = scanner.scan_tokens();

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
        let mut scanner = Scanner::new("+//++++++\n+");
        let result = scanner.scan_tokens();

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
        let mut scanner = Scanner::new("+ \t\r\n+");
        let result = scanner.scan_tokens();

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
}
