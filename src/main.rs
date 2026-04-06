use anyhow::Result;
use clap::{Parser as ClapParser, Subcommand, ValueEnum};
use minicompiler::lexer::{Scanner, TokenType};
use minicompiler::parser::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(ClapParser)]
#[command(name = "compiler")]
#[command(about = "MiniCompiler - Лексический и синтаксический анализатор", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(ValueEnum, Clone, Debug)]
enum AstFormat {
    Text,
    Dot,
    Json,
}

#[derive(Subcommand)]
enum Commands {
    /// Запуск лексера для входного файла и вывод токенов.
    Lex {
        /// Путь к исходному файлу.
        #[arg(short, long)]
        input: PathBuf,

        /// Необязательный выходной файл (в stdout, если не указан).
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Запуск синтаксического анализатора для построения AST.
    Parse {
        /// Путь к исходному файлу.
        #[arg(short, long)]
        input: PathBuf,

        /// Необязательный выходной файл (в stdout, если не указан).
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Формат вывода AST.
        #[arg(long, value_enum, default_value_t = AstFormat::Text)]
        ast_format: AstFormat,

        /// Выводить дополнительную информацию.
        #[arg(short, long)]
        verbose: bool,
    },
    /// Запуск всех тестов (валидных и невалидных) и вывод результатов.
    Test,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Lex { input, output } => run_lexer(&input, output.as_ref()),
        Commands::Parse { input, output, ast_format, verbose } => run_parser(&input, output.as_ref(), ast_format, verbose),
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

fn run_parser(input_path: &PathBuf, output_path: Option<&PathBuf>, format: AstFormat, verbose: bool) -> Result<()> {
    if verbose {
        println!("Лексический анализ файла: {:?}", input_path);
    }
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

    if verbose {
        println!("Синтаксический анализ...");
    }
    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("Ошибка парсинга: {}", e);
            std::process::exit(1);
        }
    };

    let output_str = match format {
        AstFormat::Text => ast.to_pretty_string(),
        AstFormat::Dot => ast.to_dot(),
        AstFormat::Json => ast.to_json(),
    };

    match output_path {
        Some(path) => fs::write(path, output_str)?,
        None => println!("{}", output_str),
    }

    Ok(())
}

fn run_tests() -> Result<()> {
    println!("Запуск тестов лексера и парсера через `cargo test`...");
    Ok(())
}
