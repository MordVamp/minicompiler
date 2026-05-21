use std::fs;
use minicompiler::lexer::scanner::Scanner;
use minicompiler::lexer::token::TokenType;
use minicompiler::parser::parser::Parser;
use minicompiler::ir::ir_generator::IRGenerator;
use minicompiler::ir::ssa_constructor::SSAConstructor;
use minicompiler::ir::optimizer::IROptimizer;
use minicompiler::codegen::X86Generator;
use minicompiler::codegen::optimizer::PeepholeOptimizer;

#[test]
fn test_sprints_asm_generation() {
    let files = vec![
        "examples/sprint6/complex_test.src",
        "examples/sprint6/nested_loop.src",
        "examples/sprint6/sprint6_control.src",
        "examples/sprint7/sprint7_arrays.src",
        "examples/sprint7/test_arrays.src",
        "examples/sprint7/test_extern.src",
    ];

    for file in files {
        println!("Testing ASM generation for: {}", file);
        let src = fs::read_to_string(file).expect(&format!("Failed to read {}", file));
        
        let mut scanner = Scanner::new(&src);
        let mut tokens = Vec::new();
        loop {
            let t = scanner.next_token();
            let is_eof = t.token_type == TokenType::EndOfFile;
            tokens.push(t);
            if is_eof {
                break;
            }
        }
        
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Failed to parse");
        
        let mut ir_gen = IRGenerator::new();
        ir_gen.generate(&ast);
        
        let mut ssa_builder = SSAConstructor::new(ir_gen.blocks);
        ssa_builder.construct();
        
        let mut ir_optimizer = IROptimizer::new(ssa_builder.blocks);
        ir_optimizer.optimize();
        let blocks = ir_optimizer.blocks;
        
        let mut codegen = X86Generator::new(blocks, ir_gen.functions, ir_gen.strings);
        let asm = codegen.generate();
        let optimized_asm = PeepholeOptimizer::optimize(asm);
        
        assert!(!optimized_asm.is_empty(), "Generated ASM should not be empty for {}", file);
        assert!(optimized_asm.contains("global main"), "Generated ASM should contain main function for {}", file);
        
        // Basic checks for correctness
        assert!(!optimized_asm.contains("idiv 2"), "Should not contain literal idiv");
        
        // Ensure no infinite loop constructs (e.g. jmp to next line or bad fallthrough)
        // Since we can't easily parse asm structure in a simple test, we just check it compiles without crashing.
    }
}
