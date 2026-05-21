use anyhow::Result;
use minicompiler::codegen::optimizer::PeepholeOptimizer;
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
        #[arg(long, value_enum, default_value_t = AstFormat::Text)]
        format: AstFormat,
        #[arg(short, long)]
        verbose: bool,
        #[arg(short, long)]
        stats: bool,
    },
    /// Компиляция в x86-64 ассемблер (Sprint 5-6).
    Compile {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(short, long)]
        verbose: bool,
        #[arg(short, long, action = clap::ArgAction::SetTrue, help = "Print assembly to console even if output file is specified")] 
        stdout: bool,
        #[arg(short = 'O', long, action = clap::ArgAction::SetTrue, help = "Show assembly before and after optimization")]
        optimize: bool,
    },
    /// Запуск всех тестов.
    Test,
    /// Полный дамп всех этапов компиляции (Токены, AST, Символы, IR).
    Dump {
        #[arg(short, long)]
        input: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Lex { input, output } => run_lexer(&input, output.as_ref()),
        Commands::Parse { input, output, ast_format, verbose } => run_parser(&input, output.as_ref(), ast_format, verbose),
        Commands::Check { input, verbose } => run_check(&input, verbose),
        Commands::Ir { input, output, format, verbose, stats } => run_ir(&input, output.as_ref(), format, verbose, stats),
        Commands::Compile { input, output, verbose, stdout, optimize } => run_compile(&input, output.as_ref(), verbose, stdout, optimize),
        Commands::Dump { input } => run_dump(&input),
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
            eprintln!("{}", minicompiler::utils::format_error_with_context(&source, err.position.line, err.position.column, &err.message));
        }
        std::process::exit(1);
    }
    Ok(())
}

fn run_ir(input_path: &PathBuf, output_path: Option<&PathBuf>, format: AstFormat, verbose: bool, stats: bool) -> Result<()> {
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
            eprintln!("{}", minicompiler::utils::format_error_with_context(&source, err.position.line, err.position.column, &err.message));
        }
        std::process::exit(1);
    }

    let mut ir_gen = minicompiler::ir::ir_generator::IRGenerator::new();
    ir_gen.generate(&ast);

    let mut ssa_builder = minicompiler::ir::ssa_constructor::SSAConstructor::new(ir_gen.blocks);
    ssa_builder.construct();

    let output_str = match format {
        AstFormat::Text => {
            let mut s = String::from("--- IR Code (SSA Form) ---\n");
            let mut keys: Vec<String> = ssa_builder.blocks.keys().cloned().collect();
            keys.sort();
            for key in keys {
                s.push_str(&ssa_builder.blocks[&key].to_string());
                s.push('\n');
            }
            s
        }
        AstFormat::Dot => {
            let mut s = String::from("digraph CFG {\n  node [shape=record];\n");
            let mut keys: Vec<String> = ssa_builder.blocks.keys().cloned().collect();
            keys.sort();
            for key in &keys {
                let bb = &ssa_builder.blocks[key];
                let mut label = format!("{}:\\l", bb.label);
                for inst in &bb.instructions {
                    label.push_str(&format!("  {}\\l", format!("{}", inst).replace("\"", "\\\"")));
                }
                s.push_str(&format!("  {} [label=\"{{{}}}\"];\n", key.replace(".", "_"), label));
                for succ in &bb.successors {
                    s.push_str(&format!("  {} -> {};\n", key.replace(".", "_"), succ.replace(".", "_")));
                }
            }
            s.push_str("}\n");
            s
        }
        AstFormat::Json => {
            // Placeholder for JSON or reuse serde if implemented for IR
            String::from("JSON IR not implemented yet")
        }
    };

    if stats {
        let mut inst_count = 0;
        let block_count = ssa_builder.blocks.len();
        for bb in ssa_builder.blocks.values() {
            inst_count += bb.instructions.len();
        }
        println!("=== IR Statistics ===");
        println!("Number of basic blocks: {}", block_count);
        println!("Number of instructions: {}", inst_count);
        println!("=====================");
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

fn run_dump(input_path: &PathBuf) -> Result<()> {
    let source = fs::read_to_string(input_path)?;

    // 1. Lexer
    println!("=== 1. TOKENS (OUTPUT TABLE) ===");
    let mut scanner = Scanner::new(&source);
    let mut tokens = Vec::new();
    loop {
        let token = scanner.next_token();
        let is_eof = token.token_type == TokenType::EndOfFile;
        println!("{}", token);
        tokens.push(token);
        if is_eof { break; }
    }
    println!();

    // 2. Parser
    println!("=== 2. ABSTRACT SYNTAX TREE ===");
    let mut parser = Parser::new(tokens.clone());
    let mut ast = match parser.parse() {
        Ok(ast) => {
            println!("{}", ast.to_pretty_string());
            ast
        },
        Err(e) => {
            eprintln!("Ошибка парсинга: {}", e);
            std::process::exit(1);
        }
    };
    println!();

    // 3. Semantic
    println!("=== 3. ANNOTATED SYMBOL TABLE ===");
    let mut analyzer = minicompiler::semantic::analyzer::SemanticAnalyzer::new();
    if analyzer.analyze(&mut ast) {
        println!("{}", analyzer.symbol_table.dump());
    } else {
        eprintln!("Найдены семантические ошибки:");
        for err in analyzer.errors {
            eprintln!("{}", err);
        }
        std::process::exit(1);
    }
    println!();

    // 4. IR / SSA
    println!("=== 4. SSA IR CODE ===");
    let mut ir_gen = minicompiler::ir::ir_generator::IRGenerator::new();
    ir_gen.generate(&ast);

    let mut ssa_builder = minicompiler::ir::ssa_constructor::SSAConstructor::new(ir_gen.blocks);
    ssa_builder.construct();

    let mut keys: Vec<String> = ssa_builder.blocks.keys().cloned().collect();
    keys.sort();
    for key in keys {
        println!("{}", ssa_builder.blocks[&key].to_string());
    }

    Ok(())
}
fn run_compile(input_path: &PathBuf, output_path: Option<&PathBuf>, verbose: bool, stdout: bool, optimize: bool) -> Result<()> {
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
        eprintln!("Семантический анализ выявил ошибки. Компиляция отменена.");
        for err in analyzer.errors {
            eprintln!("{}", minicompiler::utils::format_error_with_context(&source, err.position.line, err.position.column, &err.message));
        }
        std::process::exit(1);
    }

    let mut ir_gen = minicompiler::ir::ir_generator::IRGenerator::new();
    ir_gen.generate(&ast);

    let mut ssa_builder = minicompiler::ir::ssa_constructor::SSAConstructor::new(ir_gen.blocks);
    ssa_builder.construct();
    
    let mut blocks = ssa_builder.blocks;
    if optimize {
        let mut ir_optimizer = minicompiler::ir::optimizer::IROptimizer::new(blocks);
        ir_optimizer.optimize();
        blocks = ir_optimizer.blocks;
        
        println!("--- Blocks Dump After Opt ---");
        for (name, blk) in &blocks {
            println!("Block: {}", name);
            for inst in &blk.instructions {
                println!("  {:?}", inst);
            }
        }
        println!("-------------------");
    }

    let mut codegen = minicompiler::codegen::X86Generator::new(blocks, ir_gen.functions, ir_gen.strings);
    let asm = codegen.generate();

    if verbose {
        println!("Генерация кода x86-64 завершена успешно.");
    }

    // Optimization and output handling
    if optimize {
        println!("--- Assembly before optimization ---\n{}", asm);
        let optimized = minicompiler::codegen::optimizer::PeepholeOptimizer::optimize(asm.clone());
        println!("--- Assembly after optimization ---\n{}", optimized);
        if stdout {
            // print optimized asm to console
            print!("{}", optimized);
        } else {
            match output_path {
                Some(path) => fs::write(path, optimized)?,
                None => print!("{}", optimized),
            }
        }
    } else {
        if stdout {
            print!("{}", asm);
        } else {
            match output_path {
                Some(path) => fs::write(path, asm)?,
                None => print!("{}", asm),
            }
        }
    }

    Ok(())
}
