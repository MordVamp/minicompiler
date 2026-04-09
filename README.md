# MiniCompiler

MiniCompiler is a lexical and syntax analyzer for a C-like language, implemented in Rust. It transforms source code into an Abstract Syntax Tree (AST) and provides visualization tools.

## Features

- **Lexical Analysis**: Robust scanner following ANSI C lexical rules.
- **Syntax Analysis**: Recursive descent parser building a comprehensive AST.
- **Symbol Table**: Tracks identifier declarations, scopes, and types.
- **Visualization**: Export AST to Text, JSON, and Graphviz (DOT) formats.
- **Error Handling**: Detailed error reporting with line/column tracking and recovery.
- **Comprehensive Testing**: Automated test suite for lexer and parser logic.

## Project Structure

- `src/lexer/`: Lexer implementation (Scanner, Tokens).
- `src/parser/`: Parser implementation (AST, Symbol Table, Parser logic).
- `tests/`: Integration and unit tests.
- `docs/`: Formal grammar and language specifications.
- `PROCESS.md`: Detailed explanation of the work process and architecture.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/) (Cargo)
- [Graphviz](https://graphviz.org/) (optional, for AST visualization)

### Build and Run

```bash
# Build the project
cargo build

# Run the lexer
cargo run -- lex --input examples/hello.src

# Run the parser (outputs pretty-printed AST)
cargo run -- parse --input examples/hello.src

# Generate Graphviz visualization
cargo run -- parse --input examples/hello.src --ast-format dot --output ast.dot
dot -Tpng ast.dot -o ast.png
```

### Running Tests

```bash
# Run all automated tests
cargo test
```

## Work Process

For a detailed look at how the MiniCompiler works, see [PROCESS.md](file:///c:/Prog/Rust/minicompiler_2/PROCESS.md).
