pub mod ast;

use crate::{error::Error, lexer::token::Token};
use ast::Program;

pub trait Parse
where
    Self: Sized,
{
    fn parse(p: &mut Parser) -> Result<Self, Error>;
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
        Program::parse(self)
    }

    pub fn current_token(&self) -> Token {
        self.tokens[self.pos].clone()
    }

    pub fn next_token(&mut self) -> &Token {
        self.pos += 1;

        &self.tokens[self.pos]
    }

    pub fn peek_token(&self) -> &Token {
        if self.pos + 1 >= self.tokens.len() {
            &Token::EOF
        } else {
            &self.tokens[self.pos + 1]
        }
    }

    pub fn back_token(&mut self) {
        self.pos -= 1;
    }
}
