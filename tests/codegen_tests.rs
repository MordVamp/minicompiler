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

// Additional complex tests for code generation (Sprint 5-6)

#[test]
fn test_nested_loops_and_conditionals() {
    let src = "fn main() -> int { int sum = 0; for (int i = 0; i < 5; i = i + 1) { if (i % 2 == 0) { sum = sum + i; } } return sum; }";
    let asm = compile(src);
    // Expect loop labels and conditional jumps
    assert!(asm.contains(".for_cond_1:"));
    assert!(asm.contains(".for_body_2:"));
    assert!(asm.contains(".for_update_3:"));
    // Conditional inside loop
    assert!(asm.contains(".if_then_"));
    assert!(asm.contains(".if_end_"));
}

#[test]
fn test_function_call_and_return() {
    let src = "fn add(a int, b int) -> int { return a + b; } fn main() -> int { int result = add(3, 4); return result; }";
    let asm = compile(src);
    // Check that a function label is emitted and called
    assert!(asm.contains("func_add:"));
    assert!(asm.contains("call func_add"));
    assert!(asm.contains("ret"));
}

#[test]
fn test_recursive_factorial() {
    let src = "fn fact(n int) -> int { if (n <= 1) { return 1; } else { return n * fact(n - 1); } } fn main() -> int { return fact(5); }";
    let asm = compile(src);
    // Ensure recursive call label exists
    assert!(asm.contains("func_fact:"));
    assert!(asm.contains("call func_fact"));
}

// Test optimizer behavior directly
#[test]
fn test_optimizer_mov_zero_to_xor() {
    let input_asm = "    mov rax, 0\n    mov rbx, 1";
    let optimized = minicompiler::codegen::optimizer::PeepholeOptimizer::optimize(input_asm.to_string());
    assert!(optimized.contains("xor eax, eax"));
    assert!(!optimized.contains("mov rax, 0"));
}
