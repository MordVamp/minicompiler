# MiniCompiler - Work Process & Project Structure

This document explains the technical workflow and organization of the MiniCompiler project.

## 1. Project Architecture

The compiler is organized into modular components:

- **Lexer (`src/lexer/`)**: Converts source code into a stream of tokens.
    - `scanner.rs`: The main scanning logic using a character-by-character approach.
    - `token.rs`: Definitions of `TokenType`, `Token`, and `LiteralValue`.
- **Parser (`src/parser/`)**: Analyzes the token stream and builds an Abstract Syntax Tree (AST).
    - `parser.rs`: Recursive descent parser implementing the formal grammar.
    - `ast.rs`: Data structure representing the syntax tree with Graphviz and JSON export support.
    - `symbol_table.rs`: Tracks identifiers, scopes, and types during parsing.
- **CLI (`src/main.rs`)**: Provides a user-friendly interface to run the lexer, parser, and tests.

## 2. Compilation Workflow

1.  **Lexical Analysis**: Source text is processed into `Token` objects. Each token tracks its lexeme, type, and source position (line/column).
2.  **Syntax Analysis (Parsing)**: The `Parser` consumes tokens using LL(1) recursive descent.
    - It builds an `ASTNode` hierarchy.
    - It maintains a `SymbolTable` to detect re-declarations and track scopes.
3.  **Visualization**: The AST can be exported to:
    - **Text**: Indented tree view.
    - **JSON**: Machine-readable format.
    - **DOT**: Graphviz format for visual mapping.

## 3. Grammar Adherence

The compiler follows a formal grammar adapted from **ANSI C**.
- **Expressions**: Standard precedence (Unary > Multiplicative > Additive > Relational > Equality > Logical > Assignment).
- **Control Flow**: Supports `if-else`, `while`, and `for` loops.
- **Functions**: Supports declarations with return types and parameters.

## 4. Development Process

- **Test-Driven**: Valid and invalid source files in `tests/` are used for regression testing.
- **Continuous Audit**: Requirements from each sprint are tracked in `sprintX.md` and verified against the implementation.
