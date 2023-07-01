use std::{iter::Peekable, str::Chars};

use self::token::{Location, Token, TokenValue};

pub mod token;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    line: i32,
    col: i32,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
            line: 0,
            col: 0,
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut res = Vec::new();

        loop {
            let loc = Location::new(0, 0);

            match self.input.peek() {
                Some(&c) => match c {
                    ' ' | '\t' => {
                        _ = self.input.next();
                    }
                    '\r' => {
                        if let Some(c) = self.input.next() {
                            if c == '\n' {
                                res.push(Token::new(
                                    TokenValue::Newline,
                                    loc.stop(self.line, self.col),
                                ));
                                _ = self.input.next();
                            }
                        }
                    }
                    '\n' => {
                        res.push(Token::new(
                            TokenValue::Newline,
                            loc.stop(self.line, self.col),
                        ));
                        _ = self.input.next();
                    }
                    ';' => {
                        res.push(Token::new(
                            TokenValue::Semicolon,
                            loc.stop(self.line, self.col),
                        ));
                        _ = self.input.next();
                    }
                    '#' => loop {
                        match self.input.next() {
                            Some(c) => {
                                if c == '\n' {
                                    break;
                                }
                            }
                            None => {
                                res.push(Token::new(
                                    TokenValue::EOF,
                                    loc.stop(self.line, self.col),
                                ));
                                break;
                            }
                        }
                    },
                    '(' => {
                        res.push(Token::new(
                            TokenValue::LeftParen,
                            loc.stop(self.line, self.col),
                        ));
                        _ = self.input.next();
                    }
                    ')' => {
                        res.push(Token::new(
                            TokenValue::RightParen,
                            loc.stop(self.line, self.col),
                        ));
                        _ = self.input.next();
                    }
                    '[' => {
                        res.push(Token::new(
                            TokenValue::LeftBracket,
                            loc.stop(self.line, self.col),
                        ));
                        _ = self.input.next();
                    }
                    ']' => {
                        res.push(Token::new(
                            TokenValue::RightBracket,
                            loc.stop(self.line, self.col),
                        ));
                        _ = self.input.next();
                    }
                    '{' => {
                        res.push(Token::new(
                            TokenValue::BlockStart,
                            loc.stop(self.line, self.col),
                        ));
                        _ = self.input.next();
                    }
                    '}' => {
                        res.push(Token::new(
                            TokenValue::BlockEnd,
                            loc.stop(self.line, self.col),
                        ));
                        _ = self.input.next();
                    }
                    '=' => {
                        _ = self.input.next();
                        match self.input.peek() {
                            Some(&c) => {
                                if c == '=' {
                                    res.push(Token::new(
                                        TokenValue::Equal,
                                        loc.stop(self.line, self.col),
                                    ));
                                    _ = self.input.next();
                                } else {
                                    res.push(Token::new(
                                        TokenValue::Assign,
                                        loc.stop(self.line, self.col),
                                    ));
                                }
                            }
                            None => {
                                res.push(Token::new(
                                    TokenValue::Assign,
                                    loc.stop(self.line, self.col),
                                ));
                            }
                        }
                    }
                    '+' => {
                        res.push(Token::new(TokenValue::Plus, loc.stop(self.line, self.col)));
                        _ = self.input.next();
                    }
                    '-' => {
                        res.push(Token::new(TokenValue::Minus, loc.stop(self.line, self.col)));
                        _ = self.input.next();
                    }
                    '*' => {
                        res.push(Token::new(
                            TokenValue::Asterisk,
                            loc.stop(self.line, self.col),
                        ));
                        _ = self.input.next();
                    }
                    '/' => {
                        res.push(Token::new(TokenValue::Slash, loc.stop(self.line, self.col)));
                        _ = self.input.next();
                    }
                    '&' => {
                        _ = self.input.next();
                        match self.input.peek() {
                            Some(&c) => {
                                if c == '&' {
                                    res.push(Token::new(
                                        TokenValue::And,
                                        loc.stop(self.line, self.col),
                                    ));
                                    _ = self.input.next();
                                } else {
                                    res.push(Token::new(
                                        TokenValue::Illegal("unexpected: &".to_string()),
                                        loc.stop(self.line, self.col),
                                    ));
                                }
                            }
                            None => {
                                res.push(Token::new(
                                    TokenValue::Illegal("unexpected: &".to_string()),
                                    loc.stop(self.line, self.col),
                                ));
                            }
                        }
                    }
                    '|' => {
                        _ = self.input.next();
                        match self.input.peek() {
                            Some(&c) => {
                                if c == '|' {
                                    res.push(Token::new(
                                        TokenValue::Or,
                                        loc.stop(self.line, self.col),
                                    ));
                                    _ = self.input.next();
                                } else {
                                    res.push(Token::new(
                                        TokenValue::Illegal("unexpected: |".to_string()),
                                        loc.stop(self.line, self.col),
                                    ));
                                }
                            }
                            None => {
                                res.push(Token::new(
                                    TokenValue::Illegal("unexpected: |".to_string()),
                                    loc.stop(self.line, self.col),
                                ));
                            }
                        }
                    }
                    '!' => {
                        res.push(Token::new(TokenValue::Bang, loc.stop(self.line, self.col)));
                        _ = self.input.next();
                    }
                    '0'..='9' => res.push(self.lex_int_or_float(loc)),
                    '"' => res.push(self.lex_string(loc)),
                    'a'..='z' | 'A'..='Z' | '_' => res.push(self.lex_ident(loc)),
                    _ => {
                        res.push(Token::new(
                            TokenValue::Illegal(format!("unexpected: {c}")),
                            loc.stop(self.line, self.col),
                        ));
                        _ = self.input.next();
                    }
                },
                None => {
                    res.push(Token::new(TokenValue::EOF, loc.stop(self.line, self.col)));
                    break;
                }
            }
        }

        res
    }

    fn lex_int_or_float(&mut self, loc: Location) -> Token {
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
                        return Token::new(
                            TokenValue::Illegal(format!("unexpected: {c}")),
                            loc.stop(self.line, self.col),
                        );
                    }
                    float = true;
                    value.push('.');
                    _ = self.input.next();
                }
                _ => break,
            }
        }

        if float {
            Token::new(TokenValue::Float(value), loc.stop(self.line, self.col))
        } else {
            Token::new(TokenValue::Integer(value), loc.stop(self.line, self.col))
        }
    }

    fn lex_string(&mut self, loc: Location) -> Token {
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
                        break Token::new(
                            TokenValue::String(string),
                            loc.stop(self.line, self.col),
                        );
                    }
                    _ => {
                        string.push(c);
                        _ = self.input.next();
                    }
                },
                None => {
                    break Token::new(
                        TokenValue::Illegal("unterminated quote string".to_string()),
                        loc.stop(self.line, self.col),
                    );
                }
            }
        }
    }

    fn lex_ident(&mut self, loc: Location) -> Token {
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

        let value = match ident.as_str() {
            "if" => TokenValue::If,
            "elif" => TokenValue::Elif,
            "else" => TokenValue::Else,
            "true" => TokenValue::True,
            "false" => TokenValue::False,
            _ => TokenValue::Ident(ident),
        };

        Token::new(value, loc.stop(self.line, self.col))
    }
}

// #[cfg(test)]
// mod test {
//     use super::{Lexer, Token};

//     #[test]
//     fn test_empty_input() {
//         let tokens = Lexer::new("").lex();
//         assert_eq!(tokens, [Token::EOF]);
//     }

//     #[test]
//     fn test_parentheses() {
//         let tokens = Lexer::new("(()()()())").lex();
//         assert_eq!(
//             tokens,
//             [
//                 Token::LeftParen,
//                 Token::LeftParen,
//                 Token::RightParen,
//                 Token::LeftParen,
//                 Token::RightParen,
//                 Token::LeftParen,
//                 Token::RightParen,
//                 Token::LeftParen,
//                 Token::RightParen,
//                 Token::RightParen,
//                 Token::EOF
//             ]
//         );
//     }

//     #[test]
//     fn test_integers() {
//         let tokens = Lexer::new("123456").lex();
//         assert_eq!(tokens, [Token::Integer("123456".to_string()), Token::EOF]);
//     }

//     #[test]
//     fn test_floats() {
//         let tokens = Lexer::new("3.14159265").lex();
//         assert_eq!(tokens, [Token::Float("3.14159265".to_string()), Token::EOF]);
//     }

//     #[test]
//     fn test_strings() {
//         let tokens = Lexer::new("\"foo bar baz\"").lex();
//         assert_eq!(
//             tokens,
//             [Token::String("foo bar baz".to_string()), Token::EOF]
//         );
//     }

//     #[test]
//     fn test_multi_line_strings() {
//         let tokens = Lexer::new(
//             "
// \"
// foo
// bar
// baz
// \"",
//         )
//         .lex();

//         assert_eq!(
//             tokens,
//             [
//                 Token::Newline,
//                 Token::String("\nfoo\nbar\nbaz\n".to_string()),
//                 Token::EOF
//             ]
//         );
//     }

//     #[test]
//     fn test_boolean_true() {
//         let tokens = Lexer::new("true").lex();
//         assert_eq!(tokens, [Token::True, Token::EOF]);
//     }

//     #[test]
//     fn test_boolean_false() {
//         let tokens = Lexer::new("false").lex();
//         assert_eq!(tokens, [Token::False, Token::EOF]);
//     }

//     #[test]
//     fn test_identifiers() {
//         let tokens = Lexer::new("foo bar").lex();
//         assert_eq!(
//             tokens,
//             [
//                 Token::Ident("foo".to_string()),
//                 Token::Ident("bar".to_string()),
//                 Token::EOF
//             ]
//         );
//     }

//     #[test]
//     fn test_assignment() {
//         let tokens = Lexer::new("= foo 123").lex();
//         assert_eq!(
//             tokens,
//             [
//                 Token::Assign,
//                 Token::Ident("foo".to_string()),
//                 Token::Integer("123".to_string()),
//                 Token::EOF
//             ]
//         );
//     }

//     #[test]
//     fn test_operators() {
//         let tokens = Lexer::new("===+-*/!").lex();
//         assert_eq!(
//             tokens,
//             [
//                 Token::Equal,
//                 Token::Assign,
//                 Token::Plus,
//                 Token::Minus,
//                 Token::Asterisk,
//                 Token::Slash,
//                 Token::Bang,
//                 Token::EOF
//             ]
//         );
//     }
// }
