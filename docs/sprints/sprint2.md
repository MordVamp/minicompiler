# **Project: MiniCompiler - Technical Requirements Document (Sprint 2)**

**Sprint Goal:** Define the language grammar and implement a parser that builds an Abstract Syntax Tree (AST) from tokens.

## **1. Project Structure & Repository Hygiene**

The codebase must be extended with parser components while maintaining existing structure and quality.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| STR-1 | All requirements from Sprint 1 (STR-1 to STR-4) **must** still be met. | Must |
| STR-2 | New source code **must** be integrated into the existing structure: | Must |
| | ```<br/>compiler-project/<br/>├── src/<br/>│ ├── lexer/          # From Sprint 1<br/>│ ├── parser/         # New directory for Sprint 2<br/>│ │ ├── parser.cpp/rs/py<br/>│ │ ├── ast.cpp/rs/py<br/>│ │ └── grammar.txt<br/>│ ├── utils/<br/>│ └── main.cpp/rs/py<br/>└── ...<br/>``` | |
| STR-3 | The `README.md` **must** be updated to include: | Must |
| | - Documentation for the new parser command-line options<br/>- Updated usage examples showing AST output<br/>- Formal grammar specification<br/>- Instructions for running parser tests | |

## **2. Formal Grammar Specification**

A complete, unambiguous Context-Free Grammar (CFG) must be documented in formal notation.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| GRAM-1 | **Grammar Document** must be provided in `docs/grammar.md` or `src/parser/grammar.txt` using EBNF or BNF notation. | Must |
| GRAM-2 | **Grammar Components** must define: | Must |
| | - Start symbol: `Program`<br/>- Non-terminals for all language constructs<br/>- Terminal symbols matching token types from Sprint 1<br/>- Production rules with clear precedence and associativity | |
| GRAM-3 | **Expression Grammar** must handle operator precedence (highest to lowest): | Must |
| | 1. Primary expressions (literals, identifiers, parenthesized)<br/>2. Unary operators (`-`, `!`)<br/>3. Multiplicative (`*`, `/`, `%`)<br/>4. Additive (`+`, `-`)<br/>5. Relational (`<`, `<=`, `>`, `>=`)<br/>6. Equality (`==`, `!=`)<br/>7. Logical AND (`&&`)<br/>8. Logical OR (`||`)<br/>9. Assignment (`=`, `+=`, `-=`, etc.) | |
| GRAM-4 | **Statement Grammar** must include: | Must |
| | - Block statements `{ ... }`<br/>- Variable declarations `int x = 5;`<br/>- Assignment statements `x = 10;`<br/>- Conditional statements `if (cond) { ... } else { ... }`<br/>- Loop statements `while (cond) { ... }`, `for (init; cond; update) { ... }`<br/>- Return statements `return expr;`<br/>- Function calls `foo(1, 2);`<br/>- Empty statements `;` | |
| GRAM-5 | **Declaration Grammar** must include: | Must |
| | - Function declarations `fn name(params) -> return_type { ... }`<br/>- Variable declarations `type name [= initializer];`<br/>- Struct declarations `struct Name { fields }` | |
| GRAM-6 | **Precedence & Associativity** must be explicitly documented: | Must |
| | - Left-associative: `+ - * / % && \|\|`<br/>- Right-associative: `= += -=` (assignment operators)<br/>- Non-associative: `== != < <= > >=` | |

## **3. Parser Implementation**

A recursive descent parser must be implemented that consumes tokens and constructs an AST.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| PAR-1 | **Parser Interface** must provide: | Must |
| | - `Parser(tokens: List[Token])` constructor<br/>- `parse(): ASTNode` method returning the root of the AST<br/>- Error reporting with line/column information<br/>- Integration with the lexer from Sprint 1 | |
| PAR-2 | **Recursive Descent Methods** must implement grammar rules as methods: | Must |
| | - `parseProgram(): ProgramNode`<br/>- `parseStatement(): StatementNode`<br/>- `parseExpression(): ExpressionNode`<br/>- `parseDeclaration(): DeclarationNode`<br/>- Helper methods: `match(token_type)`, `consume(token_type, error_msg)`, `peek()`, `isAtEnd()` | |
| PAR-3 | **Error Detection** must: | Must |
| | - Report syntax errors with location and expected tokens<br/>- Handle missing semicolons, mismatched parentheses/braces<br/>- Continue parsing after errors (basic recovery) | |
| PAR-4 | **Lookahead & Prediction** must: | Must |
| | - Use at least 1-token lookahead for LL(1) grammar portions<br/>- Implement prediction logic for statement/expression disambiguation<br/>- Handle the "dangling else" problem correctly | |
| PAR-5 | **Performance** must: | Should |
| | - Parse in O(n) time for LL(1) grammar<br/>- Use memory proportional to AST size<br/>- Handle recursive structures without stack overflow (within reasonable limits) | |

## **4. Abstract Syntax Tree (AST) Data Structure**

A well-defined AST must represent the syntactic structure without unnecessary lexical details.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| AST-1 | **Node Hierarchy** must define base classes/interfaces: | Must |
| | - `ASTNode` (abstract base with line/column info)<br/>- `ExpressionNode` (extends ASTNode)<br/>- `StatementNode` (extends ASTNode)<br/>- `DeclarationNode` (extends ASTNode)<br/>- `ProgramNode` (container for declarations) | |
| AST-2 | **Expression Nodes** must include: | Must |
| | - `LiteralExprNode` (value: int, float, bool, string)<br/>- `IdentifierExprNode` (name: string)<br/>- `BinaryExprNode` (left, operator, right)<br/>- `UnaryExprNode` (operator, operand)<br/>- `CallExprNode` (callee, arguments: List[ExpressionNode])<br/>- `AssignmentExprNode` (target, operator, value) | |
| AST-3 | **Statement Nodes** must include: | Must |
| | - `BlockStmtNode` (statements: List[StatementNode])<br/>- `ExprStmtNode` (expression: ExpressionNode)<br/>- `IfStmtNode` (condition, thenBranch, elseBranch)<br/>- `WhileStmtNode` (condition, body)<br/>- `ForStmtNode` (init, condition, update, body)<br/>- `ReturnStmtNode` (value: ExpressionNode or null)<br/>- `VarDeclStmtNode` (type, name, initializer) | |
| AST-4 | **Declaration Nodes** must include: | Must |
| | - `FunctionDeclNode` (returnType, name, parameters, body)<br/>- `StructDeclNode` (name, fields: List<VarDeclStmtNode>)<br/>- `ParamNode` (type, name) | |
| AST-5 | **Visitor Pattern** should be implemented for: | Should |
| | - AST traversal and analysis<br/>- Pretty printing<br/>- Code generation (future sprint)<br/>- Type checking (future sprint) | |

## **5. AST Visualization & Output**

The parser must provide mechanisms to inspect and visualize the AST.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| VIS-1 | **Pretty Printer** must output AST in human-readable text format: | Must |
| | - Indentation to show nesting<br/>- Parentheses to clarify precedence<br/>- Type annotations on declarations<br/>- Example output:<br/>```<br/>Program:<br/>  FunctionDecl: main -> void<br/>    Parameters: []<br/>    Body:<br/>      Block:<br/>        VarDecl: int x = 42<br/>        Return: x<br/>``` | |
| VIS-2 | **Graphviz DOT Output** must generate `.dot` files for visualization: | Must |
| | - Each node as a box with label<br/>- Edges showing parent-child relationships<br/>- Color coding by node type<br/>- Example: `./compiler parse --input program.src --output ast.dot` | |
| VIS-3 | **JSON Output** should optionally output AST in JSON format: | Should |
| | - Machine-readable for automated testing<br/>- Preserves all node properties<br/>- Can be consumed by other tools | |
| VIS-4 | **Command-line Interface** must support: | Must |
| | - `--ast-format [text\|dot\|json]` option<br/>- `--output-file` or standard output<br/>- `--verbose` for additional parsing information | |

## **6. Testing & Verification**

A comprehensive test suite must validate the parser's correctness and error handling.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| TEST-1 | **Unit Tests** must cover: | Must |
| | - Each production rule in the grammar<br/>- Operator precedence and associativity<br/>- Nested structures (if-in-if, loops-in-functions)<br/>- Edge cases (empty blocks, zero-argument functions) | |
| TEST-2 | **Test Categories** must include: | Must |
| | ```<br/>tests/parser/valid/<br/>├── expressions/<br/>├── statements/<br/>├── declarations/<br/>└── full_programs/<br/><br/>tests/parser/invalid/<br/>├── syntax_errors/<br/>├── type_errors/      # Basic checking if implemented<br/>└── semantic_errors/<br/>``` | |
| TEST-3 | **Golden Testing** must compare AST output: | Must |
| | - Expected ASTs stored in `.expected` files<br/>- Text or JSON format for comparison<br/>- Automated comparison with diff output on failure | |
| TEST-4 | **Error Detection Tests** must verify: | Must |
| | - Syntax error detection (missing tokens, misplaced tokens)<br/>- Error messages include line and column<br/>- Parser recovers to continue checking rest of file | |
| TEST-5 | **Integration Tests** must: | Should |
| | - Test lexer-parser integration<br/>- Process complete example programs from `examples/` directory<br/>- Round-trip: parse → pretty-print → re-parse should produce equivalent AST | |

## **7. Stretch Goal: Robust Error Recovery**

Advanced error recovery techniques may be implemented to improve parser usability.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| ERR-1 | **Error Recovery Strategies** may implement: | Optional |
| | - Panic mode recovery: skip to next synchronization point (semicolon, brace)<br/>- Phrase-level recovery: insert missing tokens or delete erroneous ones<br/>- Error productions: add error-handling productions to grammar | |
| ERR-2 | **Synchronization Points** should be defined at: | Optional |
| | - Statement boundaries (semicolons, closing braces)<br/>- Declaration boundaries<br/>- Expression boundaries in some cases | |
| ERR-3 | **Error Metrics** may track: | Optional |
| | - Number of errors detected vs. actual errors<br/>- Cascade prevention (avoiding spurious additional errors)<br/>- Quality of recovery (can meaningful analysis continue?) | |
| ERR-4 | **User Experience** should provide: | Optional |
| | - Multiple errors reported in single run (not just first error)<br/>- Suggestions for fixes (e.g., "Did you forget a semicolon?")<br/>- Ability to limit error count before aborting | |

**Example Grammar (EBNF Notation):**
```
Program        ::= { Declaration }
Declaration    ::= FunctionDecl | StructDecl | VarDecl
FunctionDecl   ::= "fn" Identifier "(" [ Parameters ] ")" ["->" Type] Block
StructDecl     ::= "struct" Identifier "{" { VarDecl } "}"
VarDecl        ::= Type Identifier [ "=" Expression ] ";"

Statement      ::= Block | IfStmt | WhileStmt | ForStmt | ReturnStmt
                | ExprStmt | VarDecl
Block          ::= "{" { Statement } "}"
IfStmt         ::= "if" "(" Expression ")" Statement [ "else" Statement ]
WhileStmt      ::= "while" "(" Expression ")" Statement
ForStmt        ::= "for" "(" [ ExprStmt ] ";" [ Expression ] ";" [ Expression ] ")" Statement
ReturnStmt     ::= "return" [ Expression ] ";"
ExprStmt       ::= Expression ";"

Expression     ::= Assignment
Assignment     ::= LogicalOr { ("=" | "+=" | "-=" | "*=" | "/=") Assignment }
LogicalOr      ::= LogicalAnd { "||" LogicalAnd }
LogicalAnd     ::= Equality { "&&" Equality }
Equality       ::= Relational { ("==" | "!=") Relational }
Relational     ::= Additive { ("<" | "<=" | ">" | ">=") Additive }
Additive       ::= Multiplicative { ("+" | "-") Multiplicative }
Multiplicative ::= Unary { ("*" | "/" | "%") Unary }
Unary          ::= [ "-" | "!" ] Primary
Primary        ::= Literal | Identifier | "(" Expression ")" | Call
Call           ::= Identifier "(" [ Arguments ] ")"
Arguments      ::= Expression { "," Expression }

Type           ::= "int" | "float" | "bool" | "void" | Identifier
Parameters     ::= Parameter { "," Parameter }
Parameter      ::= Type Identifier
Literal        ::= Integer | Float | String | Boolean | "null"
```

**Example Parser Usage:**
```bash
# Parse and output AST in text format
$ ./compiler parse --input examples/factorial.src --output ast.txt

# Generate Graphviz DOT file
$ ./compiler parse --input examples/factorial.src --format dot --output ast.dot

# Convert DOT to PNG visualization
$ dot -Tpng ast.dot -o ast.png

# Run parser tests
$ ./compiler test --parser
# or
$ make test-parser
# or
$ cargo test parser
# or
$ python -m pytest tests/parser/
```

**Example AST Output (Text Format):**
```
Program [line 1]:
  FunctionDecl: main -> void [line 1]:
    Parameters: []
    Body [line 1]:
      Block [line 2-5]:
        VarDecl: int n = 5 [line 3]
        VarDecl: int result = 1 [line 4]
        WhileStmt [line 5-9]:
          Condition [line 5]:
            Binary: n > 0
          Body [line 5]:
            Block [line 6-8]:
              Assignment: result = result * n [line 7]
              Assignment: n = n - 1 [line 8]
        Return: result [line 10]
```
