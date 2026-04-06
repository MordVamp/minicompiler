# MiniCompiler Formal Grammar Specification (Sprint 2)

This document describes the Context-Free Grammar (CFG) for the MiniCompiler language in Extended Backus-Naur Form (EBNF).

## 1. Start Symbol & Program
```ebnf
Program ::= Declaration* EOF
```

## 2. Declarations
```ebnf
Declaration    ::= FunctionDecl | StructDecl | VarDeclStmt
FunctionDecl   ::= "fn" Identifier "(" [ParamList] ")" ["->" Type] BlockStmt
StructDecl     ::= "struct" Identifier "{" VarDeclStmt* "}"
ParamList      ::= Param ("," Param)*
Param          ::= Type Identifier
Type           ::= "int" | "float" | "bool" | "void" | Identifier
```

## 3. Statements
```ebnf
Statement      ::= ExprStmt 
                 | BlockStmt 
                 | IfStmt 
                 | WhileStmt 
                 | ForStmt 
                 | ReturnStmt 
                 | VarDeclStmt 
                 | EmptyStmt

BlockStmt      ::= "{" Statement* "}"
VarDeclStmt    ::= Type Identifier ["=" Expression] ";"
ExprStmt       ::= Expression ";"
IfStmt         ::= "if" "(" Expression ")" Statement ["else" Statement]
WhileStmt      ::= "while" "(" Expression ")" Statement
ForStmt        ::= "for" "(" [ForInit] ";" [Expression] ";" [Expression] ")" Statement
ForInit        ::= VarDeclStmt | ExprStmt
ReturnStmt     ::= "return" [Expression] ";"
EmptyStmt      ::= ";"
```

## 4. Expressions
Expressions are parsed based on operator precedence, from lowest (Assignment) to highest (Primary).

```ebnf
Expression     ::= Assignment
Assignment     ::= Identifier ( "=" | "+=" | "-=" | "*=" | "/=" ) Assignment 
                 | LogicalOr

LogicalOr      ::= LogicalAnd ( "||" LogicalAnd )*
LogicalAnd     ::= Equality ( "&&" Equality )*
Equality       ::= Relational ( ( "==" | "!=" ) Relational )*
Relational     ::= Additive ( ( "<" | "<=" | ">" | ">=" ) Additive )*
Additive       ::= Multiplicative ( ( "+" | "-" ) Multiplicative )*
Multiplicative ::= Unary ( ( "*" | "/" | "%" ) Unary )*
Unary          ::= ( "!" | "-" ) Unary | Primary

Primary        ::= IntLiteral 
                 | FloatLiteral 
                 | StringLiteral 
                 | BoolLiteral 
                 | Identifier ["(" [ArgList] ")"] 
                 | "(" Expression ")"

ArgList        ::= Expression ("," Expression)*
```

## 5. Token Terminals (Lexical Elements)
- `Identifier`
- `IntLiteral`
- `FloatLiteral`
- `StringLiteral`
- `BoolLiteral` (`true`, `false`)

## 6. Precedence and Associativity Table

| Precedence Level | Operators | Associativity | Example |
| :--- | :--- | :--- | :--- |
| 1 (Highest) | `!`, `-` (Unary) | Right | `!a`, `-b` |
| 2 | `*`, `/`, `%` | Left | `a * b / c` |
| 3 | `+`, `-` | Left | `a + b - c` |
| 4 | `<`, `<=`, `>`, `>=` | Non-Associative | `a < b` |
| 5 | `==`, `!=` | Non-Associative | `a == b` |
| 6 | `&&` | Left | `a && b && c` |
| 7 | `\|\|` | Left | `a \|\| b \|\| c` |
| 8 (Lowest) | `=`, `+=`, `-=`, `*=`, `/=` | Right | `a = b = c` |
