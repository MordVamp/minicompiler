// ============================================================
// Sprint 3 — Semantic Analysis: Golden File Tests
//
// Structure (per sprint3.md TEST-2 / TEST-3):
//   tests/semantic/valid/*/   → source must pass, golden = symbol table dump
//   tests/semantic/invalid/*/ → source must fail, golden = error messages
//
// Regenerate golden files:
//   UPDATE_EXPECT=1 cargo test --test semantic_golden
// ============================================================

use minicompiler::lexer::{Scanner, TokenType};
use minicompiler::parser::Parser;
use minicompiler::semantic::analyzer::SemanticAnalyzer;
use std::fs;
use std::path::Path;

// ── Pipeline helpers ──────────────────────────────────────────

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

/// Run full semantic analysis on `source`.
/// Returns (ok, symbol_table_dump, error_lines).
fn run_semantic(source: &str) -> (bool, String, Vec<String>) {
    let mut parser = Parser::new(tokens(source));
    let mut ast = parser.parse().expect("parser failed in golden test");
    let mut analyzer = SemanticAnalyzer::new();
    let ok = analyzer.analyze(&mut ast);
    let dump = analyzer.symbol_table.dump();
    let errors: Vec<String> = analyzer.errors.iter().map(|e| e.to_string()).collect();
    (ok, dump, errors)
}

// ── Golden runner ─────────────────────────────────────────────

/// Run a single golden test.
///
/// For **valid** files: the program must pass, actual output = symbol table dump.
/// For **invalid** files: the program must fail, actual output = error messages (one per line).
fn run_golden(src_path: &Path, expected_path: &Path, expect_valid: bool) {
    let source = fs::read_to_string(src_path)
        .unwrap_or_else(|_| panic!("Cannot read source {:?}", src_path));

    let (ok, dump, errors) = run_semantic(&source);

    let actual_output: String = if expect_valid {
        assert!(
            ok,
            "Golden valid test FAILED for {:?}: expected no errors, got:\n  {}",
            src_path.file_name().unwrap(),
            errors.join("\n  ")
        );
        dump
    } else {
        assert!(
            !ok,
            "Golden invalid test FAILED for {:?}: expected errors but got none",
            src_path.file_name().unwrap()
        );
        errors.join("\n")
    };

    if std::env::var("UPDATE_EXPECT").is_ok() {
        fs::write(expected_path, &actual_output)
            .unwrap_or_else(|_| panic!("Cannot write golden {:?}", expected_path));
        return; // do not compare when updating
    }

    let expected_output = fs::read_to_string(expected_path).unwrap_or_else(|_| {
        panic!(
            "Missing golden file {:?}. Run with UPDATE_EXPECT=1 to generate it.",
            expected_path
        )
    });

    // Sort entries within each scope block so HashMap ordering doesn't cause flakiness.
    let actual_lines = normalize_dump(&actual_output);
    let expected_lines = normalize_dump(&expected_output);

    assert_eq!(
        actual_lines,
        expected_lines,
        "Golden mismatch for {:?}\n\n=== ACTUAL ===\n{}\n\n=== EXPECTED ===\n{}",
        src_path.file_name().unwrap(),
        actual_output,
        expected_output
    );
}

/// Normalize a symbol table dump so that symbol entries within each scope block
/// are sorted alphabetically. This makes the comparison stable regardless of
/// the HashMap iteration order used internally by SymbolTable.
fn normalize_dump(text: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut current_scope_entries: Vec<String> = Vec::new();
    let mut in_scope = false;

    for line in text.trim().lines() {
        if line.contains("Scope") && line.ends_with(':') {
            // Flush any accumulated entries from the previous scope block.
            if in_scope {
                current_scope_entries.sort();
                result.extend(current_scope_entries.drain(..));
            }
            result.push(line.to_string());
            in_scope = true;
        } else if line.starts_with("---") || line.starts_with("-") {
            // Separator line — flush and reset.
            if in_scope {
                current_scope_entries.sort();
                result.extend(current_scope_entries.drain(..));
                in_scope = false;
            }
            result.push(line.to_string());
        } else if in_scope && line.starts_with("  ") {
            // Symbol entry inside a scope block — buffer for sorting.
            current_scope_entries.push(line.to_string());
        } else {
            result.push(line.to_string());
        }
    }
    // Final flush.
    if in_scope {
        current_scope_entries.sort();
        result.extend(current_scope_entries);
    }
    result
}

/// Walk a directory, run golden test for every `.src` / `.txt` pair.
fn discover_and_run(dir: &str, expect_valid: bool) {
    // Walk recursively through subdirectories
    let entries = walkdir(dir);
    let mut ran = 0;
    for src_path in entries {
        let mut expected_path = src_path.clone();
        expected_path.set_extension("txt");
        run_golden(&src_path, &expected_path, expect_valid);
        ran += 1;
    }
    assert!(ran > 0, "No .src files found in '{}' — check test directory structure.", dir);
}

/// Recursively collect all `.src` paths under `dir`.
fn walkdir(dir: &str) -> Vec<std::path::PathBuf> {
    let mut results = Vec::new();
    let rd = match fs::read_dir(dir) {
        Ok(rd) => rd,
        Err(_) => return results, // directory may not exist yet
    };
    for entry in rd.flatten() {
        let path = entry.path();
        if path.is_dir() {
            results.extend(walkdir(path.to_str().unwrap()));
        } else if path.extension().and_then(|s| s.to_str()) == Some("src") {
            results.push(path);
        }
    }
    results.sort();
    results
}

// ════════════════════════════════════════════════════════════
// TEST-2 / TEST-3 (sprint3.md): Golden tests
// ════════════════════════════════════════════════════════════

#[test]
fn test_semantic_golden_valid() {
    discover_and_run("tests/semantic/valid", true);
}

#[test]
fn test_semantic_golden_invalid() {
    discover_and_run("tests/semantic/invalid", false);
}
