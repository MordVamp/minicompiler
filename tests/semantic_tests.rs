// ============================================================
// Sprint 3 — Semantic Analysis Tests
// Covers: symbol table, type checking, scoping, error recovery
// ============================================================

use minicompiler::lexer::{Scanner, TokenType};
use minicompiler::parser::Parser;
use minicompiler::semantic::analyzer::SemanticAnalyzer;

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

/// Returns (ok, errors) after running semantic analysis.
fn analyze(source: &str) -> (bool, Vec<String>) {
    let mut parser = Parser::new(tokens(source));
    let mut ast = parser.parse().expect("parser failed");
    let mut analyzer = SemanticAnalyzer::new();
    let ok = analyzer.analyze(&mut ast);
    let errors = analyzer.errors.iter().map(|e| e.to_string()).collect();
    (ok, errors)
}

/// Returns the symbol-table dump after analysis.
fn symbol_dump(source: &str) -> String {
    let mut parser = Parser::new(tokens(source));
    let mut ast = parser.parse().expect("parser failed");
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&mut ast);
    analyzer.symbol_table.dump()
}

// ════════════════════════════════════════════════════════════
// SYM — Symbol Table Tests
// ════════════════════════════════════════════════════════════

#[test]
fn test_sym_global_var_registered() {
    let dump = symbol_dump("int x = 1;");
    assert!(dump.contains("x"), "Global var 'x' must appear in symbol table:\n{}", dump);
}

#[test]
fn test_sym_function_registered() {
    let dump = symbol_dump("fn add(int a, int b) -> int { return a; }");
    assert!(dump.contains("add"), "Function 'add' must appear in symbol table:\n{}", dump);
}

#[test]
fn test_sym_duplicate_declaration_error() {
    let (ok, errors) = analyze("fn f() -> void { int x = 1; int x = 2; }");
    assert!(!ok, "Duplicate declaration must fail");
    assert!(
        errors.iter().any(|e| e.contains("already defined")),
        "Expected 'already defined' error, got: {:?}", errors
    );
}

#[test]
fn test_sym_lookup_nested_scope() {
    // variable declared outside if-block must still be visible inside
    let (ok, _) = analyze("fn f() -> void { int x = 0; if (x == 0) { int y = x; } }");
    // No undefined variable error expected
    assert!(ok, "Nested scope lookup should succeed");
}

#[test]
fn test_sym_scope_isolation() {
    // Variable declared inside a block must NOT be visible after the block.
    // We expect an error because 'inner' is used out of scope.
    let (ok, errors) = analyze(
        "fn f() -> void { \
           if (1 == 1) { int inner = 5; } \
           inner = inner + 1; \
         }"
    );
    assert!(!ok, "Out-of-scope variable use must be an error");
    assert!(
        errors.iter().any(|e| e.contains("inner")),
        "Error must mention 'inner', got: {:?}", errors
    );
}

// ════════════════════════════════════════════════════════════
// SEM — Type Checking Tests
// ════════════════════════════════════════════════════════════

#[test]
fn test_sem_compatible_assignment_int() {
    let (ok, errors) = analyze("fn f() -> void { int x = 42; }");
    assert!(ok, "int = int should succeed; errors: {:?}", errors);
}

#[test]
fn test_sem_type_mismatch_assignment() {
    let (ok, errors) = analyze(
        "fn f() -> void { \
           int x = 5; \
           float y = 3.0; \
           x = y; \
         }"
    );
    assert!(!ok, "float assigned to int must fail");
    assert!(
        errors.iter().any(|e| e.to_lowercase().contains("mismatch")),
        "Expected type mismatch error, got: {:?}", errors
    );
}

#[test]
fn test_sem_undeclared_variable() {
    let (ok, errors) = analyze("fn f() -> void { int x = ghost; }");
    assert!(!ok, "Undeclared variable must fail");
    assert!(
        errors.iter().any(|e| e.contains("ghost") || e.to_lowercase().contains("undefined")),
        "Expected 'undefined' error mentioning ghost, got: {:?}", errors
    );
}

#[test]
fn test_sem_binary_arithmetic_ok() {
    let (ok, errors) = analyze("fn f() -> void { int a = 2; int b = 3; int c = a + b; }");
    assert!(ok, "int + int must succeed; errors: {:?}", errors);
}

#[test]
fn test_sem_binary_type_mismatch() {
    let (ok, errors) = analyze(
        "fn f() -> void { \
           int a = 2; \
           float b = 3.0; \
           int c = a + b; \
         }"
    );
    assert!(!ok, "int + float must raise a type mismatch");
    assert!(
        errors.iter().any(|e| e.to_lowercase().contains("mismatch")),
        "Expected type mismatch, got: {:?}", errors
    );
}

#[test]
fn test_sem_comparison_yields_bool() {
    // The condition of an if must be bool; comparison produces bool — should be fine
    let (ok, errors) = analyze("fn f() -> void { int x = 5; if (x > 3) { } }");
    assert!(ok, "Comparison in if-condition must typecheck; errors: {:?}", errors);
}

#[test]
fn test_sem_if_condition_must_be_bool() {
    // int as condition (not bool) must error
    let (ok, errors) = analyze("fn f() -> void { int x = 1; if (x) { } }");
    assert!(!ok, "Non-bool condition must fail");
    assert!(
        errors.iter().any(|e| e.contains("bool") || e.to_lowercase().contains("condition")),
        "Expected bool condition error, got: {:?}", errors
    );
}

#[test]
fn test_sem_while_condition_must_be_bool() {
    let (ok, errors) = analyze("fn f() -> void { int i = 0; while (i) { } }");
    assert!(!ok, "Non-bool while condition must fail");
    assert!(
        errors.iter().any(|e| e.contains("bool") || e.to_lowercase().contains("condition")),
        "Expected bool condition error, got: {:?}", errors
    );
}

#[test]
fn test_sem_unary_minus_int_ok() {
    let (ok, errors) = analyze("fn f() -> void { int x = -5; }");
    assert!(ok, "Unary minus on int must pass; errors: {:?}", errors);
}

#[test]
fn test_sem_logical_not_ok() {
    let (ok, errors) = analyze("fn f() -> void { bool b = true; bool c = !b; }");
    assert!(ok, "Logical NOT on bool must pass; errors: {:?}", errors);
}

// ════════════════════════════════════════════════════════════
// SEM — Function Validation Tests
// ════════════════════════════════════════════════════════════

#[test]
fn test_sem_return_type_match() {
    let (ok, errors) = analyze("fn square(int n) -> int { return n; }");
    assert!(ok, "Matching return type must pass; errors: {:?}", errors);
}

#[test]
fn test_sem_return_type_mismatch() {
    let (ok, errors) = analyze("fn f() -> int { return true; }");
    assert!(!ok, "bool returned from int function must fail");
    assert!(
        errors.iter().any(|e| e.to_lowercase().contains("return") || e.to_lowercase().contains("mismatch")),
        "Expected return type mismatch error, got: {:?}", errors
    );
}

#[test]
fn test_sem_call_correct_arg_count() {
    let (ok, errors) = analyze(
        "fn add(int a, int b) -> int { return a; } \
         fn main() -> void { int r = add(1, 2); }"
    );
    assert!(ok, "Correct arg count must pass; errors: {:?}", errors);
}

#[test]
fn test_sem_call_too_few_args() {
    let (ok, errors) = analyze(
        "fn add(int a, int b) -> int { return a; } \
         fn main() -> void { int r = add(1); }"
    );
    assert!(!ok, "Too few arguments must fail");
    assert!(
        errors.iter().any(|e| e.contains("argument") || e.to_lowercase().contains("requires")),
        "Expected argument count error, got: {:?}", errors
    );
}

#[test]
fn test_sem_call_too_many_args() {
    let (ok, errors) = analyze(
        "fn id(int a) -> int { return a; } \
         fn main() -> void { int r = id(1, 2, 3); }"
    );
    assert!(!ok, "Too many arguments must fail");
    assert!(
        errors.iter().any(|e| e.contains("argument") || e.to_lowercase().contains("requires")),
        "Expected argument count error, got: {:?}", errors
    );
}

#[test]
fn test_sem_call_undefined_function() {
    let (ok, errors) = analyze("fn main() -> void { int r = mystery(1); }");
    assert!(!ok, "Undefined function must fail");
    assert!(
        errors.iter().any(|e| e.contains("mystery") || e.to_lowercase().contains("undefined")),
        "Expected undefined error mentioning mystery, got: {:?}", errors
    );
}

// ════════════════════════════════════════════════════════════
// ERR — Error Recovery (multiple errors in one pass)
// ════════════════════════════════════════════════════════════

#[test]
fn test_err_multiple_errors_collected() {
    // Both 'ghost1' and 'ghost2' are undeclared — both errors must surface
    let (ok, errors) = analyze(
        "fn f() -> void { \
           int x = ghost1; \
           int y = ghost2; \
         }"
    );
    assert!(!ok);
    assert!(
        errors.len() >= 2,
        "Both undeclared-var errors must be collected; got {} errors: {:?}",
        errors.len(), errors
    );
}

#[test]
fn test_err_multiple_type_mismatches() {
    // Three distinct type mismatch errors in one function:
    //   1. binary: int + float  → type mismatch
    //   2. assignment: float assigned to int variable → type mismatch
    //   3. return: bool returned from int function → return type mismatch
    // All three must be collected in a single pass (error recovery).
    let (ok, errors) = analyze(r#"
        fn compute(int a, float b) -> int {
            int bad_binary = a + b;
            int x = 0;
            float y = 1.0;
            x = y;
            return true;
        }
    "#);
    assert!(!ok, "Multiple type mismatches must fail");
    assert!(
        errors.len() >= 3,
        "All 3 type mismatch errors must be collected; got {} errors:\n  {}",
        errors.len(),
        errors.join("\n  ")
    );
    assert!(
        errors.iter().any(|e| e.to_lowercase().contains("mismatch")),
        "Expected at least one 'mismatch' error; got: {:?}", errors
    );
    assert!(
        errors.iter().any(|e| e.to_lowercase().contains("return")),
        "Expected a return type error; got: {:?}", errors
    );
}

#[test]
fn test_err_unreachable_code() {
    let (ok, errors) = analyze(
        "fn f() -> void { \
           return; \
           int x = 1; \
         }"
    );
    assert!(!ok, "Unreachable code after return must be an error");
    assert!(
        errors.iter().any(|e| e.to_lowercase().contains("unreachable")),
        "Expected 'unreachable' error, got: {:?}", errors
    );
}

#[test]
fn test_err_missing_return() {
    let (ok, errors) = analyze("fn f() -> int { int x = 1; }");
    assert!(!ok, "Missing return in int function must be an error");
    assert!(
        errors.iter().any(|e| e.to_lowercase().contains("missing return")),
        "Expected 'missing return' error, got: {:?}", errors
    );
}

// ════════════════════════════════════════════════════════════
// Integration — full pipeline lex → parse → semantic
// ════════════════════════════════════════════════════════════

#[test]
fn test_integration_factorial_semantic_ok() {
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
    let (ok, errors) = analyze(src);
    assert!(ok, "Factorial must pass semantic analysis; errors: {:?}", errors);
}

#[test]
fn test_integration_nested_scopes_ok() {
    let src = r#"
        fn f() -> void {
            int x = 1;
            if (x > 0) {
                int y = x + 1;
                if (y > 1) {
                    int z = y + x;
                }
            }
        }
    "#;
    let (ok, errors) = analyze(src);
    assert!(ok, "Nested scope test must pass; errors: {:?}", errors);
}

#[test]
fn test_integration_decorated_ast_has_type_info() {

    let src = "fn f() -> void { int x = 42; }";
    let mut parser = Parser::new(tokens(src));
    let mut ast = parser.parse().expect("parser failed");
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&mut ast);

    // Walk into the VarDecl initializer and check type_info was set
    for decl in &ast.declarations {
        if let minicompiler::parser::ast::DeclarationNode::FunctionDecl { body, .. } = decl {
            if let minicompiler::parser::ast::StatementNode::Block { statements, .. } = body.as_ref() {
                for stmt in statements {
                    if let minicompiler::parser::ast::StatementNode::VarDeclStmt { decl, .. } = stmt {
                        if let minicompiler::parser::ast::DeclarationNode::VarDecl { initializer: Some(init), .. } = decl {
                            assert!(
                                init.type_info().is_some(),
                                "Initializer expression must have type_info after semantic analysis"
                            );
                            assert_eq!(init.type_info().unwrap(), "int");
                        }
                    }
                }
            }
        }
    }
}
