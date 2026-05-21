use minicompiler::lexer::{Scanner, TokenType};
use minicompiler::parser::Parser;
use minicompiler::semantic::analyzer::SemanticAnalyzer;
use minicompiler::ir::ir_generator::IRGenerator;
use minicompiler::ir::ssa_constructor::SSAConstructor;

fn main() {
    let src = std::fs::read_to_string("examples/sprint6/complex_test.src").unwrap();
    let mut scanner = Scanner::new(&src);
    let mut tokens = Vec::new();
    loop {
        let t = scanner.next_token();
        let done = t.token_type == TokenType::EndOfFile;
        tokens.push(t);
        if done { break; }
    }
    let mut parser = Parser::new(tokens);
    let mut ast = parser.parse().unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&mut ast);
    let mut ir_gen = IRGenerator::new();
    ir_gen.generate(&ast);
    let mut ssa = SSAConstructor::new(ir_gen.blocks);
    ssa.construct();

    println!("=== Block successors after SSA ===");
    let mut keys: Vec<_> = ssa.blocks.keys().collect();
    keys.sort();
    for k in keys {
        let b = &ssa.blocks[k];
        println!("{} -> succs: {:?}  preds: {:?}", k, b.successors, b.predecessors);
    }
}
