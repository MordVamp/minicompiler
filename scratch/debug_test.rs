use minicompiler::lexer::{Scanner, TokenType};
use minicompiler::parser::Parser;
use minicompiler::semantic::analyzer::SemanticAnalyzer;
use minicompiler::ir::ir_generator::IRGenerator;
use minicompiler::ir::ssa_constructor::SSAConstructor;
use minicompiler::codegen::X86Generator;

fn main() {
    let src = "fn main() -> int { if (1 > 0) { return 10; } else { return 20; } }";
    let mut scanner = Scanner::new(src);
    let mut tokens = Vec::new();
    loop {
        let t = scanner.next_token();
        let is_eof = t.token_type == TokenType::EndOfFile;
        tokens.push(t);
        if is_eof { break; }
    }
    let mut parser = Parser::new(tokens);
    let mut ast = parser.parse().unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&mut ast);
    let mut ir_gen = IRGenerator::new();
    ir_gen.generate(&ast);
    let mut ssa_builder = SSAConstructor::new(ir_gen.blocks);
    ssa_builder.construct();
    let mut codegen = X86Generator::new(ssa_builder.blocks, ir_gen.functions, ir_gen.strings);
    let asm = codegen.generate();
    println!("{}", asm);
}
