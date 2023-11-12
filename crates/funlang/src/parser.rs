use funlang_error::ErrorCascade;

use crate::{
    error,
    errors::ParserError,
    expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr},
    literal::LiteralData,
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

    fn unwrap_tokens(&self) -> Result<&Vec<Token>, ErrorCascade<ParserError>> {
        self.tokens
            .as_ref()
            .ok_or(error!(ParserError::MissingTokens))
    }

    fn _synchronize(&mut self) -> Result<(), ErrorCascade<ParserError>> {
        if self.previous()?.token_type == TokenType::Semicolon {
            return Ok(());
        }

        match self.peek()?.token_type {
            TokenType::Let => Ok(()),
            TokenType::For => Ok(()),
            TokenType::If => Ok(()),
            TokenType::While => Ok(()),
            TokenType::Print => Ok(()),
            TokenType::Return => Ok(()),
            _ => {
                self.advance()?;
                Ok(())
            }
        }
    }

    fn is_at_end(&self) -> Result<bool, ErrorCascade<ParserError>> {
        Ok(self.peek()?.token_type == TokenType::EOF)
    }

    fn consume(
        &mut self,
        token_type: TokenType,
        error: ErrorCascade<ParserError>,
    ) -> Result<(), ErrorCascade<ParserError>> {
        if self.check(&token_type)? {
            self.advance()?;
            Ok(())
        } else {
            Err(error)
        }
    }

    fn previous(&self) -> Result<Token, ErrorCascade<ParserError>> {
        match self.unwrap_tokens()?.get(self.crawled_index - 1) {
            Some(token) => Ok(token.clone()),
            None => Err(error!(ParserError::InvalidTokenIndex)),
        }
    }

    fn peek(&self) -> Result<Token, ErrorCascade<ParserError>> {
        match self.unwrap_tokens()?.get(self.crawled_index) {
            Some(token) => Ok(token.clone()),
            None => Err(error!(ParserError::InvalidTokenIndex)),
        }
    }

    fn advance(&mut self) -> Result<(), ErrorCascade<ParserError>> {
        if !self.is_at_end()? {
            self.crawled_index += 1;
        }
        Ok(())
    }

    fn check(&self, token_type: &TokenType) -> Result<bool, ErrorCascade<ParserError>> {
        if self.is_at_end()? {
            Ok(false)
        } else {
            Ok(self.peek()?.token_type == *token_type)
        }
    }

    fn r#match(&mut self, token_types: Vec<TokenType>) -> Result<bool, ErrorCascade<ParserError>> {
        let mut result = false;

        for token_type in token_types {
            if self.check(&token_type)? {
                self.advance()?;
                result = true;
            }
        }

        Ok(result)
    }

    fn primary(&mut self) -> Result<Expr, ErrorCascade<ParserError>> {
        if self.r#match(vec![TokenType::False])? {
            Ok(Expr::Literal(Box::new(LiteralExpr {
                literal: LiteralData::Bool(false),
            })))
        } else if self.r#match(vec![TokenType::True])? {
            Ok(Expr::Literal(Box::new(LiteralExpr {
                literal: LiteralData::Bool(true),
            })))
        } else if self.r#match(vec![TokenType::Null])? {
            Ok(Expr::Literal(Box::new(LiteralExpr {
                literal: LiteralData::Null,
            })))
        } else if self.r#match(vec![TokenType::Number, TokenType::String])? {
            let span = self.peek()?.span.ok_or(error!(ParserError::MissingSpan))?;
            Ok(Expr::Literal(Box::new(LiteralExpr {
                literal: self
                    .previous()?
                    .literal_data
                    .ok_or(error!(ParserError::InvalidLiteralData).set_span(span.into()))?,
            })))
        } else if self.r#match(vec![TokenType::LeftParen])? {
            let expr = self.expression()?;
            let span = self.peek()?.span.ok_or(error!(ParserError::MissingSpan))?;
            self.consume(
                TokenType::RightParen,
                error!(ParserError::UnterminatedGrouping).set_span(span.into()),
            )?;
            Ok(Expr::Grouping(Box::new(GroupingExpr { expression: expr })))
        } else {
            let span = self.peek()?.span.ok_or(error!(ParserError::MissingSpan))?;
            Err(error!(ParserError::UnexpectedExpression).set_span(span.into()))
        }
    }

    fn unary(&mut self) -> Result<Expr, ErrorCascade<ParserError>> {
        if self.r#match(vec![TokenType::Bang, TokenType::Minus])? {
            let operator = self.previous()?;
            let right = self.unary()?;
            Ok(Expr::Unary(Box::new(UnaryExpr { operator, right })))
        } else {
            Ok(self.primary()?)
        }
    }

    fn factor(&mut self) -> Result<Expr, ErrorCascade<ParserError>> {
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

    fn term(&mut self) -> Result<Expr, ErrorCascade<ParserError>> {
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

    fn comparison(&mut self) -> Result<Expr, ErrorCascade<ParserError>> {
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

    fn equality(&mut self) -> Result<Expr, ErrorCascade<ParserError>> {
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

    fn expression(&mut self) -> Result<Expr, ErrorCascade<ParserError>> {
        Ok(self.equality()?)
    }

    fn clear_state(&mut self) {
        self.crawled_index = 0;
    }

    pub fn parse(&mut self, tokens: Vec<Token>) -> Result<Expr, ErrorCascade<ParserError>> {
        self.clear_state();
        self.tokens = Some(tokens);

        Ok(self.expression()?)
    }
}

#[cfg(test)]
mod parser_tests {
    use crate::lexer::Lexer;

    use super::*;

    #[test]
    fn parses_expressions() {
        let mut lexer = Lexer::new();
        let lexer_result = lexer.tokenize("(1 + 1) / 6");
        assert!(lexer_result.is_ok());

        let mut parser = Parser::new();
        let parser_result = parser.parse(lexer_result.unwrap());
        assert!(parser_result.is_ok());

        println!("\n{:#?}", parser_result.unwrap());
    }
}
