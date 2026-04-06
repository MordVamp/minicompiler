use minicompiler::lexer::{Scanner, TokenType};
use std::fs;
use std::path::Path;

fn run_test_file(src_path: &Path, expected_path: &Path) {
    let source = fs::read_to_string(src_path).unwrap();
    let expected_output = fs::read_to_string(expected_path).unwrap_or_default();
    
    let mut scanner = Scanner::new(&source);
    let mut tokens = Vec::new();
    loop {
        let t = scanner.next_token();
        let is_eof = t.token_type == TokenType::EndOfFile;
        tokens.push(t);
        if is_eof {
            break;
        }
    }

    let actual_output: String = tokens
        .iter()
        .map(|t| t.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    if std::env::var("UPDATE_EXPECT").is_ok() {
        fs::write(expected_path, &actual_output).unwrap();
    }

    let expected_output = fs::read_to_string(expected_path).unwrap_or_default();

    let actual_lines: Vec<&str> = actual_output.trim().lines().collect();
    let expected_lines: Vec<&str> = expected_output.trim().lines().collect();

    assert_eq!(
        actual_lines, expected_lines,
        "Test failed for file {:?}\n\nActual:\n{}\n\nExpected:\n{}",
        src_path.file_name().unwrap(),
        actual_output,
        expected_output
    );
}

fn discover_and_run_tests(dir: &str) {
    let entries = fs::read_dir(dir).expect("Directory not found");
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("src") {
            let mut expected_path = path.clone();
            expected_path.set_extension("txt");
            
            run_test_file(&path, &expected_path);
        }
    }
}

#[test]
fn test_valid_files() {
    discover_and_run_tests("tests/lexer/valid");
}

#[test]
fn test_invalid_files() {
    discover_and_run_tests("tests/lexer/invalid");
}
