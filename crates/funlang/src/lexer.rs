use crate::{
    errors::lexer_errors::LexerError,
    source,
    token::{Span, Token, TokenType},
    token_lit_identifier, token_lit_number, token_lit_string,
};

#[derive(Debug)]
pub struct Lexer {
    pub source: Option<String>,
    pub tokens: Vec<Token>,
    start_index: usize,
    crawled_index: usize,
    current_line_number: u32,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            source: None,
            tokens: vec![],
            start_index: 0,
            crawled_index: 0,
            current_line_number: 1,
        }
    }

    fn unwrap_source(&self) -> Result<&String, LexerError> {
        self.source.as_ref().ok_or(LexerError::MissingSource)
    }

    fn is_at_end(&self) -> Result<bool, LexerError> {
        Ok(self.crawled_index >= self.unwrap_source()?.len())
    }

    fn advance(&mut self, value: usize) {
        self.crawled_index += value;
    }

    fn peek(&mut self, lookahead_offset: usize) -> Result<char, LexerError> {
        match self
            .unwrap_source()?
            .chars()
            .nth(self.crawled_index + lookahead_offset)
        {
            Some(c) => Ok(c),
            None => Ok('\0'),
        }
    }

    fn match_next(&mut self, expected: char) -> Result<bool, LexerError> {
        let is_match = match self.unwrap_source()?.chars().nth(self.crawled_index + 1) {
            Some(next_char) => next_char == expected,
            None => false,
        };
        if is_match {
            self.advance(1);
        }
        Ok(is_match)
    }

    fn string(&mut self) -> Result<Token, LexerError> {
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
            Err(LexerError::UnterminatedString(source!(
                self.current_line_number,
                self.start_index as u32
            )))
        } else {
            let literal_value = &self.unwrap_source()?[(self.start_index + 1)..self.crawled_index];
            Ok(token_lit_string!(literal_value.to_string()))
        }
    }

    fn number(&mut self) -> Result<Token, LexerError> {
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
            Err(_) => Err(LexerError::InvalidCharacterIndex(source!(
                self.current_line_number,
                self.start_index as u32
            ))),
        }?;
        Ok(token_lit_number!(parsed_literal_value))
    }

    fn identifier(&mut self) -> Result<Token, LexerError> {
        while self.peek(1)?.is_alphanumeric() {
            self.advance(1);
        }

        let literal_value = &self.unwrap_source()?[self.start_index..self.crawled_index + 1];
        let parsed_keyword = TokenType::get_keyword(literal_value);

        let token = match parsed_keyword {
            Some(keyword) => Token::new(keyword),
            None => token_lit_identifier!(literal_value.to_string()),
        };

        Ok(token)
    }

    fn scan_token(&mut self) -> Result<(), LexerError> {
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
                self.current_line_number += 1;
                Ok(None)
            }
            '"' => Ok(Some(self.string()?)),
            c => {
                if c.is_digit(10) {
                    Ok(Some(self.number()?))
                } else if c.is_alphabetic() {
                    Ok(Some(self.identifier()?))
                } else {
                    Err(LexerError::UnexpectedCharacter)
                }
            }
        }?;

        match token {
            Some(token) => {
                let token = token.set_span(Span {
                    line: self.current_line_number,
                    line_offset: self.start_index as u32,
                });
                self.tokens.push(token);
            }
            None => (),
        }

        self.advance(1);

        Ok(())
    }

    fn clear_state(&mut self) {
        self.start_index = 0;
        self.crawled_index = 0;
        self.current_line_number = 1;
    }

    pub fn tokenize(&mut self, source: &str) -> Result<(), LexerError> {
        self.clear_state();
        self.source = Some(source.to_string());

        while !self.is_at_end()? {
            self.start_index = self.crawled_index;
            self.scan_token()?;
        }

        self.tokens.push(Token::new(TokenType::EOF).set_span(Span {
            line: self.current_line_number,
            line_offset: self.start_index as u32 + 1,
        }));

        Ok(())
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
            format!("{:?}", lexer.tokens),
            format!(
                "{:?}",
                vec![
                    Token::new(TokenType::LeftBracket).set_span(Span {
                        line: 1,
                        line_offset: 0
                    }),
                    Token::new(TokenType::RightBracket).set_span(Span {
                        line: 1,
                        line_offset: 1
                    }),
                    Token::new(TokenType::LeftParen).set_span(Span {
                        line: 1,
                        line_offset: 2
                    }),
                    Token::new(TokenType::RightParen).set_span(Span {
                        line: 1,
                        line_offset: 3
                    }),
                    Token::new(TokenType::LeftBrace).set_span(Span {
                        line: 1,
                        line_offset: 4
                    }),
                    Token::new(TokenType::RightBrace).set_span(Span {
                        line: 1,
                        line_offset: 5
                    }),
                    Token::new(TokenType::Comma).set_span(Span {
                        line: 1,
                        line_offset: 6
                    }),
                    Token::new(TokenType::Dot).set_span(Span {
                        line: 1,
                        line_offset: 7
                    }),
                    Token::new(TokenType::Minus).set_span(Span {
                        line: 1,
                        line_offset: 8
                    }),
                    Token::new(TokenType::Plus).set_span(Span {
                        line: 1,
                        line_offset: 9
                    }),
                    Token::new(TokenType::Semicolon).set_span(Span {
                        line: 1,
                        line_offset: 10
                    }),
                    Token::new(TokenType::Star).set_span(Span {
                        line: 1,
                        line_offset: 11
                    }),
                    Token::new(TokenType::Slash).set_span(Span {
                        line: 1,
                        line_offset: 12
                    }),
                    Token::new(TokenType::EOF).set_span(Span {
                        line: 1,
                        line_offset: 13
                    })
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
            format!("{:?}", lexer.tokens),
            format!(
                "{:?}",
                vec![
                    Token::new(TokenType::Bang).set_span(Span {
                        line: 1,
                        line_offset: 0
                    }),
                    Token::new(TokenType::BangEqual).set_span(Span {
                        line: 1,
                        line_offset: 1
                    }),
                    Token::new(TokenType::EqualEqual).set_span(Span {
                        line: 1,
                        line_offset: 3
                    }),
                    Token::new(TokenType::Equal).set_span(Span {
                        line: 1,
                        line_offset: 5
                    }),
                    Token::new(TokenType::Less).set_span(Span {
                        line: 1,
                        line_offset: 6
                    }),
                    Token::new(TokenType::LessEqual).set_span(Span {
                        line: 1,
                        line_offset: 7
                    }),
                    Token::new(TokenType::Greater).set_span(Span {
                        line: 1,
                        line_offset: 9
                    }),
                    Token::new(TokenType::GreaterEqual).set_span(Span {
                        line: 1,
                        line_offset: 10
                    }),
                    Token::new(TokenType::EOF).set_span(Span {
                        line: 1,
                        line_offset: 11
                    }),
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
            format!("{:?}", lexer.tokens),
            format!(
                "{:?}",
                vec![
                    Token::new(TokenType::Plus).set_span(Span {
                        line: 1,
                        line_offset: 0
                    }),
                    Token::new(TokenType::Plus).set_span(Span {
                        line: 1,
                        line_offset: 10
                    }),
                    Token::new(TokenType::EOF).set_span(Span {
                        line: 1,
                        line_offset: 11
                    }),
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
            format!("{:?}", lexer.tokens),
            format!(
                "{:?}",
                vec![
                    Token::new(TokenType::Plus).set_span(Span {
                        line: 1,
                        line_offset: 0
                    }),
                    Token::new(TokenType::Plus).set_span(Span {
                        line: 2,
                        line_offset: 5
                    }),
                    Token::new(TokenType::EOF).set_span(Span {
                        line: 2,
                        line_offset: 6
                    }),
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
            format!("{:?}", lexer.tokens),
            format!(
                "{:?}",
                vec![
                    Token::new(TokenType::Plus).set_span(Span {
                        line: 1,
                        line_offset: 0
                    }),
                    token_lit_string!("Example string".to_string()).set_span(Span {
                        line: 1,
                        line_offset: 1
                    }),
                    Token::new(TokenType::Plus).set_span(Span {
                        line: 1,
                        line_offset: 17
                    }),
                    Token::new(TokenType::EOF).set_span(Span {
                        line: 1,
                        line_offset: 18
                    }),
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
            format!("{:?}", lexer.tokens),
            format!(
                "{:?}",
                vec![
                    Token::new(TokenType::Plus).set_span(Span {
                        line: 1,
                        line_offset: 0
                    }),
                    token_lit_number!(1232.23_f32).set_span(Span {
                        line: 1,
                        line_offset: 1
                    }),
                    Token::new(TokenType::Plus).set_span(Span {
                        line: 1,
                        line_offset: 8
                    }),
                    Token::new(TokenType::EOF).set_span(Span {
                        line: 1,
                        line_offset: 9
                    }),
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
            format!("{:?}", lexer.tokens),
            format!(
                "{:?}",
                vec![
                    Token::new(TokenType::Plus).set_span(Span {
                        line: 1,
                        line_offset: 0
                    }),
                    token_lit_identifier!("abcd1234".to_string()).set_span(Span {
                        line: 1,
                        line_offset: 1
                    }),
                    Token::new(TokenType::Plus).set_span(Span {
                        line: 1,
                        line_offset: 9
                    }),
                    Token::new(TokenType::EOF).set_span(Span {
                        line: 1,
                        line_offset: 10
                    }),
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
            format!("{:?}", lexer.tokens),
            format!(
                "{:?}",
                vec![
                    token_lit_identifier!("h".to_string()).set_span(Span {
                        line: 1,
                        line_offset: 0
                    }),
                    Token::new(TokenType::Plus).set_span(Span {
                        line: 1,
                        line_offset: 1
                    }),
                    Token::new(TokenType::And).set_span(Span {
                        line: 1,
                        line_offset: 2
                    }),
                    Token::new(TokenType::Plus).set_span(Span {
                        line: 1,
                        line_offset: 5
                    }),
                    token_lit_identifier!("h".to_string()).set_span(Span {
                        line: 1,
                        line_offset: 6
                    }),
                    Token::new(TokenType::EOF).set_span(Span {
                        line: 1,
                        line_offset: 7
                    }),
                ]
            )
        )
    }
}
