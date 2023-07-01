use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Debug)]
pub struct Token {
    pub value: TokenValue,
    pub loc: Location,
}

impl Token {
    pub const fn new(value: TokenValue, loc: Location) -> Self {
        Self { value, loc }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Token(")?;
        self.value.fmt(f)?;

        write!(f, ", ")?;
        self.loc.fmt(f)?;

        write!(f, ")")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TokenValue {
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

impl Display for TokenValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            TokenValue::EOF => write!(f, "eof"),
            TokenValue::Semicolon => write!(f, "semicolon"),
            TokenValue::Newline => write!(f, "newline"),
            TokenValue::LeftParen => write!(f, "left paren"),
            TokenValue::RightParen => write!(f, "right paren"),
            TokenValue::LeftBracket => write!(f, "left bracket"),
            TokenValue::RightBracket => write!(f, "right bracket"),
            TokenValue::If => write!(f, "if"),
            TokenValue::Elif => write!(f, "elif"),
            TokenValue::Else => write!(f, "else"),
            TokenValue::Assign => write!(f, "assign"),
            TokenValue::Equal => write!(f, "equal"),
            TokenValue::Plus => write!(f, "plus"),
            TokenValue::Minus => write!(f, "minus"),
            TokenValue::Asterisk => write!(f, "asterisk"),
            TokenValue::Slash => write!(f, "slash"),
            TokenValue::Bang => write!(f, "bang"),
            TokenValue::And => write!(f, "and"),
            TokenValue::Or => write!(f, "or"),
            TokenValue::BlockStart => write!(f, "block start"),
            TokenValue::BlockEnd => write!(f, "block end"),
            TokenValue::Integer(v) => write!(f, "integer: {}", v),
            TokenValue::Float(v) => write!(f, "float: {}", v),
            TokenValue::String(v) => write!(f, "string: {}", v),
            TokenValue::True => write!(f, "boolean: true"),
            TokenValue::False => write!(f, "boolean: false"),
            TokenValue::Ident(v) => write!(f, "ident: {}", v),
            TokenValue::Illegal(v) => write!(f, "illegal: {}", v),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Location {
    pub line_start: i32,
    pub line_stop: i32,
    pub col_start: i32,
    pub col_stop: i32,
}

impl Location {
    pub const fn new(line_start: i32, col_start: i32) -> Self {
        Self {
            line_start,
            line_stop: 0,
            col_start,
            col_stop: 0,
        }
    }

    pub fn stop(&self, line_stop: i32, col_stop: i32) -> Self {
        Self {
            line_start: self.line_start,
            line_stop,
            col_start: self.col_start,
            col_stop,
        }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}:{}, {}:{}",
            self.line_start, self.line_stop, self.col_start, self.col_stop
        )
    }
}
