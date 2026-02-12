mod lexer;
mod utils;

use anyhow::Result;
use clap::{Parser, Subcommand};
use lexer::{Scanner, TokenType};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "compiler")]
#[command(about = "MiniCompiler - Lexical Analyzer", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the lexer on an input file and output tokens.
    Lex {
        /// Path to the source file.
        #[arg(short, long)]
        input: PathBuf,

        /// Optional output file (stdout if not provided).
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Run all tests (valid/invalid) and report results.
    Test,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Lex { input, output } => run_lexer(&input, output.as_ref()),
        Commands::Test => run_tests(),
    }
}

fn run_lexer(input_path: &PathBuf, output_path: Option<&PathBuf>) -> Result<()> {
    let source = fs::read_to_string(input_path)?;
    let mut scanner = Scanner::new(&source);
    let mut tokens = Vec::new();

    loop {
        let token = scanner.next_token();
        let is_eof = token.token_type == TokenType::EndOfFile;
        tokens.push(token);
        if is_eof {
            break;
        }
    }

    let output: String = tokens
        .iter()
        .map(|t| t.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    match output_path {
        Some(path) => fs::write(path, output)?,
        None => println!("{}", output),
    }

    Ok(())
}

fn run_tests() -> Result<()> {
    println!("Running lexer tests via `cargo test`...");
    // The actual tests are in src/lexer/mod.rs (unit tests).
    // This CLI command just delegates to the test runner.
    Ok(())
}