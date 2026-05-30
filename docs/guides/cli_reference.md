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
cargo run -- lex --input examples/basic/hello.src
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
cargo run -- parse --input examples/basic/hello.src

# Generate Graphviz DOT file then render to PNG
cargo run -- parse --input examples/basic/hello.src --ast-format dot --output ast.dot
dot -Tpng ast.dot -o ast.png

# Save AST as JSON
cargo run -- parse --input examples/basic/hello.src --ast-format json --output ast.json

# Verbose mode
cargo run -- parse --input examples/basic/hello.src --verbose
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
cargo run -- check --input examples/basic/hello.src
# Output: OK

# Check with symbol table dump
cargo run -- check --input examples/basic/hello.src --verbose
# Output:
#   Semantic analysis passed.
#   --- Symbol Table Dump ---
#   Scope level 0:
#     main : Function of type fn() -> void
#   -------------------------

# Check a file with errors (prints to stderr)
cargo run -- check --input examples/basic/error.src
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
| `--format <FORMAT>` | `-f` | No | `text` | Output format: `text` or `dot` (CFG visualization) |
| `--stats` | `-s` | No | false | Report IR statistics (instruction and block counts) |
| `--verbose` | `-v` | No | false | Print a success message on completion |

**Note:** If semantic analysis fails, IR generation is aborted and errors are printed.

**Examples:**
```bash
# Print IR as Text to console
cargo run -- ir --input examples/basic/hello.src

# Generate CFG in DOT format and render to PNG
cargo run -- ir --input examples/basic/hello.src --format dot --output cfg.dot
dot -Tpng cfg.dot -o cfg.png

# Report IR statistics (counts of instructions and blocks)
cargo run -- ir --input examples/basic/hello.src --stats

# Save IR to file
cargo run -- ir --input examples/basic/hello.src --output output.ir
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

### `compile` — Generate x86-64 Assembly (Sprint 5-6)

Runs the full pipeline and generates x86-64 assembly code in NASM syntax.

```
cargo run -- compile --input <FILE> [--output <FILE>] [--verbose]
```

| Flag | Short | Required | Default | Description |
| :--- | :--- | :--- | :--- | :--- |
| `--input <FILE>` | `-i` | Yes | — | Path to the `.src` source file |
| `--output <FILE>` | `-o` | No | stdout | Write assembly output to this file |
| `--verbose` | `-v` | No | false | Print success message |
| `--optimize` | `-O` | No | false | Show assembly before and after optimization |

| `--stdout` | `-s` | No | false | Print assembly to console (overrides output file) |


**Example:**
```bash
# Generate assembly and save to file
cargo run -- compile --input examples/basic/hello.src --output hello.asm

# Assemble and link with runtime
nasm -f elf64 hello.asm -o hello.o
nasm -f elf64 src/runtime/runtime.asm -o runtime.o
ld hello.o runtime.o -o hello
./hello
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
Sequentially prints the output of `lex`, `parse`, `check --verbose`, and `ir`. The symbol table dump in this mode includes **Archived Scopes**, showing variables that have gone out of scope but were tracked during analysis.

**Example:**

---

### `run` — Запуск скрипта компиляции и исполнения (Спринт 7+)

Автоматизирует процесс сборки: транслирует `.src` в `.asm` через `minicompiler`, собирает через `nasm`, линкует через `gcc` и сразу запускает бинарник. 

```bash
cargo run -- run <FILE> [BIN_NAME] [--keep] [--time] [--perf]
```

| Аргумент / Флаг | Описание |
| :--- | :--- |
| `<FILE>` | Путь к исходному `.src` файлу (Обязательный) |
| `[BIN_NAME]` | Имя итогового бинарного файла (Опционально) |
| `--keep` | Сохранить сгенерированные `.asm` и `.o` файлы после линковки |
| `--time` | Замерить время выполнения сгенерированного бинарника (wall-clock time) |
| `--perf` | Собрать аппаратные метрики производительности (`perf stat`) (только для Linux) |

**Примеры (через внутренний CLI):**
```bash
cargo run -- run examples/quick_sort.src
cargo run -- run examples/sprint7/test_extern.src
cargo run -- run examples/sprint6/sprint6_control.src mybin
cargo run -- run examples/bubble_sort.src - --keep
```

**Прямой вызов скрипта (bash):**
```bash
./run.sh examples/sprint7/test_extern.src        # compile & run
./run.sh examples/sprint6/sprint6_control.src mybin  # custom binary name
./run.sh examples/sprint7/sprint7_arrays.src - --keep  # keep .asm + .o files
./run.sh examples/quick_sort.src
./run.sh examples/bubble_sort.src - --keep
```

---

### `compile` — Генерация кода x86-64 (Sprint 5-7)

Транслирует исходный код в ассемблер x86-64 (формат NASM). Поддерживает массивы и внешние функции.

```bash
cargo run -- compile --input <FILE> [--output <FILE>] [--optimize] [--stdout]
```

| Флаг | Короткий | Описание |
| :--- | :--- | :--- |
| `--input <FILE>` | `-i` | Исходный файл `.src` |
| `--output <FILE>` | `-o` | Куда записать `.asm` |
| `--optimize` | `-O` | Включить глазковую оптимизацию |
| `--stdout` | `-s` | Вывести результат в консоль |

---

## Примеры Спринта 7 (Массивы и Extern)

Компиляция кода с использованием массивов и внешних функций C:

```bash
# 1. Генерация ассемблера
cargo run -- compile --input examples/sprint7/test_extern.src --output test.asm

# 2. Сборка объектного файла (требуется nasm)
nasm -f elf64 test.asm -o test.o

# 3. Линковка с библиотекой C (требуется gcc или ld)
gcc test.o -o test -no-pie

# 4. Запуск
./test
```

**Встроенные функции:** `printf`, `scanf`, `malloc`, `free`, `print_int`, `read_int`.

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
| `cargo test --test codegen_tests` | Run only Sprint 5-6 codegen tests |
| `cargo test --test test_runner` | Run only Sprint 1 lexer golden tests |
| `UPDATE_EXPECT=1 cargo test --test semantic_golden` | Regenerate Sprint 3 golden `.txt` files |
| `UPDATE_EXPECT=1 cargo test --test ir_golden` | Regenerate Sprint 4 golden `.txt` files |
