use minicompiler::lexer::{Scanner, TokenType};
use minicompiler::parser::Parser;
use minicompiler::semantic::analyzer::SemanticAnalyzer;
use minicompiler::ir::ir_generator::IRGenerator;
use minicompiler::ir::ssa_constructor::SSAConstructor;
use minicompiler::ir::ir_instructions::IRInstruction;
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
    let mut codegen = X86Generator::new(ssa_builder.blocks, ir_gen.functions, ir_gen.strings);
    codegen.generate()
}

#[test]
fn test_if_else_generation() {
    let src = "fn main() -> int { int a = 1; int b = 0; if (a > b) { return 10; } else { return 20; } }";
    let asm = compile(src);
    assert!(asm.contains("cmp rax"));
    assert!(asm.contains("setg al"));
}

#[test]
fn test_array_codegen() {
    let source = "fn main() -> void { int a[5]; a[0] = 10; }";
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
    
    // Проверяем наличие Alloca и GEP в IR
    let mut has_alloca = false;
    let mut has_gep = false;
    for block in ir_gen.blocks.values() {
        for inst in &block.instructions {
            match inst {
                IRInstruction::Alloca { .. } => has_alloca = true,
                IRInstruction::GetElementPtr { .. } => has_gep = true,
                _ => {}
            }
        }
    }
    assert!(has_alloca, "IR should contain Alloca for arrays");
    assert!(has_gep, "IR should contain GetElementPtr for array access");

    let mut codegen = X86Generator::new(ir_gen.blocks, ir_gen.functions, ir_gen.strings);
    let asm = codegen.generate();
    
    // Проверяем ассемблер
    assert!(asm.contains("call malloc"));
    assert!(asm.contains("shl rcx, 3"));
}

#[test]
fn test_extern_printf_codegen() {
    let source = "fn main() -> void { printf(\"hello %d\", 123); }";
    let asm = compile(source);
    
    assert!(asm.contains("extern printf"), "ASM should declare extern printf");
    assert!(asm.contains("mov eax, 0"), "ASM should zero eax for variadic call");
    assert!(asm.contains("section .data"), "ASM should contain .data section for strings");
    assert!(asm.contains("db `hello %d`, 0"), "ASM should contain the string literal");
}

#[test]
fn test_optimizer_mov_zero_to_xor() {
    let input_asm = "    mov rax, 0\n    mov rbx, 1";
    let optimized = minicompiler::codegen::optimizer::PeepholeOptimizer::optimize(input_asm.to_string());
    assert!(optimized.contains("xor eax, eax"));
    assert!(!optimized.contains("mov rax, 0"));
}
