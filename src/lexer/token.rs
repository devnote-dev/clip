use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    EOF,

    Assign,

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
            Token::Assign => write!(f, "assign"),
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
