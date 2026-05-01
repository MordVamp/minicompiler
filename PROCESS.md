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
    - **Semantic (`src/semantic/`)**: Validates the AST for type safety and scope rules.
    - `analyzer.rs`: Implements type checking and constant folding.
    - `types.rs`: Formal type system definitions.
- **IR (`src/ir/`)**: Generates Single Static Assignment (SSA) Intermediate Representation.
    - `ir_generator.rs`: Translates AST to linear IR instructions.
    - `ssa_constructor.rs`: Performs versioning and PHI node insertion.
    - `ir_instructions.rs`: Definitions of the IR instruction set.
- **Codegen (`src/codegen/`)**: Translates IR into x86-64 assembly.
    - `x86_generator.rs`: Assembly emission following System V ABI.
    - `abi.rs`: Register usage and calling convention definitions.
    - `stack_frame.rs`: Stack alignment and frame allocation logic.
- **Runtime (`src/runtime/`)**: Minimal assembly library for system calls and I/O.
- **CLI (`src/main.rs`)**: Provides the unified interface for all stages, including full pipeline dumps and CFG visualization.

## 2. Compilation Workflow

1.  **Lexical Analysis**: Source text is processed into `Token` objects with line/column tracking.
2.  **Syntax Analysis (Parsing)**: The `Parser` builds the `ASTNode` hierarchy and initializes the `SymbolTable`.
3.  **Semantic Analysis**: The `SemanticAnalyzer` decorates the AST with type information, performs constant folding, and checks for semantic errors (e.g., type mismatches, undefined symbols).
4.  **IR Generation**: The `IRGenerator` lowers the AST into basic blocks.
5.  **SSA Construction**: The `SSAConstructor` transforms the IR into Static Single Assignment form, automatically inserting PHI nodes at control-flow join points.
6.  **Code Generation**: The `X86Generator` translates the SSA IR into x86-64 NASM assembly, handling function prologues, epilogues, and register-based parameter passing.
7.  **Visualization & Export**:
    - **AST**: Text, JSON, DOT formats.
    - **IR/CFG**: Text, DOT (Visualization).

## 3. Grammar Adherence

The compiler follows a formal grammar adapted from **ANSI C**.
- **Expressions**: Standard precedence (Unary > Multiplicative > Additive > Relational > Equality > Logical > Assignment).
- **Control Flow**: Supports `if-else`, `while`, and `for` loops.
- **Functions**: Supports declarations with return types and parameters.

## 4. Development Process

- **Test-Driven**: Valid and invalid source files in `tests/` are used for regression testing.
- **Continuous Audit**: Requirements from each sprint are tracked in `sprintX.md` and verified against the implementation.
