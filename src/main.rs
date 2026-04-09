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
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Запуск синтаксического анализатора для построения AST.
    Parse {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(long, value_enum, default_value_t = AstFormat::Text)]
        ast_format: AstFormat,
        #[arg(short, long)]
        verbose: bool,
    },
    /// Запуск семантического анализатора (Sprint 3).
    Check {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        verbose: bool,
    },
    /// Генерация IR кода (Sprint 4).
    Ir {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(short, long)]
        verbose: bool,
    },
    /// Запуск всех тестов.
    Test,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Lex { input, output } => run_lexer(&input, output.as_ref()),
        Commands::Parse { input, output, ast_format, verbose } => run_parser(&input, output.as_ref(), ast_format, verbose),
        Commands::Check { input, verbose } => run_check(&input, verbose),
        Commands::Ir { input, output, verbose } => run_ir(&input, output.as_ref(), verbose),
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

fn run_check(input_path: &PathBuf, verbose: bool) -> Result<()> {
    let source = fs::read_to_string(input_path)?;
    let mut scanner = Scanner::new(&source);
    let mut tokens = Vec::new();
    loop {
        let token = scanner.next_token();
        let is_eof = token.token_type == TokenType::EndOfFile;
        tokens.push(token);
        if is_eof { break; }
    }

    let mut parser = Parser::new(tokens);
    let mut ast = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("Ошибка парсинга: {}", e);
            std::process::exit(1);
        }
    };

    let mut analyzer = minicompiler::semantic::analyzer::SemanticAnalyzer::new();
    if analyzer.analyze(&mut ast) {
        if verbose {
            println!("Семантический анализ прошел успешно.");
            println!("{}", analyzer.symbol_table.dump());
        } else {
            println!("OK");
        }
    } else {
        eprintln!("Найдены семантические ошибки:");
        for err in analyzer.errors {
            eprintln!("{}", err);
        }
        std::process::exit(1);
    }
    Ok(())
}

fn run_ir(input_path: &PathBuf, output_path: Option<&PathBuf>, verbose: bool) -> Result<()> {
    let source = fs::read_to_string(input_path)?;
    let mut scanner = Scanner::new(&source);
    let mut tokens = Vec::new();
    loop {
        let token = scanner.next_token();
        let is_eof = token.token_type == TokenType::EndOfFile;
        tokens.push(token);
        if is_eof { break; }
    }

    let mut parser = Parser::new(tokens);
    let mut ast = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("Ошибка парсинга: {}", e);
            std::process::exit(1);
        }
    };

    let mut analyzer = minicompiler::semantic::analyzer::SemanticAnalyzer::new();
    if !analyzer.analyze(&mut ast) {
        eprintln!("Семантический анализ выявил ошибки. Генерация IR отменена.");
        for err in analyzer.errors {
            eprintln!("{}", err);
        }
        std::process::exit(1);
    }

    let mut ir_gen = minicompiler::ir::ir_generator::IRGenerator::new();
    ir_gen.generate(&ast);

    let mut ssa_builder = minicompiler::ir::ssa_constructor::SSAConstructor::new(ir_gen.blocks);
    ssa_builder.construct();

    let mut output_str = String::from("--- IR Code (SSA Form) ---\n");
    let mut keys: Vec<String> = ssa_builder.blocks.keys().cloned().collect();
    keys.sort();
    for key in keys {
        output_str.push_str(&ssa_builder.blocks[&key].to_string());
        output_str.push('\n');
    }

    if verbose {
        println!("Генерация IR завершена успешно.");
    }

    match output_path {
        Some(path) => fs::write(path, output_str)?,
        None => print!("{}", output_str),
    }

    Ok(())
}
