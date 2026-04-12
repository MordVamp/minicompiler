# MiniCompiler — CLI Reference

Complete reference for all commands and flags available in the MiniCompiler tool.

---

## Usage

```
cargo run -- <COMMAND> [OPTIONS]
```

Or, after `cargo build`:

```
./target/debug/minicompiler <COMMAND> [OPTIONS]
```

---

## Commands

### `lex` — Run the Lexer

Tokenizes a source file and prints (or saves) the token stream.

```
cargo run -- lex --input <FILE> [--output <FILE>]
```

| Flag | Short | Required | Description |
| :--- | :--- | :--- | :--- |
| `--input <FILE>` | `-i` | Yes | Path to the `.src` source file to scan |
| `--output <FILE>` | `-o` | No | Write token output to this file (default: stdout) |

**Output format** (one token per line):
```
LINE:COL  TOKEN_TYPE  "LEXEME"  [LITERAL_VALUE]
```

**Example:**
```bash
cargo run -- lex --input examples/hello.src
# Output:
# 1:1 KW_FN "fn"
# 1:4 IDENTIFIER "main"
# 1:8 LPAREN "("
# ...
```

---

### `parse` — Run the Parser

Parses a source file and outputs the Abstract Syntax Tree (AST).

```
cargo run -- parse --input <FILE> [--output <FILE>] [--ast-format <FORMAT>] [--verbose]
```

| Flag | Short | Required | Default | Description |
| :--- | :--- | :--- | :--- | :--- |
| `--input <FILE>` | `-i` | Yes | — | Path to the `.src` source file |
| `--output <FILE>` | `-o` | No | stdout | Write AST output to this file |
| `--ast-format <FORMAT>` | — | No | `text` | Output format: `text`, `dot`, or `json` |
| `--verbose` | `-v` | No | false | Print extra parsing information |

**Formats:**
- `text` — Human-readable, indented AST (default)
- `dot` — Graphviz DOT format (pipe into `dot -Tpng` to visualize)
- `json` — Machine-readable JSON (useful for tooling)

**Examples:**
```bash
# Pretty-print AST as text
cargo run -- parse --input examples/hello.src

# Generate Graphviz DOT file then render to PNG
cargo run -- parse --input examples/hello.src --ast-format dot --output ast.dot
dot -Tpng ast.dot -o ast.png

# Save AST as JSON
cargo run -- parse --input examples/hello.src --ast-format json --output ast.json

# Verbose mode
cargo run -- parse --input examples/hello.src --verbose
```

---

### `check` — Run Semantic Analysis (Sprint 3)

Validates the source file for type errors, undeclared variables, scope violations, etc.

```
cargo run -- check --input <FILE> [--verbose]
```

| Flag | Short | Required | Default | Description |
| :--- | :--- | :--- | :--- | :--- |
| `--input <FILE>` | `-i` | Yes | — | Path to the `.src` source file |
| `--verbose` | `-v` | No | false | Print symbol table dump on success |

**Exit codes:**
- `0` — No semantic errors found, prints `OK`
- `1` — Semantic errors found, prints error messages to stderr

**Examples:**
```bash
# Basic check
cargo run -- check --input examples/hello.src
# Output: OK

# Check with symbol table dump
cargo run -- check --input examples/hello.src --verbose
# Output:
#   Semantic analysis passed.
#   --- Symbol Table Dump ---
#   Scope level 0:
#     main : Function of type fn() -> void
#   -------------------------

# Check a file with errors (prints to stderr)
cargo run -- check --input examples/bad.src
# Stderr:
#   Semantic Error at line 3, column 10: Undefined variable 'x'
```

---

### `ir` — Generate Intermediate Representation (Sprint 4)

Runs the full pipeline (lex → parse → semantic check → IR) and outputs SSA-form IR code.

```
cargo run -- ir --input <FILE> [--output <FILE>] [--verbose]
```

| Flag | Short | Required | Default | Description |
| :--- | :--- | :--- | :--- | :--- |
| `--input <FILE>` | `-i` | Yes | — | Path to the `.src` source file |
| `--output <FILE>` | `-o` | No | stdout | Write IR output to this file |
| `--verbose` | `-v` | No | false | Print a success message on completion |

**Note:** If semantic analysis fails, IR generation is aborted and errors are printed.

**Examples:**
```bash
# Print IR to console
cargo run -- ir --input examples/hello.src

# Save IR to file
cargo run -- ir --input examples/hello.src --output output.ir

# Verbose mode
cargo run -- ir --input examples/hello.src --verbose
```

**IR output format:**
```
--- IR Code (SSA Form) ---
entry:

func_main:
  x_1 = MOVE 1
  t1_1 = ADD x_1, 2
  RETURN t1_1
```

---

### `dump` — Full Compiler Dump

Outputs everything: Tokens, AST, Symbol Table, and SSA-form IR. Useful for debugging and learning.

```
cargo run -- dump --input <FILE>
```

| Flag | Short | Required | Description |
| :--- | :--- | :--- | :--- |
| `--input <FILE>` | `-i` | Yes | Path to the `.src` source file |

**Output format:**
Sequentially prints the output of `lex`, `parse`, `check --verbose`, and `ir`.

**Example:**
```bash
cargo run -- dump --input examples/hello.src
```

---

### `test` — Run Tests (shortcut)

Prints a message directing you to use `cargo test`. Exists for CLI completeness.

```
cargo run -- test
```

For actual test execution, use:
```bash
cargo test
```

---

## Test Commands

| Command | Description |
| :--- | :--- |
| `cargo test` | Run all tests (all suites) |
| `cargo test --test semantic_golden` | Run only Sprint 3 golden file tests |
| `cargo test --test ir_golden` | Run only Sprint 4 golden file tests |
| `cargo test --test semantic_tests` | Run only Sprint 3 unit tests |
| `cargo test --test ir_tests` | Run only Sprint 4 unit tests |
| `cargo test --test test_runner` | Run only Sprint 1 lexer golden tests |
| `UPDATE_EXPECT=1 cargo test --test semantic_golden` | Regenerate Sprint 3 golden `.txt` files |
| `UPDATE_EXPECT=1 cargo test --test ir_golden` | Regenerate Sprint 4 golden `.txt` files |
