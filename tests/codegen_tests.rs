use minicompiler::lexer::{Scanner, TokenType};
use minicompiler::parser::Parser;
use minicompiler::semantic::analyzer::SemanticAnalyzer;
use minicompiler::ir::ir_generator::IRGenerator;
use minicompiler::ir::ssa_constructor::SSAConstructor;
use minicompiler::codegen::X86Generator;

fn compile(source: &str) -> String {
    let mut scanner = Scanner::new(source);
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
    let mut codegen = X86Generator::new(ssa_builder.blocks, ir_gen.functions);
    codegen.generate()
}

#[test]
fn test_if_else_generation() {
    let src = "fn main() -> int { if (1 > 0) { return 10; } else { return 20; } }";
    let asm = compile(src);
    assert!(asm.contains("cmp rax, 0"));
    assert!(asm.contains("setg al"));
    // check for the jump instruction to else
    assert!(asm.contains("je .else_")); 
    assert!(asm.contains(".then_"));
}

#[test]
fn test_for_loop_generation() {
    let src = "fn main() -> int { int s = 0; for (int i = 0; i < 10; i = i + 1) { s = s + i; } return s; }";
    let asm = compile(src);
    assert!(asm.contains(".for_cond_1:"));
    assert!(asm.contains(".for_body_2:"));
    assert!(asm.contains(".for_update_3:"));
    assert!(asm.contains("jmp .for_cond_1"));
}

#[test]
fn test_short_circuit_and() {
    let src = "fn main() -> int { bool x = true; bool y = false; if (x && y) { return 1; } return 0; }";
    let asm = compile(src);
    assert!(asm.contains(".and_false_"));
    assert!(asm.contains(".and_end_"));
}
