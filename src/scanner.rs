use crate::{
    error::{CompilerError, Source},
    source,
    token::{Token, TokenType},
};

struct IdentifyTokenResult {
    token: Token,
    advance_count: u32,
    is_new_line: bool,
}

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

    fn advance(&mut self) -> Result<char, CompilerError> {
        let c = self.source.chars().nth(self.current_char_index).ok_or(
            CompilerError::IndexOutOfBounds(source!(
                self.current_line_number,
                self.current_line_offset
            )),
        )?;
        Ok(c)
    }

    fn match_next(&mut self, expected: char) -> Result<bool, CompilerError> {
        let result = match self.source.chars().nth(self.current_char_index + 1) {
            Some(next_char) => next_char == expected,
            None => false,
        };
        if result {
            self.current_char_index += 1;
            self.current_line_offset += 1;
        }
        Ok(result)
    }

    fn scan_token(&mut self) -> Result<(), CompilerError> {
        let c = self.advance()?;

        let token_type = match c {
            '(' => Ok(TokenType::LeftParen),
            ')' => Ok(TokenType::RightParen),
            '[' => Ok(TokenType::LeftBracket),
            ']' => Ok(TokenType::RightBracket),
            '{' => Ok(TokenType::LeftBrace),
            '}' => Ok(TokenType::RightBrace),
            '.' => Ok(TokenType::Dot),
            ',' => Ok(TokenType::Comma),
            '-' => Ok(TokenType::Minus),
            '+' => Ok(TokenType::Plus),
            ';' => Ok(TokenType::Semicolon),
            '*' => Ok(TokenType::Star),
            '!' => {
                if self.match_next('=')? {
                    Ok(TokenType::BangEqual)
                } else {
                    Ok(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_next('=')? {
                    Ok(TokenType::EqualEqual)
                } else {
                    Ok(TokenType::Equal)
                }
            }
            '<' => {
                if self.match_next('=')? {
                    Ok(TokenType::LessEqual)
                } else {
                    Ok(TokenType::Less)
                }
            }
            '>' => {
                if self.match_next('=')? {
                    Ok(TokenType::GreaterEqual)
                } else {
                    Ok(TokenType::Greater)
                }
            }
            _ => Err(CompilerError::UnexpectedCharacter(source!(
                self.current_line_number,
                self.current_line_offset
            ))),
        }?;

        self.tokens.push(Token::new(token_type));
        
        self.current_char_index += 1;
        self.current_line_offset += 1;

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
        let mut scanner = Scanner::new(r"[](){},.-+;*");
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
                    Token::new(TokenType::EOF)
                ]
            )
        )
    }

    #[test]
    fn matches_one_or_two_character_lexemes() {
        let mut scanner = Scanner::new(r"!!====<<=>>=");
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
}
