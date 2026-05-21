use std::fs;

fn main() {
    let src = fs::read_to_string("opt_extreme.src").unwrap();
    let mut parser = minicompiler::parser::parser::Parser::new(&src);
    let ast = parser.parse();
    let mut ir_gen = minicompiler::ir::ir_generator::IRGenerator::new();
    ir_gen.generate(&ast);
    for (name, blk) in &ir_gen.blocks {
        println!("Block: {}", name);
        for inst in &blk.instructions {
            println!("  {:?}", inst);
        }
    }
}
