use std::fmt::Display;

#[derive(Debug)]
pub enum LiteralData {
    Identifier(String),
    String(String),
    Number(f32),
}

#[derive(Debug)]
pub enum TokenType {
    // Literals
    Literal(LiteralData),

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

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: Option<String>,
    pub line: Option<u32>,
}

impl Token {
    pub fn new(token_type: TokenType) -> Self {
        Self {
            token_type,
            lexeme: None,
            line: None,
        }
    }
    pub fn set_line(mut self, value: u32) -> Self {
        self.line = Some(value);
        self
    }
    pub fn set_lexeme(mut self, value: String) -> Self {
        self.lexeme = Some(value);
        self
    }
}
