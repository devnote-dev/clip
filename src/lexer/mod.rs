use self::token::{Location, Token, TokenValue};
use std::{iter::Peekable, str::Chars};

pub mod token;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    loc: Location,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
            loc: Location::new(0, 0),
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut res = Vec::new();

        loop {
            match self.input.peek() {
                Some(&c) => match c {
                    ' ' | '\t' => {
                        self.next();
                        self.loc.col_start = self.loc.col_stop;
                    }
                    '\r' => {
                        if let Some(c) = self.input.next() {
                            if c == '\n' {
                                res.push(Token::new(TokenValue::Newline, self.loc()));
                                self.next();
                                self.loc.line_start += 1;
                                self.loc.col_stop = 0;
                            }
                        }
                    }
                    '\n' => {
                        res.push(Token::new(TokenValue::Newline, self.loc()));
                        self.next();
                        self.loc.line_start += 1;
                        self.loc.col_stop = 0;
                    }
                    ';' => {
                        res.push(Token::new(TokenValue::Semicolon, self.loc()));
                        self.next();
                    }
                    '#' => loop {
                        match self.input.next() {
                            Some(c) => {
                                if c == '\n' {
                                    break;
                                }
                            }
                            None => {
                                res.push(Token::new(TokenValue::EOF, self.loc()));
                                break;
                            }
                        }
                    },
                    '(' => {
                        res.push(Token::new(TokenValue::LeftParen, self.loc()));
                        self.next();
                    }
                    ')' => {
                        res.push(Token::new(TokenValue::RightParen, self.loc()));
                        self.next();
                    }
                    '[' => {
                        res.push(Token::new(TokenValue::LeftBracket, self.loc()));
                        self.next();
                    }
                    ']' => {
                        res.push(Token::new(TokenValue::RightBracket, self.loc()));
                        self.next();
                    }
                    '{' => {
                        res.push(Token::new(TokenValue::BlockStart, self.loc()));
                        self.next();
                    }
                    '}' => {
                        res.push(Token::new(TokenValue::BlockEnd, self.loc()));
                        self.next();
                    }
                    '=' => {
                        self.next();
                        match self.input.peek() {
                            Some(&c) => {
                                if c == '=' {
                                    res.push(Token::new(TokenValue::Equal, self.loc()));
                                    self.next();
                                } else {
                                    res.push(Token::new(TokenValue::Assign, self.loc()));
                                }
                            }
                            None => {
                                res.push(Token::new(TokenValue::Assign, self.loc()));
                            }
                        }
                    }
                    '+' => {
                        res.push(Token::new(TokenValue::Plus, self.loc()));
                        self.next();
                    }
                    '-' => {
                        res.push(Token::new(TokenValue::Minus, self.loc()));
                        self.next();
                    }
                    '*' => {
                        res.push(Token::new(TokenValue::Asterisk, self.loc()));
                        self.next();
                    }
                    '/' => {
                        res.push(Token::new(TokenValue::Slash, self.loc()));
                        self.next();
                    }
                    '&' => {
                        self.next();
                        match self.input.peek() {
                            Some(&c) => {
                                if c == '&' {
                                    res.push(Token::new(TokenValue::And, self.loc()));
                                    self.next();
                                } else {
                                    res.push(Token::new(
                                        TokenValue::Illegal("unexpected: &".to_string()),
                                        self.loc(),
                                    ));
                                }
                            }
                            None => {
                                res.push(Token::new(
                                    TokenValue::Illegal("unexpected: &".to_string()),
                                    self.loc(),
                                ));
                            }
                        }
                    }
                    '|' => {
                        self.next();
                        match self.input.peek() {
                            Some(&c) => {
                                if c == '|' {
                                    res.push(Token::new(TokenValue::Or, self.loc()));
                                    self.next();
                                } else {
                                    res.push(Token::new(
                                        TokenValue::Illegal("unexpected: |".to_string()),
                                        self.loc(),
                                    ));
                                }
                            }
                            None => {
                                res.push(Token::new(
                                    TokenValue::Illegal("unexpected: |".to_string()),
                                    self.loc(),
                                ));
                            }
                        }
                    }
                    '!' => {
                        res.push(Token::new(TokenValue::Bang, self.loc()));
                        self.next();
                    }
                    '0'..='9' => res.push(self.lex_int_or_float()),
                    '"' => res.push(self.lex_string()),
                    'a'..='z' | 'A'..='Z' | '_' => res.push(self.lex_ident()),
                    _ => {
                        res.push(Token::new(
                            TokenValue::Illegal(format!("unexpected: {c}")),
                            self.loc(),
                        ));
                        self.next();
                    }
                },
                None => {
                    res.push(Token::new(TokenValue::EOF, self.loc()));
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
                    self.next();
                }
                '_' => continue,
                '.' => {
                    if float {
                        self.next();
                        return Token::new(
                            TokenValue::Illegal(format!("unexpected: {c}")),
                            self.loc(),
                        );
                    }
                    float = true;
                    value.push('.');
                    self.next();
                }
                _ => break,
            }
        }

        if float {
            Token::new(TokenValue::Float(value), self.loc())
        } else {
            Token::new(TokenValue::Integer(value), self.loc())
        }
    }

    fn lex_string(&mut self) -> Token {
        let mut string = String::new();
        let mut escaped = false;
        self.next();

        loop {
            match self.input.peek() {
                Some(&c) => match c {
                    '\\' => escaped = !escaped,
                    '"' => {
                        if escaped {
                            escaped = false;
                            continue;
                        }
                        self.next();
                        break Token::new(TokenValue::String(string), self.loc());
                    }
                    _ => {
                        string.push(c);
                        self.next();
                    }
                },
                None => {
                    break Token::new(
                        TokenValue::Illegal("unterminated quote string".to_string()),
                        self.loc(),
                    );
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
                    self.next();
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

        Token::new(value, self.loc())
    }

    fn next(&mut self) {
        _ = self.input.next();
        self.loc.col_stop += 1;
    }

    fn loc(&mut self) -> Location {
        let loc = self.loc.clone();
        self.loc.line_stop = self.loc.line_start;
        self.loc.col_start = self.loc.col_stop;

        loc
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
