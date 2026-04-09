use minicompiler::lexer::{Scanner, Token, TokenType};
use minicompiler::parser::Parser;
use pretty_assertions::assert_eq;

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn lex(source: &str) -> Vec<Token> {
    let mut scanner = Scanner::new(source);
    let mut tokens = Vec::new();
    loop {
        let token = scanner.next_token();
        let is_eof = token.token_type == TokenType::EndOfFile;
        tokens.push(token);
        if is_eof { break; }
    }
    tokens
}

fn parse(source: &str) -> String {
    let mut parser = Parser::new(lex(source));
    parser.parse().unwrap().to_pretty_string()
}

fn parse_err(source: &str) -> String {
    let mut parser = Parser::new(lex(source));
    parser.parse().unwrap_err()
}

// ─── Basic AST construction ────────────────────────────────────────────────────

#[test]
fn test_parse_empty() {
    assert_eq!(parse(""), "Program:\n");
}

#[test]
fn test_parse_var_decl() {
    let ast = parse("int x = 42;");
    assert!(ast.contains("VarDecl: int x = 42"), "AST:\n{}", ast);
}

#[test]
fn test_parse_binary_expression() {
    // 2 + 3 * 4 must give 2 + (3*4) by precedence
    let ast = parse("int x = 2 + 3 * 4;");
    assert!(ast.contains("(2 + (3 * 4))"), "AST:\n{}", ast);
}

#[test]
fn test_parse_unary_minus() {
    let ast = parse("int x = -5;");
    assert!(ast.contains("(-5)"), "AST:\n{}", ast);
}

#[test]
fn test_parse_logical_operators() {
    let ast = parse("fn f() -> void { if (a && b || c) { } }");
    assert!(ast.contains("If:"), "AST:\n{}", ast);
}

#[test]
fn test_parse_function_decl() {
    let src = r#"
        fn main() -> void {
            int x = 5;
            return x;
        }
    "#;
    let ast = parse(src);
    assert!(ast.contains("FunctionDecl: main -> void"), "AST:\n{}", ast);
    assert!(ast.contains("VarDecl: int x = 5"), "AST:\n{}", ast);
    assert!(ast.contains("Return: x"), "AST:\n{}", ast);
}

#[test]
fn test_parse_function_with_params() {
    let ast = parse("fn add(int a, int b) -> int { return a; }");
    assert!(ast.contains("FunctionDecl: add -> int"), "AST:\n{}", ast);
}

// ─── Increment / Decrement ─────────────────────────────────────────────────────

#[test]
fn test_parse_postfix_increment() {
    let ast = parse("fn f() -> void { int i = 0; i++; }");
    assert!(ast.contains("(i++)"), "Expected postfix ++ in AST:\n{}", ast);
}

#[test]
fn test_parse_postfix_decrement() {
    let ast = parse("fn f() -> void { int i = 0; i--; }");
    assert!(ast.contains("(i--)"), "Expected postfix -- in AST:\n{}", ast);
}

#[test]
fn test_parse_prefix_increment() {
    let ast = parse("fn f() -> void { int i = 0; ++i; }");
    assert!(ast.contains("(++i)"), "Expected prefix ++ in AST:\n{}", ast);
}

#[test]
fn test_parse_prefix_decrement() {
    let ast = parse("fn f() -> void { int i = 0; --i; }");
    assert!(ast.contains("(--i)"), "Expected prefix -- in AST:\n{}", ast);
}

// ─── Control flow ──────────────────────────────────────────────────────────────

#[test]
fn test_parse_if_else() {
    let src = "fn f() -> void { if (x > 0) { return x; } else { return 0; } }";
    let ast = parse(src);
    assert!(ast.contains("If:"), "AST:\n{}", ast);
    assert!(ast.contains("Else:"), "AST:\n{}", ast);
}

#[test]
fn test_parse_while_loop() {
    let ast = parse("fn f() -> void { while (i > 0) { i--; } }");
    assert!(ast.contains("While:"), "AST:\n{}", ast);
}

#[test]
fn test_parse_for_loop() {
    let ast = parse("fn f() -> void { for (int i = 0; i < 10; i++) { } }");
    assert!(ast.contains("For:"), "AST:\n{}", ast);
}

#[test]
fn test_parse_empty_statement() {
    let ast = parse("fn f() -> void { ; }");
    assert!(ast.contains("EmptyStmt"), "AST:\n{}", ast);
}

// ─── Struct declarations ───────────────────────────────────────────────────────

#[test]
fn test_parse_struct_decl() {
    let ast = parse("struct Point { int x; int y; }");
    assert!(ast.contains("StructDecl: Point"), "AST:\n{}", ast);
}

// ─── Function calls ────────────────────────────────────────────────────────────

#[test]
fn test_parse_function_call() {
    let ast = parse("fn f() -> void { int r = add(1, 2); }");
    assert!(ast.contains("add(1, 2)"), "AST:\n{}", ast);
}

#[test]
fn test_parse_nested_function_call() {
    let ast = parse("fn f() -> void { int r = foo(bar(1)); }");
    assert!(ast.contains("foo("), "AST:\n{}", ast);
}

// ─── Assignment operators ──────────────────────────────────────────────────────

#[test]
fn test_parse_compound_assignment() {
    let ast = parse("fn f() -> void { x += 5; }");
    assert!(ast.contains("(x += 5)"), "AST:\n{}", ast);
}

// ─── Error detection ───────────────────────────────────────────────────────────

#[test]
fn test_syntax_error_missing_semicolon() {
    let err = parse_err("fn f() -> void { int x = 5 }");
    assert!(err.contains("Syntax Error"), "Expected error, got: {}", err);
    // Error message must include line/column info
    assert!(err.contains("line") || err.contains("col"), "Error must include location: {}", err);
}

#[test]
fn test_syntax_error_missing_closing_paren() {
    let err = parse_err("fn f) -> void { }");
    assert!(err.contains("Syntax Error"), "Expected error, got: {}", err);
}

// ─── Integration: full lexer → parser pipeline ─────────────────────────────────
// These mirror the .src files in tests/parser/valid/

#[test]
fn test_integration_full_program() {
    let src = r#"
        fn factorial(int n) -> int {
            if (n <= 1) {
                return 1;
            }
            return n * factorial(n - 1);
        }
        fn main() -> void {
            int result = factorial(5);
            return result;
        }
    "#;
    let ast = parse(src);
    assert!(ast.contains("FunctionDecl: factorial -> int"), "AST:\n{}", ast);
    assert!(ast.contains("FunctionDecl: main -> void"), "AST:\n{}", ast);
    assert!(ast.contains("If:"), "AST:\n{}", ast);
    assert!(ast.contains("Return:"), "AST:\n{}", ast);
}

#[test]
fn test_integration_increments_in_function() {
    let src = r#"
        fn main() -> void {
            int i = 0;
            i++;
            ++i;
            i--;
            --i;
            return i;
        }
    "#;
    let ast = parse(src);
    assert!(ast.contains("(i++)"), "AST:\n{}", ast);
    assert!(ast.contains("(++i)"), "AST:\n{}", ast);
    assert!(ast.contains("(i--)"), "AST:\n{}", ast);
    assert!(ast.contains("(--i)"), "AST:\n{}", ast);
}

// ─── Symbol table ──────────────────────────────────────────────────────────────

#[test]
fn test_symbol_table_populated_with_function() {
    let mut parser = Parser::new(lex("fn main() -> void { int x = 5; }"));
    parser.parse().unwrap();
    let sym = parser.symbols.lookup("main");
    assert!(sym.is_some(), "main should be in symbol table");
    assert_eq!(sym.unwrap().data_type, "void");
}

#[test]
fn test_symbol_table_populated_with_variable() {
    // Variables are tracked within the parser during parse; look them up
    let mut parser = Parser::new(lex("int counter = 0;"));
    parser.parse().unwrap();
    let sym = parser.symbols.lookup("counter");
    assert!(sym.is_some(), "counter should be in symbol table");
    assert_eq!(sym.unwrap().data_type, "int");
}
