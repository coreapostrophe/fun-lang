use funlang_error::{ErrorCascade, ErrorSpan};

use crate::{
    error,
    errors::LexerError,
    token::{Span, Token, TokenType},
    token_lit_number, token_lit_string,
};

#[derive(Debug)]
pub struct Lexer {
    pub source: Option<String>,
    start_index: usize,
    crawled_index: usize,
    line_number: usize,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            source: None,
            start_index: 0,
            crawled_index: 0,
            line_number: 1,
        }
    }

    fn unwrap_source(&self) -> Result<&String, ErrorCascade<LexerError>> {
        self.source
            .as_ref()
            .ok_or(error!(LexerError::MissingSource))
    }

    fn is_at_end(&self) -> Result<bool, ErrorCascade<LexerError>> {
        Ok(self.crawled_index >= self.unwrap_source()?.len())
    }

    fn advance(&mut self, value: usize) {
        self.crawled_index += value;
    }

    fn peek(&mut self, lookahead_offset: usize) -> Result<char, ErrorCascade<LexerError>> {
        match self
            .unwrap_source()?
            .chars()
            .nth(self.crawled_index + lookahead_offset)
        {
            Some(c) => Ok(c),
            None => Ok('\0'),
        }
    }

    fn match_next(&mut self, expected: char) -> Result<bool, ErrorCascade<LexerError>> {
        let is_match = match self.unwrap_source()?.chars().nth(self.crawled_index + 1) {
            Some(next_char) => next_char == expected,
            None => false,
        };
        if is_match {
            self.advance(1);
        }
        Ok(is_match)
    }

    fn string(&mut self) -> Result<Token, ErrorCascade<LexerError>> {
        let mut is_closed = false;
        'crawler: while !self.is_at_end()? {
            self.advance(1);
            if self.peek(0)? == '\n' {
                self.line_number += 1;
            }
            if self.peek(0)? == '"' {
                is_closed = true;
                break 'crawler;
            }
        }

        if !is_closed {
            Err(
                error!(LexerError::UnterminatedString).set_span(ErrorSpan::new(
                    self.line_number,
                    self.start_index,
                    1,
                )),
            )
        } else {
            let literal_value = &self.unwrap_source()?[(self.start_index + 1)..self.crawled_index];
            Ok(token_lit_string!(literal_value.to_string()))
        }
    }

    fn number(&mut self) -> Result<Token, ErrorCascade<LexerError>> {
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
        let parsed_literal_value = literal_value
            .parse::<f32>()
            .or(Err(error!(LexerError::InvalidCharacterIndex).set_span(
                ErrorSpan::new(self.line_number, self.start_index, 1),
            )))?;
        Ok(token_lit_number!(parsed_literal_value))
    }

    fn identifier(&mut self) -> Result<Token, ErrorCascade<LexerError>> {
        while self.peek(1)?.is_alphanumeric() {
            self.advance(1);
        }

        let literal_value = &self.unwrap_source()?[self.start_index..self.crawled_index + 1];
        let parsed_keyword = TokenType::get_keyword(literal_value);

        let token = match parsed_keyword {
            Some(keyword) => Token::new(keyword),
            None => Token::new(TokenType::Identifier(literal_value.to_string())),
        };

        Ok(token)
    }

    fn identify_token(&mut self) -> Result<Option<Token>, ErrorCascade<LexerError>> {
        let c = self.peek(0)?;

        let token = match c {
            '(' => Ok(Some(Token::new(TokenType::LeftParen))),
            ')' => Ok(Some(Token::new(TokenType::RightParen))),
            '[' => Ok(Some(Token::new(TokenType::LeftBracket))),
            ']' => Ok(Some(Token::new(TokenType::RightBracket))),
            '{' => Ok(Some(Token::new(TokenType::LeftBrace))),
            '}' => Ok(Some(Token::new(TokenType::RightBrace))),
            '.' => Ok(Some(Token::new(TokenType::Dot))),
            ',' => Ok(Some(Token::new(TokenType::Comma))),
            '-' => Ok(Some(Token::new(TokenType::Minus))),
            '+' => Ok(Some(Token::new(TokenType::Plus))),
            ';' => Ok(Some(Token::new(TokenType::Semicolon))),
            '*' => Ok(Some(Token::new(TokenType::Star))),
            '!' => {
                if self.match_next('=')? {
                    Ok(Some(Token::new(TokenType::BangEqual)))
                } else {
                    Ok(Some(Token::new(TokenType::Bang)))
                }
            }
            '=' => {
                if self.match_next('=')? {
                    Ok(Some(Token::new(TokenType::EqualEqual)))
                } else {
                    Ok(Some(Token::new(TokenType::Equal)))
                }
            }
            '<' => {
                if self.match_next('=')? {
                    Ok(Some(Token::new(TokenType::LessEqual)))
                } else {
                    Ok(Some(Token::new(TokenType::Less)))
                }
            }
            '>' => {
                if self.match_next('=')? {
                    Ok(Some(Token::new(TokenType::GreaterEqual)))
                } else {
                    Ok(Some(Token::new(TokenType::Greater)))
                }
            }
            '/' => {
                if self.match_next('/')? {
                    while self.peek(0)? != '\n' && !self.is_at_end()? {
                        self.advance(1);
                    }
                    Ok(None)
                } else {
                    Ok(Some(Token::new(TokenType::Slash)))
                }
            }
            ' ' => Ok(None),
            '\r' => Ok(None),
            '\t' => Ok(None),
            '\n' => {
                self.line_number += 1;
                Ok(None)
            }
            '"' => Ok(Some(self.string()?)),
            c => {
                if c.is_digit(10) {
                    Ok(Some(self.number()?))
                } else if c.is_alphabetic() {
                    Ok(Some(self.identifier()?))
                } else {
                    Err(error!(LexerError::UnexpectedCharacter))
                }
            }
        }?;

        self.advance(1);

        match token {
            Some(token) => Ok(Some(token.set_span(Span::new(
                self.line_number,
                self.start_index,
                self.crawled_index - self.start_index,
            )))),
            None => Ok(None),
        }
    }

    fn clear_state(&mut self) {
        self.start_index = 0;
        self.crawled_index = 0;
        self.line_number = 1;
    }

    pub fn tokenize(&mut self, source: &str) -> Result<Vec<Token>, ErrorCascade<LexerError>> {
        self.clear_state();
        self.source = Some(source.to_string());

        let mut tokens: Vec<Token> = vec![];

        while !self.is_at_end()? {
            self.start_index = self.crawled_index;

            match self.identify_token()? {
                Some(token) => tokens.push(token),
                None => (),
            }
        }

        tokens.push(Token::new(TokenType::EOF).set_span(Span::new(
            self.line_number,
            self.start_index + 1,
            0,
        )));

        Ok(tokens)
    }
}

#[cfg(test)]
mod lexer_tests {
    use super::*;

    #[test]
    fn parses_single_character_lexemes() {
        let mut lexer = Lexer::new();
        let result = lexer.tokenize("[](){},.-+;*/");

        assert!(result.is_ok());
        assert_eq!(
            format!("{:?}", result.unwrap()),
            format!(
                "{:?}",
                vec![
                    Token::new(TokenType::LeftBracket).set_span(Span::new(1, 0, 1)),
                    Token::new(TokenType::RightBracket).set_span(Span::new(1, 1, 1)),
                    Token::new(TokenType::LeftParen).set_span(Span::new(1, 2, 1)),
                    Token::new(TokenType::RightParen).set_span(Span::new(1, 3, 1)),
                    Token::new(TokenType::LeftBrace).set_span(Span::new(1, 4, 1)),
                    Token::new(TokenType::RightBrace).set_span(Span::new(1, 5, 1)),
                    Token::new(TokenType::Comma).set_span(Span::new(1, 6, 1)),
                    Token::new(TokenType::Dot).set_span(Span::new(1, 7, 1)),
                    Token::new(TokenType::Minus).set_span(Span::new(1, 8, 1)),
                    Token::new(TokenType::Plus).set_span(Span::new(1, 9, 1)),
                    Token::new(TokenType::Semicolon).set_span(Span::new(1, 10, 1)),
                    Token::new(TokenType::Star).set_span(Span::new(1, 11, 1)),
                    Token::new(TokenType::Slash).set_span(Span::new(1, 12, 1)),
                    Token::new(TokenType::EOF).set_span(Span::new(1, 13, 0))
                ]
            )
        )
    }

    #[test]
    fn parses_one_or_two_character_lexemes() {
        let mut lexer = Lexer::new();
        let result = lexer.tokenize("!!====<<=>>=");

        assert!(result.is_ok());
        assert_eq!(
            format!("{:?}", result.unwrap()),
            format!(
                "{:?}",
                vec![
                    Token::new(TokenType::Bang).set_span(Span::new(1, 0, 1)),
                    Token::new(TokenType::BangEqual).set_span(Span::new(1, 1, 2)),
                    Token::new(TokenType::EqualEqual).set_span(Span::new(1, 3, 2)),
                    Token::new(TokenType::Equal).set_span(Span::new(1, 5, 1)),
                    Token::new(TokenType::Less).set_span(Span::new(1, 6, 1)),
                    Token::new(TokenType::LessEqual).set_span(Span::new(1, 7, 2)),
                    Token::new(TokenType::Greater).set_span(Span::new(1, 9, 1)),
                    Token::new(TokenType::GreaterEqual).set_span(Span::new(1, 10, 2)),
                    Token::new(TokenType::EOF).set_span(Span::new(1, 11, 0)),
                ]
            )
        )
    }

    #[test]
    fn ignores_comments() {
        let mut lexer = Lexer::new();
        let result = lexer.tokenize("+//++++++\n+");

        assert!(result.is_ok());
        assert_eq!(
            format!("{:?}", result.unwrap()),
            format!(
                "{:?}",
                vec![
                    Token::new(TokenType::Plus).set_span(Span::new(1, 0, 1)),
                    Token::new(TokenType::Plus).set_span(Span::new(1, 10, 1)),
                    Token::new(TokenType::EOF).set_span(Span::new(1, 11, 0)),
                ]
            )
        )
    }

    #[test]
    fn ignores_white_space() {
        let mut lexer = Lexer::new();
        let result = lexer.tokenize("+ \t\r\n+");

        assert!(result.is_ok());
        assert_eq!(
            format!("{:?}", result.unwrap()),
            format!(
                "{:?}",
                vec![
                    Token::new(TokenType::Plus).set_span(Span::new(1, 0, 1)),
                    Token::new(TokenType::Plus).set_span(Span::new(2, 5, 1)),
                    Token::new(TokenType::EOF).set_span(Span::new(2, 6, 0)),
                ]
            )
        )
    }

    #[test]
    fn parses_string_literals() {
        let mut lexer = Lexer::new();
        let result = lexer.tokenize("+\"Example string\"+");

        assert!(result.is_ok());
        assert_eq!(
            format!("{:?}", result.unwrap()),
            format!(
                "{:?}",
                vec![
                    Token::new(TokenType::Plus).set_span(Span::new(1, 0, 1)),
                    token_lit_string!("Example string".to_string()).set_span(Span::new(1, 1, 16)),
                    Token::new(TokenType::Plus).set_span(Span::new(1, 17, 1)),
                    Token::new(TokenType::EOF).set_span(Span::new(1, 18, 0)),
                ]
            )
        )
    }

    #[test]
    fn parses_number_literals() {
        let mut lexer = Lexer::new();
        let result = lexer.tokenize("+1232.23+");

        assert!(result.is_ok());
        assert_eq!(
            format!("{:?}", result.unwrap()),
            format!(
                "{:?}",
                vec![
                    Token::new(TokenType::Plus).set_span(Span::new(1, 0, 1)),
                    token_lit_number!(1232.23_f32).set_span(Span::new(1, 1, 7)),
                    Token::new(TokenType::Plus).set_span(Span::new(1, 8, 1)),
                    Token::new(TokenType::EOF).set_span(Span::new(1, 9, 0)),
                ]
            )
        )
    }

    #[test]
    fn parses_identifiers() {
        let mut lexer = Lexer::new();
        let result = lexer.tokenize("+abcd1234+");

        assert!(result.is_ok());
        assert_eq!(
            format!("{:?}", result.unwrap()),
            format!(
                "{:?}",
                vec![
                    Token::new(TokenType::Plus).set_span(Span::new(1, 0, 1)),
                    Token::new(TokenType::Identifier("abcd1234".to_string()))
                        .set_span(Span::new(1, 1, 8)),
                    Token::new(TokenType::Plus).set_span(Span::new(1, 9, 1)),
                    Token::new(TokenType::EOF).set_span(Span::new(1, 10, 0)),
                ]
            )
        )
    }

    #[test]
    fn parses_keywords() {
        let mut lexer = Lexer::new();
        let result = lexer.tokenize("h+and+h");

        assert!(result.is_ok());
        assert_eq!(
            format!("{:?}", result.unwrap()),
            format!(
                "{:?}",
                vec![
                    Token::new(TokenType::Identifier("h".to_string())).set_span(Span::new(1, 0, 1)),
                    Token::new(TokenType::Plus).set_span(Span::new(1, 1, 1)),
                    Token::new(TokenType::And).set_span(Span::new(1, 2, 3)),
                    Token::new(TokenType::Plus).set_span(Span::new(1, 5, 1)),
                    Token::new(TokenType::Identifier("h".to_string())).set_span(Span::new(1, 6, 1)),
                    Token::new(TokenType::EOF).set_span(Span::new(1, 7, 0)),
                ]
            )
        )
    }
}
