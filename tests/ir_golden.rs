// ============================================================
// Sprint 4 — IR Generation: Golden File Tests
//
// Structure (per sprint4.md TEST-2 / TEST-3):
//   tests/ir/generation/*/    → golden = SSA IR text dump
//   tests/ir/validation/*/    → golden = SSA IR text dump + structural assertions
//
// Regenerate golden files:
//   UPDATE_EXPECT=1 cargo test --test ir_golden
// ============================================================

use minicompiler::lexer::{Scanner, TokenType};
use minicompiler::parser::Parser;
use minicompiler::semantic::analyzer::SemanticAnalyzer;
use minicompiler::ir::ir_generator::IRGenerator;
use minicompiler::ir::ssa_constructor::SSAConstructor;
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

/// Full pipeline: lex → parse → semantic → IR → SSA.
/// Returns the sorted SSA IR text dump.
fn run_ir(source: &str) -> String {
    let mut parser = Parser::new(tokens(source));
    let mut ast = parser.parse().expect("parser failed in golden IR test");
    let mut analyzer = SemanticAnalyzer::new();
    let ok = analyzer.analyze(&mut ast);
    assert!(ok, "Semantic errors in golden IR test: {:?}", analyzer.errors);

    let mut ir_gen = IRGenerator::new();
    ir_gen.generate(&ast);

    let mut ssa = SSAConstructor::new(ir_gen.blocks);
    ssa.construct();

    let mut keys: Vec<String> = ssa.blocks.keys().cloned().collect();
    keys.sort();

    let mut out = String::new();
    for k in &keys {
        out.push_str(&ssa.blocks[k].to_string());
        out.push('\n');
    }
    out
}

// ── Golden runner ─────────────────────────────────────────────

fn run_golden(src_path: &Path, expected_path: &Path) {
    let source = fs::read_to_string(src_path)
        .unwrap_or_else(|_| panic!("Cannot read source {:?}", src_path));

    let actual_output = run_ir(&source);

    // Structural assertions — every IR dump must contain a block label and be non-empty
    assert!(
        !actual_output.trim().is_empty(),
        "IR output must not be empty for {:?}",
        src_path.file_name().unwrap()
    );
    assert!(
        actual_output.contains(':'),
        "IR output must contain block labels for {:?}",
        src_path.file_name().unwrap()
    );

    if std::env::var("UPDATE_EXPECT").is_ok() {
        fs::write(expected_path, &actual_output)
            .unwrap_or_else(|_| panic!("Cannot write golden {:?}", expected_path));
        return;
    }

    let expected_output = fs::read_to_string(expected_path).unwrap_or_else(|_| {
        panic!(
            "Missing golden file {:?}. Run with UPDATE_EXPECT=1 to generate it.",
            expected_path
        )
    });

    let actual_lines: Vec<&str> = actual_output.trim().lines().collect();
    let expected_lines: Vec<&str> = expected_output.trim().lines().collect();

    assert_eq!(
        actual_lines,
        expected_lines,
        "IR golden mismatch for {:?}\n\n=== ACTUAL ===\n{}\n\n=== EXPECTED ===\n{}",
        src_path.file_name().unwrap(),
        actual_output,
        expected_output
    );
}

/// Recursively collect all `.src` paths under `dir`.
fn walkdir(dir: &str) -> Vec<std::path::PathBuf> {
    let mut results = Vec::new();
    let rd = match fs::read_dir(dir) {
        Ok(rd) => rd,
        Err(_) => return results,
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

fn discover_and_run(dir: &str) {
    let entries = walkdir(dir);
    let mut ran = 0;
    for src_path in entries {
        let mut expected_path = src_path.clone();
        expected_path.set_extension("txt");
        run_golden(&src_path, &expected_path);
        ran += 1;
    }
    assert!(ran > 0, "No .src files found in '{}' — check test directory structure.", dir);
}

// ════════════════════════════════════════════════════════════
// TEST-2 / TEST-3 (sprint4.md): Golden tests
// ════════════════════════════════════════════════════════════

/// Golden tests for IR generation (expressions, control_flow, functions, integration).
#[test]
fn test_ir_golden_generation() {
    discover_and_run("tests/ir/generation");
}

/// Golden tests for IR validation (structural_checks, type_consistency).
#[test]
fn test_ir_golden_validation() {
    discover_and_run("tests/ir/validation");
}
