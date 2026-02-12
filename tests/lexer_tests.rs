use compiler::lexer::{LiteralValue, Scanner, Token, TokenType}; // Note: crate name is "compiler"
use pretty_assertions::assert_eq;

fn tokenize(source: &str) -> Vec<Token> {
    let mut scanner = Scanner::new(source);
    let mut tokens = Vec::new();
    loop {
        let t = scanner.next_token();
        if t.token_type == TokenType::EndOfFile {
            break;
        }
        tokens.push(t);
    }
    tokens
}

#[test]
fn test_keywords() {
    let src = "if else while for int float bool return true false void struct fn";
    let tokens = tokenize(src);
    let expected = vec![
        TokenType::If,
        TokenType::Else,
        TokenType::While,
        TokenType::For,
        TokenType::Int,
        TokenType::Float,
        TokenType::Bool,
        TokenType::Return,
        TokenType::True,
        TokenType::False,
        TokenType::Void,
        TokenType::Struct,
        TokenType::Fn,
    ];
    assert_eq!(tokens.len(), expected.len());
    for (token, exp_type) in tokens.iter().zip(expected) {
        assert_eq!(token.token_type, exp_type);
    }
}

#[test]
fn test_identifiers() {
    let src = "x _foo bar123 a_very_long_identifier_that_is_under_255_chars";
    let tokens = tokenize(src);
    assert_eq!(tokens.len(), 4);
    for token in tokens {
        assert_eq!(token.token_type, TokenType::Identifier);
        assert_eq!(token.literal, LiteralValue::None);
    }
}

#[test]
fn test_integer_literals() {
    let src = "0 42 2147483647 -2147483648";
    let tokens = tokenize(src);
    let expected_values = vec![0, 42, 2147483647, -2147483648];
    assert_eq!(tokens.len(), 4);
    for (token, &val) in tokens.iter().zip(expected_values.iter()) {
        assert_eq!(token.token_type, TokenType::IntLiteral);
        assert_eq!(token.literal, LiteralValue::Integer(val));
    }
}

#[test]
fn test_float_literals() {
    let src = "0.0 3.14 .5 10.";
    let tokens = tokenize(src);
    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].token_type, TokenType::FloatLiteral);
    assert_eq!(tokens[1].token_type, TokenType::FloatLiteral);
    assert_eq!(tokens[2].token_type, TokenType::Error);
    assert_eq!(tokens[3].token_type, TokenType::Error);
}

#[test]
fn test_string_literals() {
    let src = r#""hello" "world" ""#;
    let tokens = tokenize(src);
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].token_type, TokenType::StringLiteral);
    assert_eq!(
        tokens[0].literal,
        LiteralValue::String("hello".to_string())
    );
    assert_eq!(tokens[1].token_type, TokenType::StringLiteral);
    assert_eq!(
        tokens[1].literal,
        LiteralValue::String("world".to_string())
    );
    assert_eq!(tokens[2].token_type, TokenType::StringLiteral);
    assert_eq!(tokens[2].literal, LiteralValue::String("".to_string()));
}

#[test]
fn test_operators() {
    let src = "+ - * / % = == != < <= > >= && || ! += -= *= /=";
    let tokens = tokenize(src);
    let expected = vec![
        TokenType::Plus,
        TokenType::Minus,
        TokenType::Star,
        TokenType::Slash,
        TokenType::Percent,
        TokenType::Equal,
        TokenType::EqualEqual,
        TokenType::NotEqual,
        TokenType::Less,
        TokenType::LessEqual,
        TokenType::Greater,
        TokenType::GreaterEqual,
        TokenType::AndAnd,
        TokenType::OrOr,
        TokenType::Bang,
        TokenType::PlusEqual,
        TokenType::MinusEqual,
        TokenType::StarEqual,
        TokenType::SlashEqual,
    ];
    assert_eq!(tokens.len(), expected.len());
    for (token, exp_type) in tokens.iter().zip(expected) {
        assert_eq!(token.token_type, exp_type);
    }
}

#[test]
fn test_delimiters() {
    let src = "( ) { } [ ] ; , :";
    let tokens = tokenize(src);
    let expected = vec![
        TokenType::LParen,
        TokenType::RParen,
        TokenType::LBrace,
        TokenType::RBrace,
        TokenType::LBracket,
        TokenType::RBracket,
        TokenType::Semicolon,
        TokenType::Comma,
        TokenType::Colon,
    ];
    assert_eq!(tokens.len(), expected.len());
    for (token, exp_type) in tokens.iter().zip(expected) {
        assert_eq!(token.token_type, exp_type);
    }
}

#[test]
fn test_comments() {
    let src = r#"
    // single line comment
    int x = 5; // trailing comment
    /* block comment */
    /* nested /* block */ comment */
    "/* not a comment */"
    "#;
    let tokens = tokenize(src);
    assert_eq!(tokens.len(), 7);
    assert_eq!(tokens[0].token_type, TokenType::Int);
    assert_eq!(tokens[1].token_type, TokenType::Identifier);
    assert_eq!(tokens[1].lexeme, "x");
    assert_eq!(tokens[2].token_type, TokenType::Equal);
    assert_eq!(tokens[3].token_type, TokenType::IntLiteral);
    assert_eq!(tokens[3].literal, LiteralValue::Integer(5));
    assert_eq!(tokens[4].token_type, TokenType::Semicolon);
    assert_eq!(tokens[5].token_type, TokenType::StringLiteral);
    assert_eq!(
        tokens[5].literal,
        LiteralValue::String("/* not a comment */".to_string())
    );
    assert_eq!(tokens[6].token_type, TokenType::EndOfFile);
}

#[test]
fn test_invalid_characters() {
    let src = "@ $ #";
    let tokens = tokenize(src);
    assert_eq!(tokens.len(), 3);
    for token in tokens {
        assert_eq!(token.token_type, TokenType::Error);
    }
}

#[test]
fn test_unterminated_string() {
    let src = r#""hello"#;
    let tokens = tokenize(src);
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::Error);
    assert!(tokens[0].lexeme.contains("unterminated string"));
}

#[test]
fn test_unterminated_comment() {
    let src = "/* comment never ends";
    let mut scanner = Scanner::new(src);
    let token = scanner.next_token();
    assert_eq!(token.token_type, TokenType::EndOfFile);
}

#[test]
fn test_long_identifier() {
    let long_id = "a".repeat(300);
    let tokens = tokenize(&long_id);
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::Error);
    assert!(tokens[0].lexeme.contains("Malformed number"));
}

#[test]
fn test_integer_out_of_range() {
    let src = "2147483648";
    let tokens = tokenize(src);
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::Error);
    assert!(tokens[0].lexeme.contains("Integer out of range"));
}

#[test]
fn test_position_tracking() {
    let src = "if x\n123";
    let mut scanner = Scanner::new(src);
    let tok1 = scanner.next_token();
    assert_eq!(tok1.line, 1);
    assert_eq!(tok1.column, 1);
    let tok2 = scanner.next_token();
    assert_eq!(tok2.line, 1);
    assert_eq!(tok2.column, 4);
    let tok3 = scanner.next_token();
    assert_eq!(tok3.line, 2);
    assert_eq!(tok3.column, 1);
}