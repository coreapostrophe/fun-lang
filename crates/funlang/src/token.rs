use std::fmt::Display;

use crate::literal::LiteralData;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Literals
    Identifier(String),
    String,
    Number,

    // One or two character tokens,
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // KEYWORDS
    And,
    Or,
    Else,
    False,
    For,
    If,
    Null,
    Print,
    Return,
    True,
    Let,
    While,
    This,

    // Single-character
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    EOF,
}

impl TokenType {
    pub fn get_keyword(text: &str) -> Option<TokenType> {
        match text {
            "and" => Some(TokenType::And),
            "or" => Some(TokenType::Or),
            "else" => Some(TokenType::Else),
            "true" => Some(TokenType::True),
            "false" => Some(TokenType::False),
            "for" => Some(TokenType::For),
            "if" => Some(TokenType::If),
            "null" => Some(TokenType::Null),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "let" => Some(TokenType::Let),
            "while" => Some(TokenType::While),
            "this" => Some(TokenType::This),
            _ => None,
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct Span {
    pub line: u32,
    pub col: u32,
}

impl Span {
    pub fn new(line: u32, col: u32) -> Self {
        Self { line, col }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: Option<String>,
    pub literal_data: Option<LiteralData>,
    pub span: Option<Span>,
}

impl Token {
    pub fn new(token_type: TokenType) -> Self {
        Self {
            token_type,
            span: None,
            lexeme: None,
            literal_data: None,
        }
    }
    pub fn set_literal_data(mut self, value: LiteralData) -> Self {
        self.literal_data = Some(value);
        self
    }
    pub fn set_span(mut self, value: Span) -> Self {
        self.span = Some(value);
        self
    }
    pub fn set_lexeme(mut self, value: String) -> Self {
        self.lexeme = Some(value);
        self
    }
}
