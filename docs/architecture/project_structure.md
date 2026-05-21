# MiniCompiler Project Structure

This document provides a high-level overview of the MiniCompiler repository. It is designed to help LLMs and new developers rapidly understand the codebase architecture without needing to perform deep or recursive directory scans.

## Compilation Pipeline (`src/`)

The compiler implements a classic multi-pass architecture:

1. **Lexical Analysis (`src/lexer/`)**
   - **Purpose**: Converts the raw source code string into a stream of tokens.
   - **Key Files**: `scanner.rs` (the actual lexer logic), `token.rs`.

2. **Syntax Analysis (`src/parser/`)**
   - **Purpose**: Parses tokens into an Abstract Syntax Tree (AST) using recursive descent.
   - **Key Files**: `parser.rs`, `ast.rs`.

3. **Semantic Analysis (`src/semantic/`)**
   - **Purpose**: Validates the AST (Type checking, variable declarations, scope shadowing).
   - **Key Files**: `analyzer.rs`, `symbol_table.rs`.

4. **Intermediate Representation & Middle-end (`src/ir/`)**
   - **Purpose**: Lowers the AST into a flat Intermediate Representation (IR) composed of Basic Blocks.
   - **Key Files**: 
     - `ir_generator.rs`: Translates AST to IR instructions.
     - `ssa_constructor.rs`: Builds the Control Flow Graph (CFG) and handles basic block connections and minimal Phi node definitions.
     - `optimizer.rs`: Executes IR-level optimizations (Dead Code Elimination, Sparse Conditional Constant Propagation).

5. **Code Generation & Backend (`src/codegen/`)**
   - **Purpose**: Converts IR directly into Linux x86-64 assembly.
   - **Key Files**:
     - `x86_generator.rs`: The transpiler backend. Uses a robust Reverse Post-Order (RPO) DFS algorithm to correctly linearize conditional blocks (`if`, `while`, `for`) to ensure proper assembly fall-through.
     - `optimizer.rs`: Implements post-generation assembly passes (Peephole Optimizer and Dead Store Elimination) to clean up stack access (`[rbp-X]`) and remove redundant instructions.

## Directory Layout

* **`src/`**: The core compiler source code (Rust).
* **`examples/`**: Programs written in the compiler's source language (`.src`) and their generated assembly (`.asm`).
  * `sprint6/`: Control flow examples (nested loops, if-else).
  * `sprint7/`: Memory manipulation (arrays, malloc, free, printf).
  * `optimizations/`: Examples designed to trigger SCCP and DSE optimizations.
* **`tests/`**: Integration tests.
  * Contains `sprints_asm_test.rs`, an automated test that programmatically runs the full pipeline on `examples/` to verify structural assembly correctness.
* **`doc/`**: Documentation artifacts.

## CLI Usage

Run the compiler via Cargo:

```bash
# Compile a file with optimizations enabled
cargo run -- compile --input examples/sprint6_7.src -O --output out.asm

# Run tests
cargo test
```
