use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    EOF,
    Semicolon,
    Newline,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    BlockStart,
    BlockEnd,

    If,
    Elif,
    Else,

    Assign,
    Equal,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Bang,
    And,
    Or,

    Integer(String),
    Float(String),
    String(String),
    True,
    False,
    Ident(String),
    Illegal(String),
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Token::EOF => write!(f, "eof"),
            Token::Semicolon => write!(f, "semicolon"),
            Token::Newline => write!(f, "newline"),
            Token::LeftParen => write!(f, "left paren"),
            Token::RightParen => write!(f, "right paren"),
            Token::LeftBracket => write!(f, "left bracket"),
            Token::RightBracket => write!(f, "right bracket"),
            Token::If => write!(f, "if"),
            Token::Elif => write!(f, "elif"),
            Token::Else => write!(f, "else"),
            Token::Assign => write!(f, "assign"),
            Token::Equal => write!(f, "equal"),
            Token::Plus => write!(f, "plus"),
            Token::Minus => write!(f, "minus"),
            Token::Asterisk => write!(f, "asterisk"),
            Token::Slash => write!(f, "slash"),
            Token::Bang => write!(f, "bang"),
            Token::And => write!(f, "and"),
            Token::Or => write!(f, "or"),
            Token::BlockStart => write!(f, "block start"),
            Token::BlockEnd => write!(f, "block end"),
            Token::Integer(v) => write!(f, "integer: {}", v),
            Token::Float(v) => write!(f, "float: {}", v),
            Token::String(v) => write!(f, "string: {}", v),
            Token::True => write!(f, "boolean: true"),
            Token::False => write!(f, "boolean: false"),
            Token::Ident(v) => write!(f, "ident: {}", v),
            Token::Illegal(v) => write!(f, "illegal: {}", v),
        }
    }
}
