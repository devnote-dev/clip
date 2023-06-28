use clip::{eval::Evaluator, lexer::Lexer, parser::Parser};
use std::{
    env,
    io::{self, Write},
};

fn main() {
    if env::args().len() == 1 {
        repl();
    } else {
        let input = env::args().skip(1).collect::<String>();
        let tokens = Lexer::new(&input).lex();
        // println!("{:?}", tokens);

        match Parser::new(tokens).parse() {
            Ok(p) => match Evaluator::new(p).eval() {
                Ok(v) => println!("{:?}", v),
                Err(e) => eprintln!("{}", e),
            },
            Err(e) => eprintln!("{}", e),
        }
    }
}

fn repl() {
    let mut input = String::new();

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let tokens = Lexer::new(&input).lex();
        // println!("{:?}", tokens);

        match Parser::new(tokens).parse() {
            Ok(p) => {
                for stmt in p.statements {
                    println!("{:?}", stmt);
                }
            }
            Err(e) => eprintln!("{}", e),
        }

        input.clear();
    }
}
