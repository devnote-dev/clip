use super::{error::Error, Parse, Parser};
use crate::lexer::token::Token;

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl<'a> Parse<'a> for Program {
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

#[derive(Debug)]
pub enum Statement {
    Assign(Assign),
    Expression(Expression),
}

impl<'a> Parse<'a> for Statement {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        match p.current_token() {
            Token::Assign => Ok(Self::Assign(Assign::parse(p)?)),
            _ => Ok(Self::Expression(Expression::parse(p)?)),
        }
    }
}

#[derive(Debug)]
pub struct Assign {
    pub name: Identifier,
    pub value: Expression,
}

impl<'a> Parse<'a> for Assign {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        _ = p.next_token();
        let name = Identifier::parse(p)?;
        _ = p.next_token();
        let value = Expression::parse(p)?;

        match p.next_token() {
            Token::EOF | Token::Semicolon | Token::Newline => Ok(Self { name, value }),
            t => Err(Error::new(&format!("unexpected {t}"))),
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Primitive(Primitive),
    Identifier(Identifier),
    Operator(Operator),
    Function(Function),
}

impl<'a> Parse<'a> for Expression {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        match p.current_token() {
            Token::LeftParen => {
                _ = p.next_token();
                let expr = Expression::parse(p)?;
                let t = p.peek_token();

                if t == &Token::RightParen {
                    _ = p.next_token();
                    Ok(expr)
                } else {
                    Err(Error::new(&format!("expected right paren; got {t}")))
                }
            }
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
            t => Err(Error::new(&format!("unexpected {t}"))),
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

#[derive(Debug)]
pub struct Identifier {
    pub value: String,
}

impl<'a> Parse<'a> for Identifier {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        match p.current_token() {
            Token::Ident(value) => Ok(Self { value }),
            t => Err(Error::new(&format!("unexpected {t}"))),
        }
    }
}

#[derive(Debug)]
pub struct Operator {
    pub kind: OperatorKind,
    pub args: Vec<Expression>,
}

impl<'a> Parse<'a> for Operator {
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
            match p.next_token() {
                Token::EOF | Token::Semicolon | Token::Newline => break,
                Token::RightParen => {
                    p.back_token();
                    break;
                }
                _ => match Expression::parse(p) {
                    Ok(expr) => args.push(expr),
                    Err(_) => break,
                },
            }
        }

        Ok(Self { kind, args })
    }
}

#[derive(Debug)]
pub enum OperatorKind {
    Equal,
    Add,
    Subtract,
    Multiply,
    Divide,
    Inverse,
}

#[derive(Debug)]
pub struct Function {
    pub params: Vec<Identifier>,
    pub body: Vec<Statement>,
}

impl<'a> Parse<'a> for Function {
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
