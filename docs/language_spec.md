
# Language Specification

This document defines the lexical grammar of the source language for the MiniCompiler project.

## Character Set

The source code is encoded in UTF-8. Only ASCII characters are significant for tokens; non-ASCII characters are allowed only inside comments and string literals (but are not required to be handled specially in Sprint 1).

## Lexical Grammar

The following EBNF rules describe the lexical structure of the language.

```
(* Whitespace *)
whitespace = { ' ' | '\t' | '\r' | '\n' } ;

(* Comments *)
single_line_comment = "//" , { character - '\n' } ;
block_comment = "/*" , { block_comment | character - '*/' } , "*/" ;

(* Identifiers *)
identifier = letter , { letter | digit | '_' } ;
letter = 'a'..'z' | 'A'..'Z' ;
digit = '0'..'9' ;

(* Keywords *)
keyword = "if" | "else" | "while" | "for" | "int" | "float" | "bool"
        | "return" | "true" | "false" | "void" | "struct" | "fn" ;

(* Literals *)
integer_literal = digit , { digit } ;
float_literal = ( digit , { digit } , '.' , { digit } ) | ( '.' , digit , { digit } ) ;
string_literal = '"' , { character - '"' - '\n' } , '"' ;
boolean_literal = "true" | "false" ;  (* treated as keywords *)

(* Operators *)
operator = arithmetic_operator | relational_operator | logical_operator | assignment_operator ;
arithmetic_operator = '+' | '-' | '*' | '/' | '%' ;
relational_operator = "==" | "!=" | '<' | "<=" | '>' | ">=" ;
logical_operator = "&&" | "||" | '!' ;
assignment_operator = '=' | "+=" | "-=" | "*=" | "/=" ;

(* Delimiters *)
delimiter = '(' | ')' | '{' | '}' | '[' | ']' | ';' | ',' | ':' ;

(* Token *)
token = keyword | identifier | integer_literal | float_literal
      | string_literal | boolean_literal | operator | delimiter ;
```

## Token Categories

### Keywords
The following identifiers are reserved as keywords and cannot be used as ordinary identifiers:

- `if`
- `else`
- `while`
- `for`
- `int`
- `float`
- `bool`
- `return`
- `true`
- `false`
- `void`
- `struct`
- `fn`

### Identifiers
- Must begin with a letter (a–z, A–Z).
- Subsequent characters may be letters, digits (0–9), or underscores (`_`).
- Case-sensitive.
- Maximum length: 255 characters.

### Literals

#### Integer Literals
- Sequence of decimal digits.
- Range: -2³¹ to 2³¹-1 (inclusive). The lexer does not accept the unary minus as part of the literal; negative numbers are represented by the `-` operator followed by a positive integer literal.
- Example: `0`, `42`, `2147483647`

#### Float Literals
- Must contain a decimal point (`.`).
- At least one digit must appear either before or after the decimal point, but not both absent.
- Examples: `0.0`, `3.14`, `.5` (malformed, language does not allow leading dot), `10.` (malformed, trailing dot). The lexer will treat malformed floats as errors.
- Parsed as a 64-bit floating-point number.

#### String Literals
- Enclosed in double quotes (`"`).
- May contain any character except a double quote or newline. (Escape sequences are not supported in Sprint 1.)
- Examples: `"hello"`, `""` (empty string).

#### Boolean Literals
- `true` and `false` are keywords and produce boolean literal values.

### Operators & Delimiters

#### Arithmetic Operators
- `+` (Plus)
- `-` (Minus)
- `*` (Star)
- `/` (Slash)
- `%` (Percent)

#### Relational Operators
- `==` (EqualEqual)
- `!=` (NotEqual)
- `<`  (Less)
- `<=` (LessEqual)
- `>`  (Greater)
- `>=` (GreaterEqual)

#### Logical Operators
- `&&` (AndAnd)
- `||` (OrOr)
- `!`  (Bang)

#### Assignment Operators
- `=`  (Equal)
- `+=` (PlusEqual)
- `-=` (MinusEqual)
- `*=` (StarEqual)
- `/=` (SlashEqual)

#### Delimiters
- `(`  (LParen)
- `)`  (RParen)
- `{`  (LBrace)
- `}`  (RBrace)
- `[`  (LBracket)
- `]`  (RBracket)
- `;`  (Semicolon)
- `,`  (Comma)
- `:`  (Colon)

## Whitespace and Comments

### Whitespace
Whitespace characters (space, tab, carriage return, newline) separate tokens but are otherwise ignored.

### Comments
- **Single-line comments**: Start with `//` and extend to the end of the line.
- **Multi-line comments**: Start with `/*` and end with `*/`. Nesting is allowed (e.g., `/* outer /* inner */ outer */`).
- Comments are ignored by the lexer and do not produce tokens. They are treated as whitespace.

## Error Handling

The lexer reports errors for:
- Invalid characters (not part of any valid token).
- Unterminated string literals.
- Unterminated block comments.
- Malformed number literals (e.g., leading or trailing dot).
- Integer literals outside the 32‑bit signed range.
- Identifiers exceeding 255 characters (reported as an error; currently using a generic error message).

When an error is encountered, an `Error` token is emitted, and scanning continues after the erroneous characters.
```
