use clap::Parser as ClapParser;
use clip::{
    eval::{eval, Scope},
    lexer::Lexer,
    parser::{ast::Statement, Parser},
    repl,
};
use std::fs;

#[derive(ClapParser)]
struct Args {
    #[arg(short, long)]
    token: bool,
    #[arg(short, long)]
    parse: bool,
    file: Option<String>,
}

fn main() {
    let args = Args::parse();

    if args.token && args.parse {
        eprintln!("error: cannot specify both --token and --parse flags");
        return;
    }

    if args.file.is_none() {
        return repl::repl(args.token, args.parse);
    }

    match fs::read_to_string(args.file.unwrap()) {
        Ok(input) => {
            let tokens = Lexer::new(&input).lex();
            if args.token {
                for token in &tokens {
                    println!("{:?}", token);
                }
                return;
            }

            match Parser::new(tokens).parse() {
                Ok(p) => {
                    if args.parse {
                        for stmt in &p.statements {
                            match stmt {
                                Statement::Assign(a) => println!("{:#?}", a),
                                Statement::Expression(e) => println!("{:#?}", e),
                            }
                        }
                        return;
                    }

                    match eval(p, &mut Scope::default()) {
                        Ok(v) => println!("{} : {}", v, v.value()),
                        Err(e) => eprintln!("{}", e),
                    }
                }
                Err(e) => eprintln!("{}", e),
            }
        }
        Err(e) => eprintln!("{}", e),
    }
}
