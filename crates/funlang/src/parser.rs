use crate::{
    error::InterpreterError,
    expr::{BinaryExpr, Expr, LiteralExpr, UnaryExpr},
    token::{Token, TokenType},
};

pub struct Parser {
    tokens: Option<Vec<Token>>,
    crawled_index: usize,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            tokens: None,
            crawled_index: 0,
        }
    }

    fn unwrap_tokens(&self) -> Result<&Vec<Token>, InterpreterError> {
        self.tokens
            .as_ref()
            .ok_or(InterpreterError::UnprovidedTokens)
    }

    fn primary(&mut self) -> Result<Expr, InterpreterError> {
        if self.r#match(vec![TokenType::False])? {
            Ok(Expr::Literal(Box::new(LiteralExpr {
                literal: crate::token::LiteralData::False,
            })))
        } else if self.r#match(vec![TokenType::True])? {
            Ok(Expr::Literal(Box::new(LiteralExpr {
                literal: crate::token::LiteralData::True,
            })))
        } else if self.r#match(vec![TokenType::Null])? {
            Ok(Expr::Literal(Box::new(LiteralExpr {
                literal: crate::token::LiteralData::Null,
            })))
        } else {
            todo!()
        }
    }

    fn unary(&mut self) -> Result<Expr, InterpreterError> {
        if self.r#match(vec![TokenType::Bang, TokenType::Minus])? {
            let operator = self.previous()?;
            let right = self.unary()?;
            Ok(Expr::Unary(Box::new(UnaryExpr { operator, right })))
        } else {
            Ok(self.primary()?)
        }
    }

    fn factor(&mut self) -> Result<Expr, InterpreterError> {
        let mut expr = self.unary()?;

        while self.r#match(vec![TokenType::Slash, TokenType::Star])? {
            let operator: Token = self.previous()?;
            let right: Expr = self.unary()?;
            expr = Expr::Binary(Box::new(BinaryExpr {
                left: expr,
                operator,
                right,
            }));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, InterpreterError> {
        let mut expr = self.factor()?;

        while self.r#match(vec![TokenType::Minus, TokenType::Plus])? {
            let operator: Token = self.previous()?;
            let right: Expr = self.factor()?;
            expr = Expr::Binary(Box::new(BinaryExpr {
                left: expr,
                operator,
                right,
            }));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, InterpreterError> {
        let mut expr = self.term()?;

        while self.r#match(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ])? {
            let operator: Token = self.previous()?;
            let right: Expr = self.term()?;
            expr = Expr::Binary(Box::new(BinaryExpr {
                left: expr,
                operator,
                right,
            }));
        }

        Ok(expr)
    }

    fn previous(&self) -> Result<Token, InterpreterError> {
        match self.unwrap_tokens()?.get(self.crawled_index - 1) {
            Some(token) => Ok(token.clone()),
            None => Err(InterpreterError::InvalidTokenIndex),
        }
    }

    fn is_at_end(&self) -> Result<bool, InterpreterError> {
        Ok(self.peek()?.token_type == TokenType::EOF)
    }

    fn peek(&self) -> Result<Token, InterpreterError> {
        match self.unwrap_tokens()?.get(self.crawled_index) {
            Some(token) => Ok(token.clone()),
            None => Err(InterpreterError::InvalidTokenIndex),
        }
    }

    fn advance(&mut self) -> Result<Option<Token>, InterpreterError> {
        if !self.is_at_end()? {
            self.crawled_index += 1;
            Ok(None)
        } else {
            Ok(Some(self.previous()?))
        }
    }

    fn check(&self, token_type: &TokenType) -> Result<bool, InterpreterError> {
        if self.is_at_end()? {
            Ok(false)
        } else {
            Ok(self.peek()?.token_type == *token_type)
        }
    }

    fn r#match(&mut self, token_types: Vec<TokenType>) -> Result<bool, InterpreterError> {
        let mut result = false;

        for token_type in token_types {
            if self.check(&token_type)? {
                self.advance()?;
                result = true;
            }
        }

        Ok(result)
    }

    fn equality(&mut self) -> Result<Expr, InterpreterError> {
        let mut expr: Expr = self.comparison()?;

        while self.r#match(vec![TokenType::BangEqual, TokenType::EqualEqual])? {
            let operator: Token = self.previous()?;
            let right: Expr = self.comparison()?;
            expr = Expr::Binary(Box::new(BinaryExpr {
                left: expr,
                operator,
                right,
            }));
        }

        Ok(expr)
    }

    fn expression(&mut self) -> Result<Expr, InterpreterError> {
        Ok(self.equality()?)
    }

    fn clear_state(&mut self) {
        self.crawled_index = 0;
    }

    pub fn parse(&mut self, tokens: Vec<Token>) {
        self.clear_state();
        self.tokens = Some(tokens);
    }
}
