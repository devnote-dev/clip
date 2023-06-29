use crate::{
    eval::{eval, Scope},
    lexer::Lexer,
    parser::{ast::Statement, Parser},
};
use std::io::{self, Write};

pub fn repl(show_token: bool, show_parse: bool) {
    let mut input = String::new();
    let mut scope = Scope::default();

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let tokens = Lexer::new(&input).lex();
        if show_token {
            for token in &tokens {
                println!("{:?}", token);
            }
            continue;
        }

        match Parser::new(tokens).parse() {
            Ok(p) => {
                if show_parse {
                    for stmt in &p.statements {
                        match stmt {
                            Statement::Assign(a) => println!("{:#?}", a),
                            Statement::Expression(e) => println!("{:#?}", e),
                        }
                    }
                    continue;
                }

                match eval(p, &mut scope) {
                    Ok(v) => println!("{} : {}", v, v.value()),
                    Err(e) => eprintln!("{}", e),
                }
            }
            Err(e) => eprintln!("{}", e),
        }

        input.clear();
    }
}
