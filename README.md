We need to create two missing documentation files as per the project requirements:

1. **README.md** – project overview, build instructions, quick start.
2. **docs/language_spec.md** – formal language specification in EBNF.

Below are the complete contents for each file. Place them in the root directory (`README.md`) and in the `docs/` folder (`docs/language_spec.md`). Adjust team names or details as needed.

---

## `README.md`

```markdown
# MiniCompiler

A simple compiler for a C-like language, developed as part of a compiler construction course.  
This project implements the lexical analyzer (scanner) for Sprint 1.

## Team

- [Your Name]
- [Your Name]

## Project Description

MiniCompiler translates a small, statically-typed language into an intermediate representation (future sprints).  
The lexical analyzer converts source code into a stream of tokens with position information, according to the language specification in [`docs/language_spec.md`](docs/language_spec.md).

## Build Instructions

The project is written in Rust and uses Cargo. Ensure you have Rust installed (https://rustup.rs/).

- **Build the project**  
  ```bash
  cargo build --release
  ```

- **Run the lexer on a source file**  
  ```bash
  cargo run -- lex --input examples/hello.src --output tokens.txt
  ```
  If no output file is given, tokens are printed to stdout.

- **Run all tests**  
  ```bash
  cargo test
  ```

## Quick Start

1. Create a file `hello.src` with the following content:

   ```
   fn main() {
       int x = 42;
       return x;
   }
   ```

2. Run the lexer:

   ```bash
   cargo run -- lex --input hello.src
   ```

3. The output will be a list of tokens in the format:

   ```
   LINE:COLUMN TOKEN_TYPE "LEXEME" [LITERAL_VALUE]
   ```

   Example output:

   ```
   1:1 Fn "fn"
   1:4 Identifier "main"
   1:8 LParen "("
   1:9 RParen ")"
   1:10 LBrace "{"
   2:5 Int "int"
   2:9 Identifier "x"
   2:11 Equal "="
   2:13 IntLiteral "42" 42
   2:16 Semicolon ";"
   3:5 Return "return"
   3:12 Identifier "x"
   3:13 Semicolon ";"
   4:1 RBrace "}"
   5:1 EndOfFile ""
   ```

## Language Specification

The full language specification can be found in [docs/language_spec.md](docs/language_spec.md).


