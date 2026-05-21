# **Project: MiniCompiler - Technical Requirements Document (Sprint 1)**

**Sprint Goal:** Establish the project foundation and implement lexical analysis (tokenization) for the simplified C-like source language.

## **1. Project Structure & Repository Hygiene**

The project must establish a clean, maintainable codebase with proper build automation and documentation.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| STR-1 | **Git Repository** must be initialized with the following structure: | Must |
| | ```<br/>compiler-project/<br/>├── src/<br/>│ ├── lexer/<br/>│ └── utils/<br/>├── tests/<br/>│ ├── lexer/<br/>│ │ ├── valid/<br/>│ │ └── invalid/<br/>│ └── test_runner<br/>├── examples/<br/>├── docs/<br/>└── README.md<br/>``` | |
| STR-2 | **Build System** must provide a single command to build the project: | Must |
| | - **For C++:** CMake (≥3.10) or Makefile with C++17 support<br/>- **For Rust:** Cargo.toml with 2021 edition<br/>- **For Python:** setup.py or pyproject.toml with entry points | |
| STR-3 | **README.md** must include: | Must |
| | - Project description and team information<br/>- Build instructions for all supported platforms<br/>- Quick start example<br/>- Link to language specification | |
| STR-4 | **Code Quality** must be maintained through: | Should |
| | - Consistent naming conventions (camelCase/snake_case as per language)<br/>- No compiler warnings (for compiled languages)<br/>- Modular organization separating scanner logic from token definitions | |

## **2. Language Specification Document**

A formal specification of the source language's lexical elements must be documented.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| LANG-1 | **Lexical Grammar** must be formally specified in `docs/language_spec.md` using EBNF notation: | Must |
| | - Token categories: Keywords, Identifiers, Literals, Operators, Delimiters<br/>- Regular expressions for each token type<br/>- Character set and encoding (UTF-8) | |
| LANG-2 | **Keywords** must be explicitly listed: | Must |
| | `if`, `else`, `while`, `for`, `int`, `float`, `bool`, `return`, `true`, `false`, `void`, `struct`, `fn` | |
| LANG-3 | **Identifiers** must follow: | Must |
| | - Start with letter [a-zA-Z]<br/>- Followed by letters, digits [0-9], or underscores<br/>- Maximum length: 255 characters<br/>- Case-sensitive | |
| LANG-4 | **Literal Types** must be defined: | Must |
| | - **Integer:** Decimal digits, range [-2³¹, 2³¹-1]<br/>- **Float:** Decimal with '.' separator<br/>- **String:** Double-quoted, escape sequences optional for Sprint 1<br/>- **Boolean:** `true` or `false` | |
| LANG-5 | **Operators & Delimiters** must be enumerated: | Must |
| | - Arithmetic: `+ - * / %`<br/>- Relational: `== != < <= > >=`<br/>- Logical: `&& || !`<br/>- Assignment: `= += -= *= /=`<br/>- Delimiters: `( ) { } [ ] ; , :` | |
| LANG-6 | **Whitespace & Comments** must be specified: | Must |
| | - Whitespace: space, tab, newline (\n), carriage return (\r)<br/>- Single-line comments: `//` to end-of-line<br/>- Multi-line comments: `/* ... */` (optional nesting) | |

## **3. Lexical Analyzer (Scanner) Implementation**

A scanner must be implemented that converts source code into a stream of tokens with position information.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| LEX-1 | **Token Data Structure** must include: | Must |
| | - Token type (enumeration)<br/>- Lexeme (string)<br/>- Line number (1-indexed)<br/>- Column number (1-indexed)<br/>- Literal value (union/discriminated enum for typed values) | |
| LEX-2 | **Scanner Interface** must provide: | Must |
| | - `Scanner(source: string)` constructor<br/>- `next_token(): Token` method to advance and return token<br/>- `peek_token(): Token` method to lookahead without advancing<br/>- `is_at_end(): bool` method<br/>- `get_line(): int` and `get_column(): int` methods | |
| LEX-3 | **Token Recognition** must correctly identify: | Must |
| | - All keywords (reserved words)<br/>- Identifiers (distinguishing from keywords)<br/>- All literal types with value extraction<br/>- All operators (including multi-character like `==`, `+=`)<br/>- All delimiters<br/>- End-of-file marker | |
| LEX-4 | **Position Tracking** must: | Must |
| | - Track line and column numbers accurately<br/>- Handle Unix (\n) and Windows (\r\n) line endings<br/>- Report positions in error messages | |
| LEX-5 | **Error Handling** must: | Must |
| | - Report invalid characters with line/column<br/>- Report unterminated strings/comments<br/>- Skip invalid tokens and continue scanning (error recovery)<br/>- Provide clear, human-readable error messages | |
| LEX-6 | **Performance** must: | Should |
| | - Process input in O(n) time<br/>- Use memory efficiently (no excessive copying)<br/>- Handle files up to 1MB in size | |

## **4. Testing & Verification**

A comprehensive test suite must validate the lexer's correctness and robustness.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| TEST-1 | **Unit Tests** must cover: | Must |
| | - Each token type individually<br/>- Edge cases (minimum/maximum integers, longest identifiers)<br/>- Token boundaries and whitespace handling<br/>- Error cases | |
| TEST-2 | **Test Files** must be organized in: | Must |
| | ```<br/>tests/lexer/valid/<br/>├── test_identifiers.src<br/>├── test_numbers.src<br/>├── test_keywords.src<br/>├── test_operators.src<br/>└── test_comments.src<br/><br/>tests/lexer/invalid/<br/>├── test_invalid_char.src<br/>├── test_unterminated_string.src<br/>└── test_malformed_number.src<br/>``` | |
| TEST-3 | **Expected Output Format** must be: | Must |
| | ```<br/>LINE:COLUMN TOKEN_TYPE "LEXEME" [LITERAL_VALUE]<br/># Example:<br/>1:1 KW_IF "if"<br/>1:4 LPAREN "("<br/>1:5 IDENTIFIER "x"<br/>1:6 RPAREN ")"<br/>2:1 END_OF_FILE ""<br/>``` | |
| TEST-4 | **Automated Test Runner** must: | Must |
| | - Run all tests with a single command<br/>- Compare actual output to expected token files<br/>- Report success/failure for each test case<br/>- Provide verbose output on failure showing differences | |
| TEST-5 | **Test Coverage** must include at least: | Should |
| | - 20 valid test cases<br/>- 10 invalid/error test cases<br/>- Mixed token sequences<br/>- Files with various line ending styles | |

## **5. Stretch Goal: Preprocessor**

A preprocessor may be implemented to handle comments and simple macros, enhancing the language support.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| PRE-1 | **Comment Removal** must: | Optional |
| | - Remove single-line comments (`//`) and everything to newline<br/>- Remove multi-line comments (`/* ... */`)<br/>- Preserve line numbering for error reporting (replace with newline or single space) | |
| PRE-2 | **Simple Macros** may support: | Optional |
| | - `#define NAME value` for constant substitution<br/>- Simple token replacement (no arguments)<br/>- Basic include guards: `#ifdef`, `#ifndef`, `#endif` | |
| PRE-3 | **Preprocessor Interface** may provide: | Optional |
| | - `Preprocessor(source: string)` constructor<br/>- `process(): string` method returning cleaned source<br/>- `define(name, value)` and `undefine(name)` methods | |
| PRE-4 | **Edge Cases** must be handled: | Optional |
| | - Comments inside string literals must be preserved<br/>- Unterminated comments must report errors<br/>- Macro recursion must be detected and prevented | |

**Example Test Commands:**
```bash
# Build the project
$ make build
# or
$ cmake --build build
# or
$ cargo build

# Run the lexer on a sample file
$ ./compiler lex --input examples/hello.src --output tokens.txt

# Run all tests
$ make test
# or
$ ctest
# or
$ cargo test
# or
$ python -m pytest tests/
```

**Expected Token Output Example:**
```
1:1 KW_FN "fn"
1:4 IDENTIFIER "main"
1:8 LPAREN "("
1:9 RPAREN ")"
1:10 LBRACE "{"
2:5 KW_INT "int"
2:9 IDENTIFIER "counter"
2:16 ASSIGN "="
2:18 INT_LITERAL "42" 42
2:20 SEMICOLON ";"
3:1 RBRACE "}"
4:1 END_OF_FILE ""
```
