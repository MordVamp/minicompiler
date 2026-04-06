use minicompiler::lexer::{Scanner, TokenType};
use minicompiler::parser::Parser;
use pretty_assertions::assert_eq;

fn parse(source: &str) -> String {
    let mut scanner = Scanner::new(source);
    let mut tokens = Vec::new();
    loop {
        let token = scanner.next_token();
        let is_eof = token.token_type == TokenType::EndOfFile;
        tokens.push(token);
        if is_eof {
            break;
        }
    }
    let mut parser = Parser::new(tokens);
    parser.parse().unwrap().to_pretty_string()
}

#[test]
fn test_parse_empty() {
    let src = "";
    let ast = parse(src);
    assert_eq!(ast, "Program:\n");
}

#[test]
fn test_parse_var_decl() {
    let src = "int x = 42;";
    let ast = parse(src);
    assert!(ast.contains("VarDecl: int x = 42"));
}

#[test]
fn test_parse_binary_expression() {
    let src = "int x = 2 + 3 * 4;";
    let ast = parse(src);
    assert!(ast.contains("(2 + (3 * 4))"));
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
    assert!(ast.contains("FunctionDecl: main -> void"));
    assert!(ast.contains("VarDecl: int x = 5"));
    assert!(ast.contains("Return: x"));
}
