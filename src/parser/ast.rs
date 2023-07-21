use super::{Parse, Parser};
use crate::{error::Error, lexer::token::TokenValue};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Parse for Program {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        let mut statements = Vec::new();

        loop {
            match p.current_token().value {
                TokenValue::EOF => break,
                TokenValue::Semicolon | TokenValue::Newline => {
                    _ = p.next_token();
                }
                _ => {
                    statements.push(Statement::parse(p)?);
                    if p.current_token().value == TokenValue::EOF {
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
    If(If),
    Expression(Expression),
}

impl Parse for Statement {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        match p.current_token().value {
            TokenValue::Assign => Ok(Self::Assign(Assign::parse(p)?)),
            TokenValue::If => Ok(Self::If(If::parse(p)?)),
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

        match &p.peek_token().value {
            TokenValue::EOF | TokenValue::Semicolon | TokenValue::Newline => {
                Ok(Self { name, value })
            }
            t => Err(Error::new(&format!("unexpected token {t}"))),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct If {
    pub condition: Expression,
    pub consequence: Vec<Box<Statement>>,
    pub alternative: Option<Vec<Box<Statement>>>,
}

impl Parse for If {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        _ = p.next_token();
        let condition = Expression::parse(p)?;

        if p.next_token().value != TokenValue::BlockStart {
            return Err(Error::new(&format!(
                "expected block start; got {}",
                p.current_token().value
            )));
        }

        let mut consequence = Vec::new();

        loop {
            match p.peek_token().value {
                TokenValue::EOF => return Err(Error::new("unexpected end of file")),
                TokenValue::Semicolon | TokenValue::Newline => _ = p.next_token(),
                TokenValue::BlockEnd => {
                    _ = p.next_token();
                    break;
                }
                _ => {
                    _ = p.next_token();
                    let stmt = Statement::parse(p)?;
                    consequence.push(Box::new(stmt));
                }
            }
        }

        let mut alternative = None;

        while p.peek_token().value == TokenValue::Semicolon
            || p.peek_token().value == TokenValue::Newline
        {
            _ = p.next_token();
        }

        match p.peek_token().value {
            TokenValue::BlockEnd => _ = p.next_token(),
            TokenValue::Else => {
                _ = p.next_token();
                if p.next_token().value != TokenValue::BlockStart {
                    return Err(Error::new(&format!(
                        "expected block start; got {}",
                        p.current_token().value
                    )));
                }

                let mut statements = Vec::new();

                loop {
                    match p.peek_token().value {
                        TokenValue::EOF => return Err(Error::new("unexpected end of file")),
                        TokenValue::Semicolon | TokenValue::Newline => _ = p.next_token(),
                        TokenValue::BlockEnd => {
                            _ = p.next_token();
                            _ = p.next_token();
                            break;
                        }
                        _ => {
                            _ = p.next_token();
                            let stmt = Statement::parse(p)?;
                            statements.push(Box::new(stmt));
                        }
                    }
                }

                alternative = Some(statements);
            }
            _ => {
                return Err(Error::new(&format!(
                    "expected block end or else statement; got {}",
                    p.peek_token().value
                )))
            }
        }

        Ok(Self {
            condition,
            consequence,
            alternative,
        })
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
        match p.current_token().value {
            TokenValue::LeftParen => {
                if p.next_token().value == TokenValue::RightParen {
                    return Ok(Self::Primitive(Primitive::Null));
                }

                let expr = Expression::parse(p)?;
                let t = &p.peek_token().value;

                if t == &TokenValue::RightParen {
                    _ = p.next_token();
                    Ok(expr)
                } else {
                    Err(Error::new(&format!("expected right paren; got {t}")))
                }
            }
            TokenValue::And => Ok(Self::And(And::parse(p)?)),
            TokenValue::Or => Ok(Self::Or(Or::parse(p)?)),
            TokenValue::BlockStart => Ok(Self::Function(Function::parse(p)?)),
            TokenValue::Integer(_)
            | TokenValue::Float(_)
            | TokenValue::String(_)
            | TokenValue::True
            | TokenValue::False => Ok(Self::Primitive(Primitive::parse(p)?)),
            TokenValue::Ident(_) => Ok(Self::Identifier(Identifier::parse(p)?)),
            TokenValue::Equal
            | TokenValue::Plus
            | TokenValue::Minus
            | TokenValue::Asterisk
            | TokenValue::Slash
            | TokenValue::Bang => Ok(Self::Operator(Operator::parse(p)?)),
            t => Err(Error::new(&format!("unexpected token {t}"))),
        }
    }
}

impl Parse for Expression {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        match p.current_token().value {
            TokenValue::LeftParen => {
                if p.next_token().value == TokenValue::RightParen {
                    return Ok(Self::Primitive(Primitive::Null));
                }

                let expr = Expression::parse(p)?;
                let t = &p.peek_token().value;

                if t == &TokenValue::RightParen {
                    _ = p.next_token();
                    Ok(expr)
                } else {
                    Err(Error::new(&format!("expected right paren; got {t}")))
                }
            }
            TokenValue::And => Ok(Self::And(And::parse(p)?)),
            TokenValue::Or => Ok(Self::Or(Or::parse(p)?)),
            TokenValue::BlockStart => Ok(Self::Function(Function::parse(p)?)),
            TokenValue::Integer(_)
            | TokenValue::Float(_)
            | TokenValue::String(_)
            | TokenValue::True
            | TokenValue::False => Ok(Self::Primitive(Primitive::parse(p)?)),
            TokenValue::Ident(_) => match p.peek_token().value {
                TokenValue::EOF | TokenValue::Semicolon | TokenValue::Newline => {
                    Ok(Self::Identifier(Identifier::parse(p)?))
                }
                _ => Ok(Self::Call(Call::parse(p)?)),
            },
            TokenValue::Equal
            | TokenValue::Plus
            | TokenValue::Minus
            | TokenValue::Asterisk
            | TokenValue::Slash
            | TokenValue::Bang => Ok(Self::Operator(Operator::parse(p)?)),
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
        Ok(match p.current_token().value {
            TokenValue::Integer(v) => Self::Integer(v.parse()?),
            TokenValue::Float(v) => Self::Float(v.parse()?),
            TokenValue::String(v) => Self::String(v),
            TokenValue::True => Self::Boolean(true),
            TokenValue::False => Self::Boolean(false),
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
        match p.current_token().value {
            TokenValue::Ident(value) => Ok(Self { value }),
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
        let kind = match p.current_token().value {
            TokenValue::Equal => OperatorKind::Equal,
            TokenValue::Plus => OperatorKind::Add,
            TokenValue::Minus => OperatorKind::Subtract,
            TokenValue::Asterisk => OperatorKind::Multiply,
            TokenValue::Slash => OperatorKind::Divide,
            TokenValue::Bang => OperatorKind::Inverse,
            _ => unreachable!(),
        };

        let mut args = Vec::new();

        loop {
            match p.peek_token().value {
                TokenValue::EOF
                | TokenValue::Semicolon
                | TokenValue::Newline
                | TokenValue::RightParen
                | TokenValue::BlockStart => break,
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

        if p.next_token().value == TokenValue::LeftBracket {
            match p.next_token().value {
                TokenValue::EOF => return Err(Error::new("unexpected end of file")),
                TokenValue::RightBracket => _ = p.next_token(),
                _ => {
                    params.push(Identifier::parse(p)?);
                    loop {
                        match p.next_token().value {
                            TokenValue::EOF => return Err(Error::new("unexpected end of file")),
                            TokenValue::RightBracket => {
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
            match p.current_token().value {
                TokenValue::EOF => return Err(Error::new("unexpected end of file")),
                TokenValue::Semicolon | TokenValue::Newline => _ = p.next_token(),
                TokenValue::BlockEnd => {
                    _ = p.next_token();
                    break;
                }
                _ => {
                    body.push(Statement::parse(p)?);
                    if p.current_token().value == TokenValue::BlockEnd {
                        _ = p.next_token();
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
            match p.peek_token().value {
                TokenValue::EOF
                | TokenValue::Semicolon
                | TokenValue::Newline
                | TokenValue::RightParen => break,
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
            match p.peek_token().value {
                TokenValue::EOF
                | TokenValue::Semicolon
                | TokenValue::Newline
                | TokenValue::RightParen
                | TokenValue::BlockStart => break,
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
            match p.peek_token().value {
                TokenValue::EOF
                | TokenValue::Semicolon
                | TokenValue::Newline
                | TokenValue::RightParen
                | TokenValue::BlockStart => break,
                _ => {
                    _ = p.next_token();
                    args.push(Expression::parse(p)?);
                }
            }
        }

        Ok(Self(args))
    }
}
