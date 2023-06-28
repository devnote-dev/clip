use super::{error::Error, Parse, Parser, Precedence};
use crate::lexer::token::Token;

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl<'a> Parse<'a> for Program {
    fn parse(p: &mut Parser, _: Option<Precedence>) -> Result<Self, Error> {
        let mut statements = Vec::new();

        while p.current_token() != Token::EOF {
            statements.push(Statement::parse(p, None)?);
            _ = p.next_token();
        }

        Ok(Self { statements })
    }
}

#[derive(Debug)]
pub enum Statement {
    Assign(Assign),
    Expression(Expression),
}

impl<'a> Parse<'a> for Statement {
    fn parse(p: &mut Parser, _: Option<Precedence>) -> Result<Self, Error> {
        match p.current_token() {
            Token::Assign => Ok(Self::Assign(Assign::parse(p, None)?)),
            _ => {
                let expr = Expression::parse(p, Some(Precedence::Lowest))?;
                // TODO: check if newline or semicolon

                Ok(Self::Expression(expr))
            }
        }
    }
}

#[derive(Debug)]
pub struct Assign {
    pub name: Identifier,
    pub value: Expression,
}

impl<'a> Parse<'a> for Assign {
    fn parse(p: &mut Parser, _: Option<Precedence>) -> Result<Self, Error> {
        _ = p.next_token();
        let name = Identifier::parse(p, Some(Precedence::Lowest))?;
        _ = p.next_token();
        let value = Expression::parse(p, Some(Precedence::Lowest))?;

        Ok(Self { name, value })
    }
}

#[derive(Debug)]
pub enum Expression {
    Primitive(Primitive),
    Identifier(Identifier),
    Operator(Operator),
}

impl<'a> Parse<'a> for Expression {
    fn parse(p: &mut Parser, _: Option<Precedence>) -> Result<Self, Error> {
        match p.current_token() {
            Token::Integer(_) | Token::Float(_) | Token::String(_) | Token::True | Token::False => {
                Ok(Self::Primitive(Primitive::parse(p, None)?))
            }
            Token::Ident(_) => Ok(Self::Identifier(Identifier::parse(p, None)?)),
            Token::Equal
            | Token::Plus
            | Token::Minus
            | Token::Asterisk
            | Token::Slash
            | Token::Bang => Ok(Self::Operator(Operator::parse(p, None)?)),
            _ => Err(Error::new("unexpected thing")),
        }
    }
}

#[derive(Debug)]
pub enum Primitive {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
}

impl<'a> Parse<'a> for Primitive {
    fn parse(p: &mut Parser, _: Option<Precedence>) -> Result<Self, Error> {
        Ok(match p.current_token() {
            Token::Integer(v) => Self::Integer(v.parse()?),
            Token::Float(v) => Self::Float(v.parse()?),
            Token::String(v) => Self::String(v),
            Token::True => Self::Boolean(true),
            Token::False => Self::Boolean(false),
            _ => unreachable!(),
        })
    }
}

#[derive(Debug)]
pub struct Identifier {
    pub value: String,
}

impl<'a> Parse<'a> for Identifier {
    fn parse(p: &mut Parser, _: Option<Precedence>) -> Result<Self, Error> {
        match p.current_token() {
            Token::Ident(value) => Ok(Self { value }),
            t => Err(Error::new(&format!("unexpected {t}"))),
        }
    }
}

#[derive(Debug)]
pub struct Operator {
    pub kind: OperatorKind,
    pub rest: Box<Expression>,
}

impl<'a> Parse<'a> for Operator {
    fn parse(p: &mut Parser, _: Option<Precedence>) -> Result<Self, Error> {
        let kind = match p.current_token() {
            Token::Equal => OperatorKind::Equal,
            Token::Plus => OperatorKind::Add,
            Token::Minus => OperatorKind::Subtract,
            Token::Asterisk => OperatorKind::Multiply,
            Token::Slash => OperatorKind::Divide,
            Token::Bang => OperatorKind::Bang,
            _ => unreachable!(),
        };

        _ = p.next_token();
        let rest = Box::new(Expression::parse(p, Some(Precedence::Lowest))?);

        Ok(Self { kind, rest })
    }
}

#[derive(Debug)]
pub enum OperatorKind {
    Equal,
    Add,
    Subtract,
    Multiply,
    Divide,
    Bang,
}
