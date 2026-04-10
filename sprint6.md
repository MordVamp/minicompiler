# **Project: MiniCompiler - Technical Requirements Document (Sprint 6)**

**Sprint Goal:** Extend the code generator to handle complex control flow structures (conditionals, loops) and implement short-circuit evaluation for logical operators.

## **1. Project Structure & Repository Hygiene**

The codebase must be extended to support complex control flow while maintaining existing structure and quality.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| STR-1 | All requirements from Sprint 5 (STR-1 to STR-3) **must** still be met. | Must |
| STR-2 | Code generation components **must** be extended in the existing structure: | Must |
| | ```<br/>compiler-project/<br/>├── src/<br/>│ ├── codegen/        # Extended for Sprint 6<br/>│ │ ├── control_flow_generator.cpp/rs/py  # New file<br/>│ │ ├── expression_generator.cpp/rs/py    # Enhanced<br/>│ │ ├── x86_generator.cpp/rs/py           # Updated<br/>│ │ └── label_manager.cpp/rs/py           # Enhanced<br/>│ └── ...<br/>└── ...<br/>``` | |
| STR-3 | The `README.md` **must** be updated to include: | Must |
| | - Documentation for new control flow constructs<br/>- Examples of generated assembly for if/else, loops<br/>- Short-circuit evaluation behavior<br/>- Updated test instructions for control flow features | |

## **2. Conditional Statements Implementation**

The code generator must translate if and if-else statements to correct x86-64 conditional jump sequences.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| COND-1 | **If Statement Translation** must generate: | Must |
| | - Condition evaluation producing boolean result in register<br/>- Conditional jump to else block or past if body<br/>- Basic block for then-body with proper label<br/>- Unconditional jump to merge point after then-body | |
| COND-2 | **If-Else Statement Translation** must generate: | Must |
| | - Condition evaluation and conditional jump to else block<br/>- Then-body block with jump to merge point<br/>- Else-body block with jump to merge point<br/>- Merge point label for code after the conditional | |
| COND-3 | **Relational Operators** must generate correct x86-64 comparisons: | Must |
| | - `<` → `jl` (jump if less) for signed, `jb` for unsigned<br/>- `<=` → `jle` (jump if less or equal)<br/>- `>` → `jg` (jump if greater)<br/>- `>=` → `jge` (jump if greater or equal)<br/>- `==` → `je` (jump if equal)<br/>- `!=` → `jne` (jump if not equal) | |
| COND-4 | **Nested Conditionals** must be handled correctly: | Must |
| | - Proper label generation to avoid conflicts<br/>- Correct scoping of conditional blocks<br/>- Preservation of condition results across nested evaluations | |
| COND-5 | **Conditional Expression Types** must support: | Must |
| | - Integer comparisons (signed and unsigned)<br/>- Floating point comparisons (using `ucomisd` + `jp`/`jnp`)<br/>- Boolean value direct testing<br/>- Pointer comparisons (for equality/inequality) | |

## **3. Loop Statements Implementation**

The code generator must translate while and for loops to proper x86-64 loop structures.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| LOOP-1 | **While Loop Translation** must generate: | Must |
| | - Loop header label<br/>- Condition evaluation at loop start<br/>- Conditional jump to exit if condition false<br/>- Loop body block<br/>- Unconditional jump back to header<br/>- Loop exit label | |
| LOOP-2 | **For Loop Translation** must generate equivalent to: | Must |
| | ```<br/>init_statement<br/>while (condition) {<br/>    body<br/>    update_statement<br/>}<br/>``` | |
| | - Initialization code before loop<br/>- Condition evaluation at each iteration<br/>- Update code at end of loop body<br/>- Proper jump structure | |
| LOOP-3 | **Loop Optimization** should generate efficient code: | Should |
| | - Move loop-invariant computations outside loop when possible<br/>- Use counted loop patterns when bounds known<br/>- Minimize jumps within loop body | |
| LOOP-4 | **Loop Variable Handling** must: | Must |
| | - Allocate loop variables with appropriate scope<br/>- Handle modifications to loop variables in body<br/>- Prevent undefined behavior with loop control variables | |

## **4. Logical Operators with Short-Circuit Evaluation**

Logical operators must implement short-circuit evaluation semantics.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| LOGIC-1 | **AND Operator (`&&`)** must implement short-circuit: | Must |
| | - Evaluate left operand<br/>- Jump to false label if left operand is false<br/>- Evaluate right operand only if left is true<br/>- Combine results appropriately | |
| LOGIC-2 | **OR Operator (`||`)** must implement short-circuit: | Must |
| | - Evaluate left operand<br/>- Jump to true label if left operand is true<br/>- Evaluate right operand only if left is false<br/>- Combine results appropriately | |
| LOGIC-3 | **NOT Operator (`!`)** must generate: | Must |
| | - Boolean negation via `xor` instruction or comparison inversion<br/>- Efficient code without branching when possible | |
| LOGIC-4 | **Complex Boolean Expressions** must: | Must |
| | - Handle arbitrary nesting of logical operators<br/>- Preserve short-circuit semantics at all levels<br/>- Generate minimal jump instructions<br/>- Use efficient truth value representation (0/1) | |

## **5. Complex Expression Code Generation**

The code generator must handle nested expressions with proper operator precedence and efficient register usage.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| EXPR-1 | **Operator Precedence** must be correctly implemented: | Must |
| | 1. Primary (literals, identifiers, parentheses)<br/>2. Unary (`-`, `!`)<br/>3. Multiplicative (`*`, `/`, `%`)<br/>4. Additive (`+`, `-`)<br/>5. Relational (`<`, `<=`, `>`, `>=`)<br/>6. Equality (`==`, `!=`)<br/>7. Logical AND (`&&`)<br/>8. Logical OR (`||`)<br/>9. Assignment (`=`, `+=`, etc.) | |
| EXPR-2 | **Expression Trees** must be traversed efficiently: | Must |
| | - Post-order traversal for arithmetic expressions<br/>- In-order with short-circuit for logical expressions<br/>- Minimal temporary register usage<br/>- Common subexpression elimination when possible | |
| EXPR-3 | **Type-Promotion** must handle mixed-type expressions: | Must |
| | - `int` + `float` → `float` with conversion<br/>- Integer promotion for smaller types<br/>- Sign extension for signed operations<br/>- Zero extension for unsigned operations | |
| EXPR-4 | **Complex Operands** must support: | Must |
| | - Array element access<br/>- Structure field access<br/>- Function call results as operands<br/>- Parenthesized subexpressions | |

## **6. Testing & Verification**

Comprehensive tests must validate control flow correctness and expression evaluation.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| TEST-1 | **Control Flow Tests** must cover: | Must |
| | - Simple if statements (true and false paths)<br/>- If-else statements with both branches<br/>- Nested conditionals (if within if)<br/>- While loops (zero, one, multiple iterations)<br/>- For loops with various initialization/condition/update<br/>- Mixed control flow (loops with conditionals inside) | |
| TEST-2 | **Logical Operator Tests** must verify: | Must |
| | - Short-circuit behavior (right operand not evaluated when short-circuited)<br/>- Truth tables for all logical operators<br/>- Nested logical expressions<br/>- Mixed arithmetic and logical expressions | |
| TEST-3 | **Expression Tests** must validate: | Must |
| | - Operator precedence correctness<br/>- Associativity (left for most, right for assignment)<br/>- Complex expression evaluation order<br/>- Type promotion in mixed expressions | |
| TEST-4 | **Test Execution** must include: | Must |
| | ```<br/>tests/control_flow/valid/<br/>├── conditionals/<br/>├── loops/<br/>├── logical_ops/<br/>└── complex_expressions/<br/><br/>tests/control_flow/invalid/<br/>├── infinite_loops/      # Detection optional<br/>└── type_errors/<br/>``` | |
| TEST-5 | **Integration Tests** must: | Should |
| | - Compile and run complete programs with control flow<br/>- Verify program output matches expected behavior<br/>- Test edge cases (empty loop bodies, null statements) | |

## **7. Stretch Goal: Break and Continue Statements**

Optional implementation of loop control statements for enhanced language support.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| LOOP-CTRL-1 | **Break Statement** may implement: | Optional |
| | - Jump to loop exit point from within loop body<br/>- Handling within nested loops (break to correct level)<br/>- Validation that break is within loop context<br/>- Generation of appropriate unconditional jump | |
| LOOP-CTRL-2 | **Continue Statement** may implement: | Optional |
| | - Jump to loop update/condition evaluation point<br/>- Handling within nested loops (continue correct loop)<br/>- Validation that continue is within loop context<br/>- Generation of appropriate unconditional jump | |
| LOOP-CTRL-3 | **Scope Management** must handle: | Optional |
| | - Variable cleanup when breaking/continuing<br/>- Preservation of loop state across continue<br/>- Proper stack unwinding if needed | |
| LOOP-CTRL-4 | **Switch Statements** could be added: | Optional |
| | - Jump table generation for dense case values<br/>- Sequential comparison for sparse case values<br/>- Default case handling<br/>- Fall-through behavior (with or without explicit) | |

**Control Flow Translation Examples:**

```c
// Source: if-else statement
if (x > 0) {
    y = 10;
} else {
    y = 20;
}

// Generated x86-64 assembly:
    ; Evaluate condition: x > 0
    mov eax, [rbp-8]      ; Load x
    cmp eax, 0
    jle .Lelse            ; Jump to else if x <= 0

    ; Then branch
    mov dword [rbp-12], 10 ; y = 10
    jmp .Lendif           ; Skip else branch

.Lelse:
    ; Else branch
    mov dword [rbp-12], 20 ; y = 20

.Lendif:
    ; Continue after if-else
```

```c
// Source: while loop
int i = 0;
while (i < 10) {
    sum = sum + i;
    i = i + 1;
}

// Generated x86-64 assembly:
    mov dword [rbp-4], 0   ; i = 0

.Lwhile_cond:
    ; Condition: i < 10
    mov eax, [rbp-4]       ; Load i
    cmp eax, 10
    jge .Lwhile_end        ; Exit if i >= 10

    ; Loop body
    mov eax, [rbp-8]       ; Load sum
    add eax, [rbp-4]       ; Add i
    mov [rbp-8], eax       ; Store sum

    ; Increment i
    mov eax, [rbp-4]
    add eax, 1
    mov [rbp-4], eax

    jmp .Lwhile_cond       ; Loop back

.Lwhile_end:
```

**Short-Circuit Evaluation Example:**
```c
// Source: if (a != 0 && b / a > 2)
// Short-circuit: b/a not evaluated if a == 0

// Generated x86-64 assembly:
    ; Evaluate a != 0
    mov eax, [rbp-8]       ; Load a
    cmp eax, 0
    je .Lfalse             ; If a == 0, short-circuit to false

    ; Evaluate b / a > 2 (only reached if a != 0)
    mov eax, [rbp-12]      ; Load b
    cdq                    ; Sign extend eax to edx:eax
    idiv dword [rbp-8]     ; Divide by a
    cmp eax, 2
    jg .Ltrue              ; If b/a > 2, result is true
    jmp .Lfalse

.Ltrue:
    mov eax, 1
    jmp .Lend

.Lfalse:
    mov eax, 0

.Lend:
    ; eax contains boolean result
```

**Complex Expression Example:**
```c
// Source: result = (a + b) * (c - d) / (e % f)
// Demonstrates operator precedence and parentheses

// Generated x86-64 assembly:
    ; Compute (a + b)
    mov eax, [rbp-8]       ; Load a
    add eax, [rbp-12]      ; Add b
    mov [rbp-28], eax      ; Store temp1 = a + b

    ; Compute (c - d)
    mov eax, [rbp-16]      ; Load c
    sub eax, [rbp-20]      ; Subtract d
    mov [rbp-32], eax      ; Store temp2 = c - d

    ; Compute (e % f)
    mov eax, [rbp-24]      ; Load e
    cdq                    ; Sign extend
    idiv dword [rbp-28]    ; Divide by f
    mov [rbp-36], edx      ; Store remainder temp3 = e % f

    ; Compute temp1 * temp2
    mov eax, [rbp-28]      ; Load temp1
    imul eax, [rbp-32]     ; Multiply by temp2

    ; Divide by temp3
    cdq                    ; Sign extend eax to edx:eax
    idiv dword [rbp-36]    ; Divide by temp3

    ; Store final result
    mov [rbp-4], eax       ; result = (a+b)*(c-d)/(e%f)
```

**Example Usage:**
```bash
# Compile program with control flow
$ ./compiler compile --input examples/factorial.src --output factorial.asm

# Assemble and run
$ nasm -f elf64 factorial.asm -o factorial.o
$ nasm -f elf64 src/runtime/runtime.asm -o runtime.o
$ ld -o factorial_program runtime.o factorial.o
$ ./factorial_program

# Run control flow tests
$ ./compiler test --control-flow
# or
$ make test-control-flow
# or
$ cargo test control_flow
# or
$ python -m pytest tests/control_flow/
```

**Test Case Example:**
```c
// test_nested_loops.src
int main() {
    int sum = 0;
    for (int i = 0; i < 5; i = i + 1) {
        for (int j = 0; j < 3; j = j + 1) {
            if (i == j) {
                sum = sum + i * j;
            }
        }
    }
    return sum;
}

// Expected result: when i == j (0,0), (1,1), (2,2)
// sum = 0*0 + 1*1 + 2*2 = 0 + 1 + 4 = 5
```

**Break/Continue Implementation (Stretch Goal):**
```c
// Source: while with break and continue
int i = 0;
int sum = 0;
while (true) {
    i = i + 1;
    if (i > 10) {
        break;           // Exit loop
    }
    if (i % 2 == 0) {
        continue;        // Skip even numbers
    }
    sum = sum + i;
}
// sum = 1 + 3 + 5 + 7 + 9 = 25

// Generated assembly with break/continue:
    mov dword [rbp-4], 0   ; i = 0
    mov dword [rbp-8], 0   ; sum = 0

.Lloop_start:
    ; i = i + 1
    mov eax, [rbp-4]
    add eax, 1
    mov [rbp-4], eax

    ; if (i > 10) break;
    cmp eax, 10
    jg .Lloop_end

    ; if (i % 2 == 0) continue;
    mov eax, [rbp-4]
    and eax, 1            ; i % 2 (fast modulo for power of 2)
    jz .Lloop_start       ; If even, continue to next iteration

    ; sum = sum + i
    mov eax, [rbp-8]
    add eax, [rbp-4]
    mov [rbp-8], eax

    jmp .Lloop_start

.Lloop_end:
    ; Loop exited via break
```
