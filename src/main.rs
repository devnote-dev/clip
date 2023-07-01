use clap::{Parser as ClapParser, Subcommand};
use clip::{
    eval::{eval, Scope},
    lexer::Lexer,
    parser::{ast::Statement, Parser},
    repl,
};
use std::fs;

#[derive(ClapParser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a clip script file
    Run {
        /// Display the input script as comments
        #[arg(short, long)]
        display: bool,
        /// Print the parsed abstract syntax tree
        #[arg(short, long)]
        parse: bool,
        /// Print the parsed tokens
        #[arg(short, long)]
        token: bool,
        /// The input file
        file: String,
    },
    /// Start the clip interpreter repl
    Repl {
        /// Print the parsed abstract syntax tree
        #[arg(short, long)]
        parse: bool,
        /// Print the parsed tokens
        #[arg(short, long)]
        token: bool,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Run {
            display,
            parse,
            token,
            file,
        } => run(file, display, token, parse),
        Commands::Repl { parse, token } => repl::repl(token, parse),
    }
}

fn run(path: String, display: bool, show_token: bool, show_parse: bool) {
    if show_token && show_parse {
        eprintln!("error: cannot specify both --token and --parse flags");
        return;
    }

    match fs::read_to_string(path) {
        Ok(input) => {
            if display {
                for line in input.lines() {
                    println!("# {}", line);
                }
            }

            let tokens = Lexer::new(&input).lex();
            if show_token {
                for token in &tokens {
                    println!("{:?}", token);
                }
                return;
            }

            match Parser::new(tokens).parse() {
                Ok(p) => {
                    if show_parse {
                        for stmt in &p.statements {
                            match stmt {
                                Statement::Assign(a) => println!("{:#?}", a),
                                Statement::If(_) => println!("null"),
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
