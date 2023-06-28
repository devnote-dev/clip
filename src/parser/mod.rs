pub mod ast;
pub mod error;

use crate::lexer::token::Token;
use ast::Program;
use error::Error;

#[derive(Debug)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

pub trait Parse<'a>
where
    Self: Sized,
{
    fn parse(p: &mut Parser, prec: Option<Precedence>) -> Result<Self, Error>;
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Result<Program, Error> {
        Program::parse(self, None)
    }

    pub fn current_token(&self) -> Token {
        self.tokens[self.pos].clone()
    }

    pub fn next_token(&mut self) -> &Token {
        self.pos += 1;

        &self.tokens[self.pos]
    }

    pub fn peek_token(&self) -> Option<&Token> {
        if self.tokens.is_empty() {
            None
        } else {
            Some(&self.tokens[self.pos + 1])
        }
    }
}
