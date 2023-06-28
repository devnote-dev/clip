use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    EOF,
    LeftParen,
    RightParen,

    Assign,
    Equal,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Bang,

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
        write!(f, "Token(")?;
        match self {
            Token::EOF => write!(f, "eof"),
            Token::LeftParen => write!(f, "left paren"),
            Token::RightParen => write!(f, "right paren"),
            Token::Assign => write!(f, "assign"),
            Token::Equal => write!(f, "equal"),
            Token::Plus => write!(f, "plus"),
            Token::Minus => write!(f, "minus"),
            Token::Asterisk => write!(f, "asterisk"),
            Token::Slash => write!(f, "slash"),
            Token::Bang => write!(f, "bang"),
            Token::Integer(v) => write!(f, "integer: {}", v),
            Token::Float(v) => write!(f, "float: {}", v),
            Token::String(v) => write!(f, "string: {}", v),
            Token::True => write!(f, "boolean: true"),
            Token::False => write!(f, "boolean: false"),
            Token::Ident(v) => write!(f, "ident: {}", v),
            Token::Illegal(v) => write!(f, "illegal: {}", v),
        }?;
        write!(f, ")")
    }
}
