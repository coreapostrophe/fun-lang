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

    fn identify_token(&self) -> Result<IdentifyTokenResult, CompilerError> {
        let c = self
            .source
            .chars()
            .nth(self.current_char_index)
            .ok_or(source!(self.current_line_number, self.current_line_offset));
        let mut advance_count: usize = 1;

        todo!()
    }

    fn is_at_end(&self) -> bool {
        self.current_char_index >= self.source.len()
    }

    fn scan_tokens(&mut self) -> Result<(), CompilerError> {
        while !self.is_at_end() {
            self.start_index = self.current_char_index;

            let result = self.identify_token()?;
        }

        self.tokens.push(
            Token::new(TokenType::EndOfFile)
                .set_line(self.current_line_number)
                .set_lexeme("".to_string()),
        );

        Ok(())
    }
}
