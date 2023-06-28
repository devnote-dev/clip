use clip::{lexer::Lexer, parser::Parser};
use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let tokens = Lexer::new(&input).lex();
        match Parser::new(tokens).parse() {
            Ok(p) => {
                for stmt in p.statements {
                    println!("{:?}", stmt);
                }
            }
            Err(e) => eprintln!("{}", e),
        }
    }
}
