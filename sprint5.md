# **Project: MiniCompiler - Technical Requirements Document (Sprint 5)**

**Sprint Goal:** Generate x86-64 assembly code with proper function prologues/epilogues and stack frame management following the System V AMD64 ABI.

## **1. Project Structure & Repository Hygiene**

The codebase must be extended with x86-64 code generation components while maintaining existing structure and quality.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| STR-1 | All requirements from Sprint 4 (STR-1 to STR-3) **must** still be met. | Must |
| STR-2 | New source code **must** be integrated into the existing structure: | Must |
| | ```<br/>compiler-project/<br/>├── src/<br/>│ ├── lexer/          # Sprint 1<br/>│ ├── parser/         # Sprint 2<br/>│ ├── semantic/       # Sprint 3<br/>│ ├── ir/             # Sprint 4<br/>│ ├── codegen/        # New directory for Sprint 5<br/>│ │ ├── x86_generator.cpp/rs/py<br/>│ │ ├── stack_frame.cpp/rs/py<br/>│ │ ├── register_allocator.cpp/rs/py<br/>│ │ └── abi.cpp/rs/py<br/>│ ├── runtime/        # New directory for runtime library<br/>│ │ └── runtime.asm<br/>│ ├── utils/<br/>│ └── main.cpp/rs/py<br/>└── ...<br/>``` | |
| STR-3 | The `README.md` **must** be updated to include: | Must |
| | - Documentation for x86-64 code generation<br/>- System V AMD64 ABI compliance notes<br/>- Runtime library functions and linking instructions<br/>- Examples of source → assembly translation<br/>- Instructions for assembling and linking output | |

## **2. x86-64 Assembly Code Generation**

A code generator must translate IR instructions to valid x86-64 assembly following the System V ABI.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| ASM-1 | **Assembly Output** must produce valid NASM/GAS syntax: | Must |
| | - **NASM:** `mov rax, rbx`, `add rax, 5`, `call function`<br/>- **GAS:** `.intel_syntax noprefix` or AT&T syntax<br/>- Proper directives: `section .text`, `section .data`, `global`<br/>- Comments showing source correspondence | |
| ASM-2 | **Instruction Selection** must map IR to x86-64: | Must |
| | - Arithmetic: `ADD → add`, `SUB → sub`, `MUL → imul`, `DIV → idiv`<br/>- Logical: `AND → and`, `OR → or`, `NOT → not`, `XOR → xor`<br/>- Comparisons: `CMP_* → cmp + set* or cmov*`<br/>- Memory: `LOAD → mov reg, [mem]`, `STORE → mov [mem], reg`<br/>- Control Flow: `JUMP → jmp`, `JUMP_IF → jnz`, `CALL → call`, `RETURN → ret` | |
| ASM-3 | **Operand Size** must be correctly specified: | Must |
| | - 32-bit integers: `eax`, `ebx`, etc. or `dword` operations<br/>- 64-bit integers: `rax`, `rbx`, etc. or `qword` operations<br/>- 8-bit integers: `al`, `bl`, etc. or `byte` operations<br/>- Floating point: `xmm0-xmm15` registers with SSE instructions | |
| ASM-4 | **Function Translation** must handle: | Must |
| | - Function prologue: stack frame setup<br/>- Parameter passing in registers/stack per ABI<br/>- Local variable allocation on stack<br/>- Return value in `rax`/`xmm0`<br/>- Function epilogue: stack cleanup | |
| ASM-5 | **Label Generation** must create: | Must |
| | - Function labels: `function_name:`<br/>- Basic block labels: `.LBB0`, `.LBB1`, etc.<br/>- String literal labels: `.L.str0`, `.L.str1`<br/>- Data labels for global variables | |

## **3. Stack Frame Management**

Proper stack frame setup must follow System V AMD64 ABI conventions for function calls.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| STK-1 | **Function Prologue** must generate: | Must |
| | - Save caller's base pointer: `push rbp`<br/>- Set new base pointer: `mov rbp, rsp`<br/>- Allocate stack space for locals: `sub rsp, N` (N aligned to 16 bytes)<br/>- Save callee-saved registers if used (optional initially) | |
| STK-2 | **Parameter Passing** must follow System V ABI: | Must |
| | - **Integer/pointer args:** `rdi`, `rsi`, `rdx`, `rcx`, `r8`, `r9`<br/>- **Floating point args:** `xmm0`-`xmm7`<br/>- **Additional args:** on stack in right-to-left order<br/>- **Return values:** `rax`/`rdx` for integers, `xmm0`/`xmm1` for floats | |
| STK-3 | **Local Variable Allocation** must: | Must |
| | - Calculate total stack space needed for all locals<br/>- Align stack to 16 bytes before `call` instructions<br/>- Assign fixed offsets relative to `rbp` for each variable<br/>- Handle alignment requirements for different types | |
| STK-4 | **Function Epilogue** must generate: | Must |
| | - Restore stack pointer: `mov rsp, rbp` or `add rsp, N`<br/>- Restore base pointer: `pop rbp`<br/>- Return to caller: `ret`<br/>- Optional stack cleanup for caller-pops convention | |
| STK-5 | **Red Zone** consideration (128 bytes below `rsp`) should be: | Should |
| | - Utilized for leaf functions (no further calls)<br/>- Avoided for non-leaf functions that make calls<br/>- Documented in code generation strategy | |

## **4. Variable Storage & Memory Layout**

IR temporaries and variables must be mapped to appropriate storage locations (registers or stack).

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| MEM-1 | **Stack Slot Allocation** must assign: | Must |
| | - Negative offsets from `rbp` for local variables: `[rbp-8]`, `[rbp-16]`, etc.<br/>- Positive offsets for saved registers and return address: `[rbp+8]`, `[rbp+16]`<br/>- Consistent addressing for all variable accesses | |
| MEM-2 | **Temporary Storage** must handle: | Must |
| | - Spilling temporaries to stack when registers exhausted<br/>- Reusing stack slots for non-overlapping temporaries<br/>- Tracking liveness to minimize spills | |
| MEM-3 | **Global Variables** must be placed in appropriate sections: | Must |
| | - Initialized data: `section .data`<br/>- Uninitialized data: `section .bss`<br/>- Read-only data: `section .rodata`<br/>- External references: `extern` declarations | |
| MEM-4 | **Alignment** must ensure: | Should |
| | - Stack alignment to 16 bytes at function entry<br/>- Natural alignment for data types (4-byte for int, 8-byte for pointers, etc.)<br/>- SSE requirements for floating point operations | |

## **5. Runtime Library & Linking**

A minimal runtime library must provide basic I/O and system services.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| RT-1 | **Runtime Library** must include assembly functions: | Must |
| | - `print_int`: print integer to stdout (using Linux `write` syscall)<br/>- `print_string`: print null-terminated string<br/>- `read_int`: read integer from stdin<br/>- `exit`: exit program with status code<br/>- `malloc`/`free`: simple heap allocation (optional) | |
| RT-2 | **System Call Interface** must follow Linux x86-64 conventions: | Must |
| | - Syscall number in `rax`<br/>- Arguments in `rdi`, `rsi`, `rdx`, `r10`, `r8`, `r9`<br/>- Return value in `rax`<br/>- Preserve registers per ABI | |
| RT-3 | **Linking Strategy** must support: | Must |
| | - Static linking of runtime library<br/>- Position-independent code options<br/>- External library calls (e.g., `libc` functions if used) | |
| RT-4 | **Startup Code** must provide: | Should |
| | - `_start` entry point for standalone programs<br/>- Call to `main` function<br/>- Call to `exit` with return value<br/>- Handle command-line arguments if needed | |

## **6. Testing & Verification**

Comprehensive tests must validate assembly generation and execution correctness.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| TEST-1 | **Assembly Validation** must check: | Must |
| | - Generated assembly syntax is valid (can be assembled)<br/>- ABI compliance (register usage, stack alignment)<br/>- Function prologue/epilogue correctness<br/>- Control flow integrity | |
| TEST-2 | **Execution Tests** must verify: | Must |
| | - End-to-end compilation and execution of simple functions<br/>- Correct arithmetic results<br/>- Function call and return behavior<br/>- Input/output through runtime library | |
| TEST-3 | **Test Categories** must include: | Must |
| | ```<br/>tests/codegen/valid/<br/>├── arithmetic_ops/<br/>├── control_flow/<br/>├── function_calls/<br/>├── io_operations/<br/>└── integration/<br/><br/>tests/codegen/invalid/<br/>├── assembly_errors/<br/>└── runtime_errors/<br/>``` | |
| TEST-4 | **Test Execution Pipeline** must: | Must |
| | 1. Compile source to assembly<br/>2. Assemble with NASM: `nasm -f elf64 output.asm`<br/>3. Link with runtime: `ld runtime.o output.o`<br/>4. Execute and compare output with expected<br/>5. Return success/failure with diagnostics | |
| TEST-5 | **ABI Compliance Tests** must verify: | Should |
| | - Register preservation across calls<br/>- Stack alignment requirements<br/>- Parameter passing conventions<br/>- Return value handling | |

## **7. Stretch Goal: Simple Register Allocation**

A basic register allocator may be implemented to improve code quality.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| REG-1 | **Register Allocation Strategy** may use: | Optional |
| | - Fixed register assignment for common operations<br/>- Simple linear scan allocation for temporaries<br/>- Priority to frequently used values<br/>- Spill to stack when registers exhausted | |
| REG-2 | **Register Selection** should follow: | Optional |
| | - **Caller-saved:** `rax`, `rcx`, `rdx`, `rsi`, `rdi`, `r8`-`r11`<br/>- **Callee-saved:** `rbx`, `rsp`, `rbp`, `r12`-`r15`<br/>- **Special purpose:** `rax` for returns, `rcx` for shift counts, etc. | |
| REG-3 | **Allocation Algorithm** may implement: | Optional |
| | - Live range analysis for temporaries<br/>- Conflict graph construction<br/>- Graph coloring or linear scan algorithm<br/>- Spill code generation | |
| REG-4 | **Performance Metrics** should track: | Optional |
| | - Register usage statistics<br/>- Spill count reduction<br/>- Instruction count before/after allocation<br/>- Execution time improvement (estimated) | |

**System V AMD64 ABI Summary:**
```
Calling Convention:
  - First 6 integer args: RDI, RSI, RDX, RCX, R8, R9
  - First 8 floating point args: XMM0-XMM7
  - Additional args: pushed on stack (right-to-left)
  - Return: RAX (integer), XMM0 (float), RDX:RAX (64-bit)
  - Stack aligned to 16 bytes before CALL

Register Usage:
  - Caller-saved: RAX, RCX, RDX, RSI, RDI, R8-R11
  - Callee-saved: RBX, RSP, RBP, R12-R15
  - Special: RSP (stack), RBP (frame), RIP (instruction)

Stack Frame:
  - [RBP+16] = first argument (if >6 args)
  - [RBP+8]  = return address
  - [RBP]    = saved RBP
  - [RBP-8]  = first local variable
```

**Example Function Translation:**
```c
// Source: simple addition function
int add(int a, int b) {
    int result = a + b;
    return result;
}

// Generated x86-64 assembly (NASM syntax):
section .text
global add

add:
    ; Prologue
    push rbp
    mov rbp, rsp
    sub rsp, 16          ; Align stack and allocate space

    ; Parameters are in rdi (a) and rsi (b) per ABI
    ; Perform addition
    mov eax, edi         ; Move a to eax (32-bit)
    add eax, esi         ; Add b to a

    ; Store result in local variable (optional)
    mov [rbp-4], eax     ; Store to stack slot

    ; Epilogue
    mov rsp, rbp
    pop rbp
    ret                  ; Return value already in eax
```

**Runtime Library Example:**
```asm
; runtime.asm - Minimal runtime library
section .text

; print_int: prints integer in rax to stdout
global print_int
print_int:
    ; Convert integer to string
    ; Write to stdout using syscall
    mov rdi, rax        ; Integer to print
    ; ... implementation ...
    ret

; exit: exits program with status in rdi
global exit
exit:
    mov rax, 60         ; syscall: exit
    syscall
    ; Does not return

; _start: program entry point
global _start
_start:
    ; Call main function (assuming no arguments)
    call main

    ; Exit with return value from main
    mov rdi, rax
    call exit
```

**Example Usage:**
```bash
# Compile source to assembly
$ ./compiler compile --input examples/add.src --output add.asm --target x86_64

# Assemble with NASM
$ nasm -f elf64 -o add.o add.asm
$ nasm -f elf64 -o runtime.o src/runtime/runtime.asm

# Link objects
$ ld -o add_program runtime.o add.o

# Execute program
$ ./add_program
$ echo $?  # Check exit code (return value)

# Or with GCC for linking (using libc if needed)
$ gcc -no-pie -o add_program add.o runtime.o

# Run full test pipeline
$ ./compiler test --codegen
# or
$ make test-codegen
# or
$ cargo test codegen
# or
$ python -m pytest tests/codegen/
```

**Example Test Script:**
```bash
#!/bin/bash
# test_simple_function.sh

# Compile test program
./compiler compile --input tests/codegen/valid/simple_add.src --output test.asm

# Assemble and link
nasm -f elf64 test.asm -o test.o
nasm -f elf64 src/runtime/runtime.asm -o runtime.o
ld -o test_program runtime.o test.o

# Execute and capture result
./test_program
result=$?

# Check expected result (2 + 3 = 5)
if [ $result -eq 5 ]; then
    echo "✓ Test passed: simple addition"
    exit 0
else
    echo "✗ Test failed: expected 5, got $result"
    exit 1
fi
```

**Stack Frame Visualization:**
```
High Addresses
+-----------------+
|   ...           |
+-----------------+
| 7th argument    |  [rbp+32]  (if applicable)
+-----------------+
| 6th argument    |  [rbp+24]  (on stack)
+-----------------+
| Return Address  |  [rbp+16]
+-----------------+
| Saved RBP       |  [rbp]  ← RBP points here
+-----------------+
| Local var 1     |  [rbp-8]
+-----------------+
| Local var 2     |  [rbp-16]
+-----------------+
| ...             |
+-----------------+
| Spill slot 1    |  [rbp-24]
+-----------------+
| Spill slot 2    |  [rbp-32]
+-----------------+
| Red Zone        |  [rsp-128] to [rsp] (if used)
+-----------------+
|                 |  ← RSP points here
Low Addresses
```

**Register Allocation Example:**
```
Before allocation (stack-heavy):
  mov eax, [rbp-8]     ; Load a
  add eax, [rbp-16]    ; Add b from stack
  mov [rbp-24], eax    ; Store result to stack

After allocation (using registers):
  mov eax, edi         ; a already in register
  add eax, esi         ; b already in register
  ; No stack access needed
```
