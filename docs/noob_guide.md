# MiniCompiler — Noob Guide
### "I've never heard the word 'compiler' before" edition

---

## What is this thing?

A **compiler** is a program that reads code written by a human (like `int x = 5;`) and
translates it into something a computer can actually run. Your browser, your operating
system, every app on your phone — all produced by a compiler.

This project is a **mini-version** of a compiler. It doesn't produce runnable programs yet,
but it does all the hard thinking work: reading your code, checking if it makes sense, and
translating it into an internal "blueprint" (called IR — Intermediate Representation).

Think of it like this:

```
Your source code (.src file)
        │
        ▼
   [PHASE 1: LEXER]   — Reads every character, cuts text into "words" called TOKENS
        │
        ▼
   [PHASE 2: PARSER]  — Arranges tokens into a tree (like a sentence diagram in school)
        │
        ▼
   [PHASE 3: CHECKER] — Makes sure the code actually makes sense (types, variables, etc.)
        │
        ▼
   [PHASE 4: IR GEN]  — Translates the tree into a simple instruction list (the blueprint)
```

---

## Setup (one time only)

1. Install **Rust** from https://rustup.rs/ (free, just run the installer)
2. Open a terminal in this folder (`c:\rust\minicompiler_perfect`)
3. Build the project:
   ```bash
   cargo build
   ```

That's it. You now have a compiler tool.

---

## Writing Your First Source File

Create a file called `my_program.src` with this content:

```c
fn main() -> void {
    int x = 5;
    int y = 3;
    int result = x + y;
    return result;
}
```

**What does this mean?**
- `fn main() -> void` — declares a function called `main` that returns nothing (`void`)
- `int x = 5;` — creates an integer variable named `x` with value `5`
- `int result = x + y;` — adds `x` and `y`, stores in `result`
- `return result;` — exits the function returning `result`

---

## Step 1: Tokenizing (Lexer)

The lexer reads your source file and splits it into tokens — the "words" of the language.

```bash
cargo run -- lex --input my_program.src
```

You'll see output like:
```
1:1  KW_FN        "fn"
1:4  IDENTIFIER   "main"
1:8  LPAREN       "("
1:9  RPAREN       ")"
1:11 ARROW        "->"
1:14 KW_VOID      "void"
1:19 LBRACE       "{"
2:5  KW_INT       "int"
2:9  IDENTIFIER   "x"
2:11 ASSIGN       "="
2:13 INT_LITERAL  "5"   5
...
```

Each line means: `LINE:COLUMN  TOKEN_TYPE  "text"  [value]`

**In plain English:** The lexer is like a dictionary look-up — it identifies every "word"
in your code and labels what kind of word it is (keyword, number, operator, etc.).

---

## Step 2: Parsing (Building the Tree)

The parser takes those tokens and builds an **AST** (Abstract Syntax Tree) — a tree
structure showing how the code is organized.

```bash
cargo run -- parse --input my_program.src
```

Output:
```
Program:
  FunctionDecl: main -> void
    Parameters: []
    Body:
      Block:
        VarDecl: int x = 5
        VarDecl: int y = 3
        VarDecl: int result = (x + y)
        Return: result
```

**In plain English:** Think of it like diagramming a sentence in school. "The cat sat on
the mat" becomes Subject(cat) → Verb(sat) → PrepPhrase(on the mat). The parser does
the same for code.

You can also get a visual diagram:
```bash
cargo run -- parse --input my_program.src --ast-format dot --output ast.dot
dot -Tpng ast.dot -o ast.png
```
(Requires [Graphviz](https://graphviz.org/) to be installed)

---

## Step 3: Semantic Checking (Does the code make sense?)

The checker validates your code's logic. Even if it's grammatically correct, does it
make *sense*? For example — can you add a number to a word? No. Can you use a variable
before declaring it? No.

```bash
cargo run -- check --input my_program.src
```

If everything is fine:
```
OK
```

If there are errors, you'll get specific messages:

### Common Errors and What They Mean

**Undeclared variable:**
```
Semantic Error at line 5, column 18: Undefined variable 'z'
```
→ You used a variable `z` that was never declared with `int z = ...;`

**Type mismatch in assignment:**
```
Semantic Error at line 3, column 10: Type mismatch in assignment: cannot assign float to int 'x'
```
→ You declared `int x` (a whole number) but tried to put `3.14` (a decimal) in it.
  Whole numbers and decimals are different types in this language.

**Return type mismatch:**
```
Semantic Error at line 6, column 5: Return type mismatch: expected int, got bool
```
→ Your function says it returns `int` (a number) but you wrote `return true;` which is
  a `bool` (yes/no value). These don't match.

**Argument count mismatch:**
```
Semantic Error at line 10, column 5: Function 'add' requires 2 arguments, but 1 were provided
```
→ The function `add` expects 2 inputs, but you only gave it 1.

**Duplicate declaration:**
```
Semantic Error at line 4, column 5: Identifier 'x' already defined in this scope.
```
→ You declared `int x` twice in the same block.

### See the Symbol Table

Use `--verbose` to see every variable and function the checker found:

```bash
cargo run -- check --input my_program.src --verbose
```

Output:
```
Semantic analysis passed.
--- Symbol Table Dump ---
Scope level 0:
  main : Function of type fn() -> void
-------------------------
```

---

## Types in This Language

| Type | What it is | Example |
| :--- | :--- | :--- |
| `int` | Whole number | `int x = 42;` |
| `float` | Decimal number | `float pi = 3.14;` |
| `bool` | Yes/No (true/false) | `bool flag = true;` |
| `string` | Text | `string s = "hello";` |
| `void` | Nothing (used for functions that don't return a value) | `fn f() -> void { }` |

> **Important:** `int` and `float` are NOT automatically compatible.
> `int x = 3.14;` is a type mismatch. You must be explicit.

---

## Step 4: IR Generation (The Blueprint)

If your code passes all checks, the compiler translates it into **IR** (Intermediate
Representation) — a simplified list of instructions. Think of it like turning a
recipe into numbered steps a robot can follow.

```bash
cargo run -- ir --input my_program.src
```

Output:
```
--- IR Code (SSA Form) ---
entry:

func_main:
  x_1 = MOVE 5
  y_1 = MOVE 3
  t1_1 = ADD x_1, y_1
  result_1 = MOVE t1_1
  RETURN result_1
```

**What does SSA mean?** SSA stands for "Static Single Assignment". It means every
variable is assigned exactly once — instead of changing `x`, the compiler makes a new
version of it: `x_1`, `x_2`, `x_3`, etc. This makes the code much easier for the
compiler to optimize.

Save the IR to a file:
```bash
cargo run -- ir --input my_program.src --output my_program.ir
```

---

## Language Features

### Functions

```c
fn add(int a, int b) -> int {
    return a + b;
}

fn main() -> void {
    int result = add(3, 4);
}
```

### If / Else

```c
fn classify(int x) -> void {
    if (x > 0) {
        int positive = 1;
    } else {
        int not_positive = 1;
    }
}
```

### While Loop

```c
fn countdown(int n) -> void {
    while (n > 0) {
        n = n - 1;
    }
}
```

### For Loop

```c
fn sum(int limit) -> void {
    int total = 0;
    for (int i = 0; i < limit; i++) {
        total = total + i;
    }
}
```

### Structs

```c
struct Point {
    int x;
    int y;
}
```

### Comments

```c
// This is a single-line comment

/* This is a
   multi-line comment */
```

---

## Running Tests

The project has a comprehensive test suite. To run all tests:

```bash
cargo test
```

Current test count: **101 tests, all passing**.

| Suite | What it tests | Count |
| :--- | :--- | :--- |
| `lib.rs` inline tests | Lexer token recognition | 15 |
| `lexer_tests.rs` | Scanner methods and edge cases | 3 |
| `test_runner.rs` | Lexer golden file comparison | 2 |
| `parser_tests.rs` | AST construction and syntax errors | 25 |
| `semantic_tests.rs` | Type checking, scopes, errors | 26 |
| `semantic_golden.rs` | Semantic output vs. golden files | 2 |
| `ir_tests.rs` | IR instruction generation | 26 |
| `ir_golden.rs` | IR text output vs. golden files | 2 |

---

## The "Super Command": Dump

If you want to see everything at once (Tokens + AST + Symbol Table + IR), use the `dump` command:

```bash
cargo run -- dump --input my_program.src
```

This is the best way to see how the compiler transforms your code at every single step!


**What are golden files?** A "golden file" is a pre-saved expected output. The test
runs the compiler on a `.src` file, then compares the output to the matching `.txt`
file line-by-line. If they match, the test passes.

To regenerate golden files (needed after code changes):
```bash
UPDATE_EXPECT=1 cargo test --test semantic_golden
UPDATE_EXPECT=1 cargo test --test ir_golden
```

---

## Quick Reference Card

```bash
# Build the project
cargo build

# Tokenize a file
cargo run -- lex --input file.src

# Show the AST
cargo run -- parse --input file.src

# Check for semantic errors
cargo run -- check --input file.src

# Check with symbol table dump
cargo run -- check --input file.src --verbose

# Generate IR code
cargo run -- ir --input file.src

# Generate IR and save to file
cargo run -- ir --input file.src --output file.ir

# Export AST as PNG (requires Graphviz)
cargo run -- parse --input file.src --ast-format dot --output ast.dot
dot -Tpng ast.dot -o ast.png

# Run all tests
cargo test
```

---

## What Each File Does

```
minicompiler/
├── src/
│   ├── lexer/         Phase 1: Reads characters, produces tokens
│   ├── parser/        Phase 2: Reads tokens, builds AST
│   ├── semantic/      Phase 3: Checks the AST for logic errors
│   │   ├── analyzer.rs     — The main checker (walks the AST)
│   │   ├── symbol_table.rs — Remembers every variable/function declared
│   │   ├── types.rs        — Defines Int, Float, Bool, Void, etc.
│   │   └── errors.rs       — Error message formatting
│   ├── ir/            Phase 4: Translates AST into flat IR instructions
│   │   ├── ir_generator.rs   — Walks the AST, emits instructions
│   │   ├── ir_instructions.rs— Defines ADD, SUB, MOVE, JUMP, etc.
│   │   ├── basic_block.rs    — A group of instructions ending in a jump
│   │   └── ssa_constructor.rs— Renames variables for SSA form
│   └── main.rs        The CLI entry point (handles all commands)
├── tests/
│   ├── lexer/valid/   .src + .txt pairs for the lexer golden tests
│   ├── lexer/invalid/ .src + .txt pairs for error cases
│   ├── semantic/      .src + .txt pairs for semantic golden tests
│   ├── ir/            .src + .txt pairs for IR golden tests
│   └── *.rs           Rust test files for each phase
├── docs/
│   ├── language_spec.md  Formal grammar of the source language
│   ├── grammar.md        Parser grammar rules
│   ├── cli_reference.md  All commands and flags (this sibling file)
│   └── noob_guide.md     This file!
└── examples/          Sample .src programs
```
