use super::{Parse, Parser};
use crate::{error::Error, lexer::token::Token};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Parse for Program {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        let mut statements = Vec::new();

        loop {
            match p.current_token() {
                Token::EOF => break,
                Token::Semicolon | Token::Newline => {
                    _ = p.next_token();
                }
                _ => {
                    statements.push(Statement::parse(p)?);
                    if p.current_token() == Token::EOF {
                        break;
                    }
                    _ = p.next_token();
                }
            }
        }

        Ok(Self { statements })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    Assign(Assign),
    Expression(Expression),
}

impl Parse for Statement {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        match p.current_token() {
            Token::Assign => Ok(Self::Assign(Assign::parse(p)?)),
            _ => Ok(Self::Expression(Expression::parse(p)?)),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Assign {
    pub name: Identifier,
    pub value: Expression,
}

impl Parse for Assign {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        _ = p.next_token();
        let name = Identifier::parse(p)?;
        _ = p.next_token();
        let value = Expression::parse(p)?;

        match p.peek_token() {
            Token::EOF | Token::Semicolon | Token::Newline => Ok(Self { name, value }),
            t => Err(Error::new(&format!("unexpected token {t}"))),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Primitive(Primitive),
    Identifier(Identifier),
    Operator(Operator),
    Function(Function),
    Call(Call),
    And(And),
    Or(Or),
}

impl Expression {
    fn parse_non_call(p: &mut Parser) -> Result<Self, Error> {
        match p.current_token() {
            Token::LeftParen => {
                if p.next_token() == &Token::RightParen {
                    return Ok(Self::Primitive(Primitive::Null));
                }

                let expr = Expression::parse(p)?;
                let t = p.peek_token();

                if t == &Token::RightParen {
                    _ = p.next_token();
                    Ok(expr)
                } else {
                    Err(Error::new(&format!("expected right paren; got {t}")))
                }
            }
            Token::And => Ok(Self::And(And::parse(p)?)),
            Token::Or => Ok(Self::Or(Or::parse(p)?)),
            Token::BlockStart => Ok(Self::Function(Function::parse(p)?)),
            Token::Integer(_) | Token::Float(_) | Token::String(_) | Token::True | Token::False => {
                Ok(Self::Primitive(Primitive::parse(p)?))
            }
            Token::Ident(_) => Ok(Self::Identifier(Identifier::parse(p)?)),
            Token::Equal
            | Token::Plus
            | Token::Minus
            | Token::Asterisk
            | Token::Slash
            | Token::Bang => Ok(Self::Operator(Operator::parse(p)?)),
            t => Err(Error::new(&format!("unexpected token {t}"))),
        }
    }
}

impl Parse for Expression {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        match p.current_token() {
            Token::LeftParen => {
                if p.next_token() == &Token::RightParen {
                    return Ok(Self::Primitive(Primitive::Null));
                }

                let expr = Expression::parse(p)?;
                let t = p.peek_token();

                if t == &Token::RightParen {
                    _ = p.next_token();
                    Ok(expr)
                } else {
                    Err(Error::new(&format!("expected right paren; got {t}")))
                }
            }
            Token::And => Ok(Self::And(And::parse(p)?)),
            Token::Or => Ok(Self::Or(Or::parse(p)?)),
            Token::BlockStart => Ok(Self::Function(Function::parse(p)?)),
            Token::Integer(_) | Token::Float(_) | Token::String(_) | Token::True | Token::False => {
                Ok(Self::Primitive(Primitive::parse(p)?))
            }
            Token::Ident(_) => match p.peek_token() {
                Token::EOF | Token::Semicolon | Token::Newline => {
                    Ok(Self::Identifier(Identifier::parse(p)?))
                }
                _ => Ok(Self::Call(Call::parse(p)?)),
            },
            Token::Equal
            | Token::Plus
            | Token::Minus
            | Token::Asterisk
            | Token::Slash
            | Token::Bang => Ok(Self::Operator(Operator::parse(p)?)),
            t => Err(Error::new(&format!("unexpected token {t}"))),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Primitive {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
}

impl Parse for Primitive {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
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

impl Display for Primitive {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match *self {
            Primitive::Integer(_) => write!(f, "integer"),
            Primitive::Float(_) => write!(f, "float"),
            Primitive::String(_) => write!(f, "string"),
            Primitive::Boolean(_) => write!(f, "boolean"),
            Primitive::Null => write!(f, "null"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Identifier {
    pub value: String,
}

impl Parse for Identifier {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        match p.current_token() {
            Token::Ident(value) => Ok(Self { value }),
            t => Err(Error::new(&format!("unexpected token {t}"))),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Operator {
    pub kind: OperatorKind,
    pub args: Vec<Expression>,
}

impl Parse for Operator {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        let kind = match p.current_token() {
            Token::Equal => OperatorKind::Equal,
            Token::Plus => OperatorKind::Add,
            Token::Minus => OperatorKind::Subtract,
            Token::Asterisk => OperatorKind::Multiply,
            Token::Slash => OperatorKind::Divide,
            Token::Bang => OperatorKind::Inverse,
            _ => unreachable!(),
        };

        let mut args = Vec::new();

        loop {
            match p.peek_token() {
                Token::EOF | Token::Semicolon | Token::Newline | Token::RightParen => break,
                _ => {
                    _ = p.next_token();
                    match Expression::parse_non_call(p) {
                        Ok(expr) => args.push(expr),
                        Err(_) => break,
                    }
                }
            }
        }

        Ok(Self { kind, args })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum OperatorKind {
    Equal,
    Add,
    Subtract,
    Multiply,
    Divide,
    Inverse,
}

impl Display for OperatorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match *self {
            OperatorKind::Equal => write!(f, "equal"),
            OperatorKind::Add => write!(f, "add"),
            OperatorKind::Subtract => write!(f, "subtract"),
            OperatorKind::Multiply => write!(f, "multiply"),
            OperatorKind::Divide => write!(f, "divide"),
            OperatorKind::Inverse => write!(f, "inverse"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub params: Vec<Identifier>,
    pub body: Vec<Statement>,
}

impl Parse for Function {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        let mut params = Vec::new();

        if p.next_token() == &Token::LeftBracket {
            match p.next_token() {
                Token::EOF => return Err(Error::new("unexpected end of file")),
                Token::RightBracket => _ = p.next_token(),
                _ => {
                    params.push(Identifier::parse(p)?);
                    loop {
                        match p.next_token() {
                            Token::EOF => return Err(Error::new("unexpected end of file")),
                            Token::RightBracket => {
                                _ = p.next_token();
                                break;
                            }
                            _ => params.push(Identifier::parse(p)?),
                        }
                    }
                }
            }
        }

        let mut body = Vec::new();

        loop {
            match p.current_token() {
                Token::EOF => return Err(Error::new("unexpected end of file")),
                Token::Semicolon | Token::Newline => _ = p.next_token(),
                Token::BlockEnd => {
                    _ = p.next_token();
                    break;
                }
                _ => {
                    body.push(Statement::parse(p)?);
                    if p.current_token() == Token::BlockEnd {
                        break;
                    }
                    _ = p.next_token();
                }
            }
        }

        Ok(Self { params, body })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Call {
    pub name: Identifier,
    pub args: Vec<Expression>,
}

impl Parse for Call {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        let name = Identifier::parse(p)?;
        let mut args = Vec::new();

        loop {
            match p.peek_token() {
                Token::EOF | Token::Semicolon | Token::Newline | Token::RightParen => break,
                _ => {
                    _ = p.next_token();
                    args.push(Expression::parse(p)?);
                }
            }
        }

        Ok(Self { name, args })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct And(pub Vec<Expression>);

impl Parse for And {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        let mut args = Vec::new();

        loop {
            match p.peek_token() {
                Token::EOF | Token::Semicolon | Token::Newline | Token::RightParen => break,
                _ => {
                    _ = p.next_token();
                    args.push(Expression::parse(p)?);
                }
            }
        }

        Ok(Self(args))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Or(pub Vec<Expression>);

impl Parse for Or {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        let mut args = Vec::new();

        loop {
            match p.peek_token() {
                Token::EOF | Token::Semicolon | Token::Newline | Token::RightParen => break,
                _ => {
                    _ = p.next_token();
                    args.push(Expression::parse(p)?);
                }
            }
        }

        Ok(Self(args))
    }
}
