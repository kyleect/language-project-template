use std::fs;
use std::process;

use clap::{Parser, Subcommand};
use language_project_template::lexer::lex;
use language_project_template::parser::parse;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long, value_parser, help = "Path to the input file")]
    file_path: String,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Lex,
    Parse,
}

fn main() {
    let cli = Cli::parse();

    let contents = fs::read_to_string(&cli.file_path).unwrap_or_else(|err| {
        eprintln!("Error reading file `{}`: {}", cli.file_path, err);
        process::exit(1);
    });

    match &cli.command {
        Commands::Lex => {
            let tokens = lex(&contents);
            println!("{:#?}", tokens);
        }
        Commands::Parse => {
            let ast = parse(&contents);
            println!("{ast:#?}");
        }
    }
}
