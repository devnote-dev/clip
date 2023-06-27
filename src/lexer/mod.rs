use std::{iter::Peekable, str::Chars};

use self::token::Token;

pub mod token;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut res = Vec::new();

        loop {
            match self.input.peek() {
                Some(&c) => match c {
                    ' ' | '\t' | '\r' | '\n' => {
                        _ = self.input.next();
                    }
                    '(' => {
                        res.push(Token::LeftParen);
                        _ = self.input.next();
                    }
                    ')' => {
                        res.push(Token::RightParen);
                        _ = self.input.next();
                    }
                    '=' => {
                        _ = self.input.next();
                        match self.input.peek() {
                            Some(&c) => {
                                if c == '=' {
                                    res.push(Token::Equal);
                                    _ = self.input.next();
                                } else {
                                    res.push(Token::Assign);
                                }
                            }
                            None => {
                                res.push(Token::Assign);
                            }
                        }
                    }
                    '+' => {
                        res.push(Token::Plus);
                        _ = self.input.next();
                    }
                    '-' => {
                        res.push(Token::Minus);
                        _ = self.input.next();
                    }
                    '*' => {
                        res.push(Token::Asterisk);
                        _ = self.input.next();
                    }
                    '/' => {
                        res.push(Token::Slash);
                        _ = self.input.next();
                    }
                    '!' => {
                        res.push(Token::Bang);
                        _ = self.input.next();
                    }
                    '0'..='9' => res.push(self.lex_int_or_float()),
                    '"' => res.push(self.lex_string()),
                    'a'..='z' | 'A'..='Z' | '_' => res.push(self.lex_ident()),
                    _ => {
                        res.push(Token::Illegal(format!("unexpected: {c}")));
                        _ = self.input.next();
                    }
                },
                None => {
                    res.push(Token::EOF);
                    break;
                }
            }
        }

        res
    }

    fn lex_int_or_float(&mut self) -> Token {
        let mut value = String::new();
        let mut float = false;

        while let Some(&c) = self.input.peek() {
            match c {
                '0'..='9' => {
                    value.push(c);
                    _ = self.input.next();
                }
                '_' => continue,
                '.' => {
                    if float {
                        _ = self.input.next();
                        return Token::Illegal(format!("unexpected: {c}"));
                    }
                    float = true;
                    value.push('.');
                    _ = self.input.next();
                }
                _ => break,
            }
        }

        if float {
            Token::Float(value)
        } else {
            Token::Integer(value)
        }
    }

    fn lex_string(&mut self) -> Token {
        let mut string = String::new();
        let mut escaped = false;
        _ = self.input.next();

        loop {
            match self.input.peek() {
                Some(&c) => match c {
                    '\\' => escaped = !escaped,
                    '"' => {
                        if escaped {
                            escaped = false;
                            continue;
                        }
                        _ = self.input.next();
                        break Token::String(string);
                    }
                    _ => {
                        string.push(c);
                        _ = self.input.next();
                    }
                },
                None => {
                    break Token::Illegal("unterminated quote string".to_string());
                }
            }
        }
    }

    fn lex_ident(&mut self) -> Token {
        let mut ident = String::new();

        while let Some(&c) = self.input.peek() {
            match c {
                'a'..='z' | 'A'..='Z' | '_' => {
                    ident.push(c);
                    _ = self.input.next();
                }
                _ => break,
            }
        }

        match ident.as_str() {
            "true" => Token::True,
            "false" => Token::False,
            _ => Token::Ident(ident),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Lexer, Token};

    #[test]
    fn test_empty_input() {
        let tokens = Lexer::new("").lex();
        assert_eq!(tokens, [Token::EOF]);
    }

    #[test]
    fn test_parentheses() {
        let tokens = Lexer::new("(()()()())").lex();
        assert_eq!(
            tokens,
            [
                Token::LeftParen,
                Token::LeftParen,
                Token::RightParen,
                Token::LeftParen,
                Token::RightParen,
                Token::LeftParen,
                Token::RightParen,
                Token::LeftParen,
                Token::RightParen,
                Token::RightParen,
                Token::EOF
            ]
        );
    }

    #[test]
    fn test_integers() {
        let tokens = Lexer::new("123456").lex();
        assert_eq!(tokens, [Token::Integer("123456".to_string()), Token::EOF]);
    }

    #[test]
    fn test_floats() {
        let tokens = Lexer::new("3.14159265").lex();
        assert_eq!(tokens, [Token::Float("3.14159265".to_string()), Token::EOF]);
    }

    #[test]
    fn test_strings() {
        let tokens = Lexer::new("\"foo bar baz\"").lex();
        assert_eq!(
            tokens,
            [Token::String("foo bar baz".to_string()), Token::EOF]
        );
    }

    #[test]
    fn test_multi_line_strings() {
        let tokens = Lexer::new(
            "
\"
foo
bar
baz
\"",
        )
        .lex();

        assert_eq!(
            tokens,
            [Token::String("\nfoo\nbar\nbaz\n".to_string()), Token::EOF]
        );
    }

    #[test]
    fn test_boolean_true() {
        let tokens = Lexer::new("true").lex();
        assert_eq!(tokens, [Token::True, Token::EOF]);
    }

    #[test]
    fn test_boolean_false() {
        let tokens = Lexer::new("false").lex();
        assert_eq!(tokens, [Token::False, Token::EOF]);
    }

    #[test]
    fn test_identifiers() {
        let tokens = Lexer::new("foo bar").lex();
        assert_eq!(
            tokens,
            [
                Token::Ident("foo".to_string()),
                Token::Ident("bar".to_string()),
                Token::EOF
            ]
        );
    }

    #[test]
    fn test_assignment() {
        let tokens = Lexer::new("= foo 123").lex();
        assert_eq!(
            tokens,
            [
                Token::Assign,
                Token::Ident("foo".to_string()),
                Token::Integer("123".to_string()),
                Token::EOF
            ]
        );
    }

    #[test]
    fn test_operators() {
        let tokens = Lexer::new("===+-*/!").lex();
        assert_eq!(
            tokens,
            [
                Token::Equal,
                Token::Assign,
                Token::Plus,
                Token::Minus,
                Token::Asterisk,
                Token::Slash,
                Token::Bang,
                Token::EOF
            ]
        );
    }
}
