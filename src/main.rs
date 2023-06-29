use clap::Parser as ClapParser;
use clip::{
    eval::{eval, Scope},
    lexer::Lexer,
    parser::{ast::Statement, Parser},
};
use std::{
    fs,
    io::{self, Write},
};

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

    match args.file {
        Some(f) => match fs::read_to_string(f) {
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
        },
        None => repl(args),
    }
}

fn repl(args: Args) {
    let mut input = String::new();
    let mut scope = Scope::default();

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let tokens = Lexer::new(&input).lex();
        if args.token {
            for token in &tokens {
                println!("{:?}", token);
            }
            continue;
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
