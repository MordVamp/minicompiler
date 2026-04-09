# **Project: MiniCompiler - Technical Requirements Document (Sprint 3)**

**Sprint Goal:** Implement semantic analysis to validate declarations, types, and scopes, producing a decorated AST ready for code generation.

## **1. Project Structure & Repository Hygiene**

The codebase must be extended with semantic analysis components while maintaining existing structure and quality.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| STR-1 | All requirements from Sprint 2 (STR-1 to STR-3) **must** still be met. | Must |
| STR-2 | New source code **must** be integrated into the existing structure: | Must |
| | ```<br/>compiler-project/<br/>├── src/<br/>│ ├── lexer/          # Sprint 1<br/>│ ├── parser/         # Sprint 2<br/>│ ├── semantic/       # New directory for Sprint 3<br/>│ │ ├── analyzer.cpp/rs/py<br/>│ │ ├── symbol_table.cpp/rs/py<br/>│ │ ├── type_system.cpp/rs/py<br/>│ │ └── errors.cpp/rs/py<br/>│ ├── utils/<br/>│ └── main.cpp/rs/py<br/>└── ...<br/>``` | |
| STR-3 | The `README.md` **must** be updated to include: | Must |
| | - Documentation for semantic analysis phase<br/>- New command-line options for semantic checking<br/>- Updated error message examples<br/>- Instructions for running semantic tests | |

## **2. Symbol Table Data Structure**

A hierarchical symbol table must be implemented to track identifiers across nested scopes.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| SYM-1 | **Symbol Table Interface** must provide: | Must |
| | - `SymbolTable()` constructor creating global scope<br/>- `enter_scope()`: push new nested scope<br/>- `exit_scope()`: pop current scope<br/>- `insert(name, symbol_info)`: add symbol to current scope<br/>- `lookup(name)`: search from current to outer scopes<br/>- `lookup_local(name)`: search only current scope | |
| SYM-2 | **Scope Management** must support: | Must |
| | - Global scope (program level)<br/>- Function scope (parameters and local variables)<br/>- Block scope (e.g., inside if/while blocks)<br/>- Struct scope (for field names)<br/>- Nesting depth tracking and validation | |
| SYM-3 | **Symbol Information** must store for each identifier: | Must |
| | - Name (string)<br/>- Type (Type object)<br/>- Kind (variable, function, parameter, struct)<br/>- Declaration location (line, column)<br/>- For functions: return type, parameters<br/>- For structs: field definitions | |
| SYM-4 | **Type Representation** must define: | Must |
| | - Base types: `int`, `float`, `bool`, `void`, `string`<br/>- Struct types: name and field map<br/>- Function types: parameter types and return type<br/>- Array types (optional for stretch)<br/>- Type equivalence checking method | |
| SYM-5 | **Memory Layout** should track: | Should |
| | - Stack offset for local variables<br/>- Size in bytes for each type<br/>- Alignment requirements (for future code generation) | |

## **3. Semantic Analyzer Implementation**

A semantic analyzer must traverse the AST to validate all language rules.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| SEM-1 | **Analyzer Interface** must provide: | Must |
| | - `SemanticAnalyzer()` constructor<br/>- `analyze(ast: ProgramNode)`: perform all checks<br/>- `get_errors()`: return list of semantic errors<br/>- `get_symbol_table()`: return populated symbol table<br/>- `get_decorated_ast()`: return AST with type annotations | |
| SEM-2 | **Declaration Validation** must: | Must |
| | - Check for duplicate declarations in same scope<br/>- Validate function declarations (unique names, valid return types)<br/>- Validate struct declarations (no duplicate field names)<br/>- Process declarations before use (allow forward references for functions) | |
| SEM-3 | **Type Checking** must implement: | Must |
| | - Expression type inference (determine type of each expression)<br/>- Assignment compatibility (LHS type accepts RHS type)<br/>- Binary operator type rules (e.g., `int + int → int`, `int + float → float`)<br/>- Unary operator type rules (e.g., `!bool → bool`, `-int → int`)<br/>- Function return type checking | |
| SEM-4 | **Function Validation** must: | Must |
| | - Check function calls match declared signatures<br/>- Validate argument count matches parameter count<br/>- Check argument types are compatible with parameter types<br/>- Verify return statements match function return type<br/>- Handle `void` functions (no return value or empty return) | |
| SEM-5 | **Variable Usage** must: | Must |
| | - Detect undeclared variables before use<br/>- Prevent use of uninitialized variables (basic check)<br/>- Validate variable scope (no use outside declaring scope)<br/>- Check assignments to constants (if implemented) | |
| SEM-6 | **Control Flow** must validate: | Must |
| | - Condition expressions in if/while/for are boolean type<br/>- Loop control variables are mutable<br/>- Break/continue statements are within loops (if language has them) | |

## **4. Error Reporting & Recovery**

Meaningful error messages must guide the programmer to fix semantic issues.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| ERR-1 | **Error Categories** must include: | Must |
| | - Undeclared identifier<br/>- Duplicate declaration<br/>- Type mismatch<br/>- Argument count mismatch<br/>- Argument type mismatch<br/>- Invalid return type<br/>- Invalid condition type<br/>- Use before declaration<br/>- Invalid assignment target | |
| ERR-2 | **Error Messages** must contain: | Must |
| | - Error type and description<br/>- File name and location (line, column)<br/>- Context information (e.g., "in function 'foo'")<br/>- Expected vs. actual types where applicable<br/>- Suggestion for fix when possible | |
| ERR-3 | **Error Recovery** must: | Must |
| | - Continue analysis after errors to find multiple issues<br/>- Maintain symbolic information for subsequent analysis<br/>- Prevent cascading errors from single mistake<br/>- Collect all errors before returning | |
| ERR-4 | **Error Format** should be: | Should |
| | ```<br/>semantic error: undeclared variable 'x'<br/>  --> program.src:12:5<br/>  |<br/>  |     x = y + 5;<br/>  |     ^<br/>  |<br/>  note: did you mean 'y'? (declared at line 8)<br/>``` | |

## **5. Decorated AST & Output**

The semantic analyzer must produce an enriched AST with type information for code generation.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| DEC-1 | **AST Decoration** must add: | Must |
| | - Type annotation to every expression node<br/>- Symbol references linking identifiers to symbol table entries<br/>- Resolved function calls (linking to declaration)<br/>- Constant folding results for literal expressions | |
| DEC-2 | **Output Formats** must include: | Must |
| | - Type-annotated AST in text format<br/>- Symbol table dump<br/>- Type inference results for each expression<br/>- Error report summary | |
| DEC-3 | **Validation Report** must: | Should |
| | - Count of errors/warnings<br/>- List of all declared symbols by scope<br/>- Type hierarchy information<br/>- Memory layout summary (if tracked) | |

## **6. Testing & Verification**

Comprehensive tests must validate semantic rules and error detection.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| TEST-1 | **Unit Tests** must cover: | Must |
| | - Symbol table operations (insert, lookup, scope nesting)<br/>- Type compatibility checking<br/>- Function signature matching<br/>- Variable declaration and usage rules | |
| TEST-2 | **Test Programs** must include: | Must |
| | ```<br/>tests/semantic/valid/<br/>├── type_compatibility/<br/>├── function_overloading/  # if supported<br/>├── nested_scopes/<br/>└── complex_programs/<br/><br/>tests/semantic/invalid/<br/>├── undeclared_variable/<br/>├── type_mismatch/<br/>├── duplicate_declaration/<br/>├── argument_errors/<br/>└── scope_errors/<br/>``` | |
| TEST-3 | **Golden Testing** must compare: | Must |
| | - Expected vs. actual symbol table dumps<br/>- Expected vs. actual type annotations<br/>- Expected vs. actual error messages for invalid programs | |
| TEST-4 | **Integration Tests** must: | Should |
| | - Test full compilation pipeline (lex → parse → semantic)<br/>- Process all example programs with semantic checking<br/>- Verify decorated AST can be serialized and deserialized | |
| TEST-5 | **Error Message Tests** must verify: | Should |
| | - Error locations are accurate<br/>- Error messages are helpful and not misleading<br/>- Multiple errors are reported correctly<br/>- Error recovery doesn't produce false positives | |

## **7. Stretch Goal: Basic Type Inference for Constants**

Limited type inference may be implemented to enhance the type system.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| INF-1 | **Constant Inference** may infer types for: | Optional |
| | - Integer literals → `int` type<br/>- Float literals → `float` type<br/>- Boolean literals → `bool` type<br/>- String literals → `string` type<br/>- `null` literal → compatible with any pointer/reference type | |
| INF-2 | **Expression Inference** may propagate types: | Optional |
| | - Binary operations: infer result type from operand types<br/>- Function calls: infer type from return type<br/>- Array literals: infer element type from first element<br/>- Conditional expressions: infer common type of branches | |
| INF-3 | **Variable Inference** may support: | Optional |
| | - `var x = 42;` infers `int` type<br/>- `var y = 3.14;` infers `float` type<br/>- Limited to initializers with unambiguous types<br/>- No inference for function parameters or return types | |
| INF-4 | **Type Constraints** must ensure: | Optional |
| | - Inferred types are consistent across uses<br/>- No ambiguity in inference (error if ambiguous)<br/>- Inference doesn't violate existing type annotations<br/>- Error messages explain inference failures | |

**Example Semantic Rules:**
```
Type Compatibility:
- int is compatible with int
- float is compatible with float
- bool is compatible with bool
- int can be implicitly converted to float (widening)
- float cannot be implicitly converted to int (narrowing requires explicit cast)
- Assignment: LHS must be assignable and types must be compatible
- Function call: argument types must be compatible with parameter types

Binary Operators:
- Arithmetic (+, -, *, /, %): int ∨ int → int, float ∨ float → float, int ∨ float → float
- Comparison (==, !=, <, <=, >, >=): T ∨ T → bool (where T is numeric)
- Logical (&&, ||): bool ∨ bool → bool

Unary Operators:
- (-): int → int, float → float
- (!): bool → bool
```

**Example Usage:**
```bash
# Run semantic analysis on a program
$ ./compiler check --input examples/program.src --output symbols.txt

# Run with verbose output showing type inference
$ ./compiler check --input examples/program.src --verbose --show-types

# Dump symbol table
$ ./compiler symbols --input examples/program.src --format json

# Run semantic tests
$ ./compiler test --semantic
# or
$ make test-semantic
# or
$ cargo test semantic
# or
$ python -m pytest tests/semantic/
```

**Example Error Output:**
```
semantic error: type mismatch in assignment
  --> program.src:15:10
   |
14 |     float y;
15 |     y = "hello";  // Error: cannot assign string to float
   |          ^^^^^^
   |
   = expected: float
   = found: string

semantic error: undeclared variable 'unknown'
  --> program.src:22:5
   |
21 | fn foo() {
22 |     x = unknown + 5;
   |     ^ variable 'unknown' not found in this scope
   |
   = note: did you mean 'known'? (declared at line 18)

semantic error: argument count mismatch
  --> program.src:30:5
   |
29 |     bar(1, 2);  // Error: bar expects 3 arguments
30 |     ^^^^^^^^^
   |
   = expected: 3 arguments
   = found: 2 arguments
   = function signature: bar(int, int, int) -> void
```

**Example Decorated AST Output:**
```
Program [global scope]:
  Symbol Table:
    Global:
      - foo: function(int, float) -> bool (line 1)
      - x: int variable (line 10)
      - Point: struct (line 20)
        Fields: x: int, y: int

  FunctionDecl: foo -> bool (line 1):
    Parameters:
      - a: int
      - b: float
    Body [type checked]:
      Block:
        VarDecl: int result = (a > 5) [type: bool]  // Error: type mismatch
        Return: result [type: int, expected: bool]  // Error: return type mismatch

  Type Annotations:
    Line 5: a > 5 [type: bool]
    Line 5: result = (a > 5) [expected: int, actual: bool]
    Line 6: return result [expected: bool, actual: int]
```
