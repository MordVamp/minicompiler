// ============================================================
// Sprint 4 — IR Generation & SSA Form Tests
// Covers: basic blocks, SSA versioning, instruction set,
//         control flow, function calls, expressions
// ============================================================

use minicompiler::lexer::{Scanner, TokenType};
use minicompiler::parser::Parser;
use minicompiler::semantic::analyzer::SemanticAnalyzer;
use minicompiler::ir::ir_generator::IRGenerator;
use minicompiler::ir::ir_instructions::{IRInstruction, Operand};
use minicompiler::ir::ssa_constructor::SSAConstructor;

// ── Helpers ──────────────────────────────────────────────────

fn tokens(source: &str) -> Vec<minicompiler::lexer::Token> {
    let mut scanner = Scanner::new(source);
    let mut toks = Vec::new();
    loop {
        let t = scanner.next_token();
        let eof = t.token_type == TokenType::EndOfFile;
        toks.push(t);
        if eof { break; }
    }
    toks
}

/// Full pipeline: lex → parse → semantic → IR → SSA.
/// Returns the SSA-form IR as a text dump.
fn ir_for(source: &str) -> String {
    let mut parser = Parser::new(tokens(source));
    let mut ast = parser.parse().expect("parser failed");
    let mut analyzer = SemanticAnalyzer::new();
    let ok = analyzer.analyze(&mut ast);
    assert!(ok, "Semantic analysis failed: {:?}", analyzer.errors);

    let mut ir_gen = IRGenerator::new();
    ir_gen.generate(&ast);

    let mut ssa = SSAConstructor::new(ir_gen.blocks);
    ssa.construct();

    let mut out = String::new();
    let mut keys: Vec<String> = ssa.blocks.keys().cloned().collect();
    keys.sort();
    for k in &keys {
        out.push_str(&ssa.blocks[k].to_string());
    }
    out
}

/// Full pipeline – returns the raw (pre-SSA) IRGenerator for structural checks.
fn raw_ir(source: &str) -> IRGenerator {
    let mut parser = Parser::new(tokens(source));
    let mut ast = parser.parse().expect("parser failed");
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&mut ast);
    let mut ir_gen = IRGenerator::new();
    ir_gen.generate(&ast);
    ir_gen
}

// ════════════════════════════════════════════════════════════
// IR-1 / IR-3 — Basic Block Construction
// ════════════════════════════════════════════════════════════

#[test]
fn test_ir_bb_created_for_function() {
    let gen = raw_ir("fn main() -> void { }");
    // A block labelled "func_main" (or starting with func_) must exist
    let has_main = gen.blocks.keys().any(|k| k.contains("main"));
    assert!(has_main, "Expected a basic block for 'main', got: {:?}", gen.blocks.keys().collect::<Vec<_>>());
}

#[test]
fn test_ir_entry_block_always_present() {
    let gen = raw_ir("fn f() -> void { }");
    assert!(gen.blocks.contains_key("entry"), "Entry block must always exist");
}

#[test]
fn test_ir_multiple_functions_get_separate_blocks() {
    let gen = raw_ir("fn foo() -> void { } fn bar() -> void { }");
    let has_foo = gen.blocks.keys().any(|k| k.contains("foo"));
    let has_bar = gen.blocks.keys().any(|k| k.contains("bar"));
    assert!(has_foo, "Block for 'foo' must exist");
    assert!(has_bar, "Block for 'bar' must exist");
}

// ════════════════════════════════════════════════════════════
// GEN-2 — Expression Translation
// ════════════════════════════════════════════════════════════

#[test]
fn test_ir_literal_assignment_emits_move() {
    let gen = raw_ir("fn f() -> void { int x = 5; }");
    let all_insts: Vec<&IRInstruction> = gen.blocks.values()
        .flat_map(|bb| bb.instructions.iter())
        .collect();
    let has_move = all_insts.iter().any(|i| matches!(i, IRInstruction::Move { .. }));
    assert!(has_move, "Variable initialisation must emit a MOVE instruction; got: {:?}", all_insts);
}

#[test]
fn test_ir_addition_emits_add() {
    let gen = raw_ir("fn f() -> void { int a = 1; int b = 2; int c = a + b; }");
    let all_insts: Vec<&IRInstruction> = gen.blocks.values()
        .flat_map(|bb| bb.instructions.iter())
        .collect();
    let has_add = all_insts.iter().any(|i| matches!(i, IRInstruction::Add { .. }));
    assert!(has_add, "Addition expression must emit an ADD instruction; insts: {:?}", all_insts);
}

#[test]
fn test_ir_subtraction_emits_sub() {
    let gen = raw_ir("fn f() -> void { int a = 10; int b = 3; int c = a - b; }");
    let all: Vec<&IRInstruction> = gen.blocks.values()
        .flat_map(|bb| bb.instructions.iter()).collect();
    assert!(all.iter().any(|i| matches!(i, IRInstruction::Sub { .. })),
        "Subtraction must emit SUB");
}

#[test]
fn test_ir_multiplication_emits_mul() {
    let gen = raw_ir("fn f() -> void { int a = 2; int b = 4; int c = a * b; }");
    let all: Vec<&IRInstruction> = gen.blocks.values()
        .flat_map(|bb| bb.instructions.iter()).collect();
    assert!(all.iter().any(|i| matches!(i, IRInstruction::Mul { .. })),
        "Multiplication must emit MUL");
}

#[test]
fn test_ir_division_emits_div() {
    let gen = raw_ir("fn f() -> void { int a = 8; int b = 2; int c = a / b; }");
    let all: Vec<&IRInstruction> = gen.blocks.values()
        .flat_map(|bb| bb.instructions.iter()).collect();
    assert!(all.iter().any(|i| matches!(i, IRInstruction::Div { .. })),
        "Division must emit DIV");
}

#[test]
fn test_ir_function_call_emits_param_and_call() {
    let gen = raw_ir(
        "fn add(int a, int b) -> int { return a; } \
         fn main() -> void { int r = add(1, 2); }"
    );
    let all: Vec<&IRInstruction> = gen.blocks.values()
        .flat_map(|bb| bb.instructions.iter()).collect();
    let has_param = all.iter().any(|i| matches!(i, IRInstruction::Param { .. }));
    let has_call  = all.iter().any(|i| matches!(i, IRInstruction::Call { .. }));
    assert!(has_param, "Function call must emit PARAM instructions; insts: {:?}", all);
    assert!(has_call,  "Function call must emit a CALL instruction; insts: {:?}", all);
}

// ════════════════════════════════════════════════════════════
// GEN-3 — Statement Translation
// ════════════════════════════════════════════════════════════

#[test]
fn test_ir_return_emits_return_instruction() {
    let gen = raw_ir("fn f() -> int { return 42; }");
    let all: Vec<&IRInstruction> = gen.blocks.values()
        .flat_map(|bb| bb.instructions.iter()).collect();
    assert!(all.iter().any(|i| matches!(i, IRInstruction::Return { .. })),
        "Return statement must emit RETURN instruction");
}

#[test]
fn test_ir_if_emits_conditional_jump() {
    let gen = raw_ir(
        "fn f() -> void { \
           int x = 5; \
           if (x > 3) { int y = x; } \
         }"
    );
    let all: Vec<&IRInstruction> = gen.blocks.values()
        .flat_map(|bb| bb.instructions.iter()).collect();
    let has_cond_jump = all.iter().any(|i| {
        matches!(i, IRInstruction::JumpIfFalse { .. } | IRInstruction::JumpIfTrue { .. })
    });
    assert!(has_cond_jump, "If statement must produce a conditional jump; insts: {:?}", all);
}

#[test]
fn test_ir_if_creates_branch_blocks() {
    let gen = raw_ir(
        "fn f() -> void { \
           int x = 5; \
           if (x > 3) { int y = x; } \
         }"
    );
    // Expect at least 3 blocks: func_f, then_*, end_if_*
    assert!(
        gen.blocks.len() >= 3,
        "If statement must produce at least 3 basic blocks; got {:?}", gen.blocks.keys().collect::<Vec<_>>()
    );
}

#[test]
fn test_ir_if_else_creates_branch_blocks() {
    let gen = raw_ir(
        "fn f() -> void { \
           int x = 5; \
           if (x > 3) { int y = 1; } else { int y = 2; } \
         }"
    );
    let has_else = gen.blocks.keys().any(|k| k.contains("else"));
    assert!(has_else, "If-else must produce an else block; got {:?}", gen.blocks.keys().collect::<Vec<_>>());
}

#[test]
fn test_ir_assignment_emits_move() {
    let gen = raw_ir("fn f() -> void { int x = 0; x = 99; }");
    let all: Vec<&IRInstruction> = gen.blocks.values()
        .flat_map(|bb| bb.instructions.iter()).collect();
    // count MOVE instructions — should be at least 2 (init + assignment)
    let move_count = all.iter().filter(|i| matches!(i, IRInstruction::Move { .. })).count();
    assert!(move_count >= 2, "Two MOVE instructions expected (init + assign); got {}", move_count);
}

// ════════════════════════════════════════════════════════════
// GEN-5 / SSA — SSA Versioning Tests
// ════════════════════════════════════════════════════════════

#[test]
fn test_ssa_variables_get_versioned() {
    let ir_text = ir_for("fn f() -> void { int x = 5; }");
    // After SSA, 'x' must appear versioned as 'x_N' (e.g., x_1)
    assert!(
        ir_text.contains("x_"),
        "Variables must be versioned in SSA form (e.g. x_1); IR:\n{}", ir_text
    );
}

#[test]
fn test_ssa_temporaries_get_versioned() {
    let ir_text = ir_for("fn f() -> void { int a = 2; int b = 3; int c = a + b; }");
    // Temporaries like t1_1 must appear
    assert!(
        ir_text.contains("t") && (ir_text.contains("_1") || ir_text.contains("_2")),
        "Temporaries must be SSA-versioned; IR:\n{}", ir_text
    );
}

#[test]
fn test_ssa_each_var_assigned_once_per_version() {
    // x is assigned twice in source → two different SSA versions expected
    let ir_text = ir_for("fn f() -> void { int x = 1; x = 2; }");
    // x_1 for init, x_2 for reassignment
    assert!(
        ir_text.contains("x_1") && ir_text.contains("x_2"),
        "Two assignments to x must produce x_1 and x_2 in SSA; IR:\n{}", ir_text
    );
}

// ════════════════════════════════════════════════════════════
// OUT-1 — IR Text Format
// ════════════════════════════════════════════════════════════

#[test]
fn test_ir_text_output_has_block_label() {
    let ir_text = ir_for("fn main() -> void { }");
    // Every basic block dump must contain its label followed by ':'
    assert!(
        ir_text.contains(':'),
        "IR text must contain block labels (e.g., 'func_main:'); IR:\n{}", ir_text
    );
}

#[test]
fn test_ir_text_output_is_nonempty_for_function() {
    let ir_text = ir_for("fn main() -> void { int x = 1; }");
    assert!(!ir_text.trim().is_empty(), "IR output must not be empty for non-empty function");
}

#[test]
fn test_ir_instruction_display_add() {
    let inst = IRInstruction::Add {
        result: Operand::Temp { id: 1, version: 1 },
        left:   Operand::Var  { name: "x".into(), version: 1 },
        right:  Operand::Literal { value: "2".into() },
    };
    let s = inst.to_string();
    assert!(s.contains("ADD"), "ADD instruction display must contain 'ADD': {}", s);
    assert!(s.contains("x_1"), "Versioned var must appear in display: {}", s);
}

#[test]
fn test_ir_instruction_display_phi() {
    let inst = IRInstruction::Phi {
        result: Operand::Var { name: "x".into(), version: 3 },
        sources: vec![
            (Operand::Var { name: "x".into(), version: 1 }, "then_1".into()),
            (Operand::Var { name: "x".into(), version: 2 }, "else_1".into()),
        ],
    };
    let s = inst.to_string();
    assert!(s.contains("PHI"), "PHI instruction must render 'PHI': {}", s);
    assert!(s.contains("then_1"), "PHI must show source block labels: {}", s);
    assert!(s.contains("else_1"), "PHI must show source block labels: {}", s);
}

#[test]
fn test_ir_instruction_display_return() {
    let inst = IRInstruction::Return { value: Some(Operand::Literal { value: "0".into() }) };
    let s = inst.to_string();
    assert!(s.contains("RETURN"), "RETURN must appear in display: {}", s);
}

// ════════════════════════════════════════════════════════════
// Integration — full pipeline source → SSA IR
// ════════════════════════════════════════════════════════════

#[test]
fn test_ir_integration_factorial() {
    let src = r#"
        fn factorial(int n) -> int {
            if (n <= 1) {
                return 1;
            }
            return n;
        }
        fn main() -> void {
            int result = factorial(5);
        }
    "#;
    let ir_text = ir_for(src);
    assert!(!ir_text.is_empty(), "IR must be generated for factorial");
    // Check factorial gets its own function block
    assert!(ir_text.contains("factorial"), "IR must reference 'factorial' block; IR:\n{}", ir_text);
}

#[test]
fn test_ir_integration_multiple_expressions() {
    let ir_text = ir_for(
        "fn f() -> void { \
           int a = 1; \
           int b = 2; \
           int c = a + b; \
           int d = c * a; \
         }"
    );
    // Must have both ADD and MUL
    assert!(ir_text.contains("ADD"), "IR must have ADD; IR:\n{}", ir_text);
    assert!(ir_text.contains("MUL"), "IR must have MUL; IR:\n{}", ir_text);
}

// ════════════════════════════════════════════════════════════
// Multi-function SSA Tests
// ════════════════════════════════════════════════════════════

#[test]
fn test_ir_multiple_functions_mixed() {
    let src = r#"
        fn square(int n) -> int { return n * n; }
        fn log(int val) -> void { }
        fn main() -> void { log(square(2)); }
    "#;
    let gen = raw_ir(src);
    
    let has_square = gen.blocks.keys().any(|k| k.contains("square"));
    let has_log = gen.blocks.keys().any(|k| k.contains("log"));
    let has_main = gen.blocks.keys().any(|k| k.contains("main"));
    
    assert!(has_square, "Block for 'square' must exist");
    assert!(has_log, "Block for 'log' must exist");
    assert!(has_main, "Block for 'main' must exist");
}

#[test]
fn test_ir_complex_math_ssa() {
    let ir_text = ir_for(r#"
        fn main() -> void {
            int a = 10;
            int b = 20;
            int c = 30;
            int res = (a + b) * c - (b / a) + (a % 3);
        }
    "#);
    assert!(ir_text.contains("ADD"), "Should contain ADD");
    assert!(ir_text.contains("MUL"), "Should contain MUL");
    assert!(ir_text.contains("SUB"), "Should contain SUB");
    assert!(ir_text.contains("DIV"), "Should contain DIV");
}

