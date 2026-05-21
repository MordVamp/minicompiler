# **Project: MiniCompiler - Technical Requirements Document (Sprint 7)**

**Sprint Goal:** Integrate advanced language features (arrays, external calls) and implement optimization passes to improve generated code quality.

## **1. Project Structure & Repository Hygiene**

The codebase must be extended with array support, external function integration, and optimization components while maintaining existing structure.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| STR-1 | All requirements from Sprint 6 (STR-1 to STR-3) **must** still be met. | Must |
| STR-2 | New source code **must** be integrated into the existing structure: | Must |
| | ```<br/>compiler-project/<br/>├── src/<br/>│ ├── codegen/        # Extended for Sprint 7<br/>│ │ ├── array_generator.cpp/rs/py     # New file<br/>│ │ ├── external_calls.cpp/rs/py      # New file<br/>│ │ └── optimization_passes.cpp/rs/py # New file<br/>│ ├── ir/<br/>│ │ └── optimizer.cpp/rs/py          # Enhanced for IR optimizations<br/>│ ├── libc/          # New directory for C library bindings<br/>│ │ └── stdlib.h<br/>│ └── ...<br/>└── ...<br/>``` | |
| STR-3 | The `README.md` **must** be updated to include: | Must |
| | - Documentation for array syntax and memory layout<br/>- Instructions for linking with C standard library<br/>- Description of implemented optimizations<br/>- Examples of optimized code generation<br/>- Building and running the demo program | |

## **2. Array Support Implementation**

The compiler must support static and stack-allocated arrays with proper bounds checking and element access.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| ARR-1 | **Array Declaration Syntax** must support: | Must |
| | - Static arrays: `int arr[10];`<br/>- Initialized arrays: `int arr[3] = {1, 2, 3};`<br/>- Multi-dimensional arrays: `int matrix[3][4];`<br/>- Array parameters: `void foo(int arr[], int size)` | |
| ARR-2 | **Memory Allocation** must handle: | Must |
| | - Global arrays in `.data` or `.bss` sections<br/>- Local arrays on the stack with proper alignment<br/>- Array size calculation at compile time when possible<br/>- Padding for alignment requirements | |
| ARR-3 | **Array Element Access** must generate: | Must |
| | - Address calculation: `base + index * element_size`<br/>- Bounds checking (optional for performance)<br/>- Proper pointer arithmetic for array decay<br/>- Element loading/storing with correct size | |
| ARR-4 | **Array Operations** must support: | Must |
| | - Array initialization loops<br/>- Array copying (element-wise or memcpy)<br/>- Passing arrays to functions (as pointers)<br/>- Returning arrays (via pointers only) | |
| ARR-5 | **Bounds Checking** may implement: | Optional |
| | - Compile-time bounds checking for constant indices<br/>- Runtime bounds checking with error reporting<br/>- Configurable checking levels (full, minimal, none)<br/>- Optimized bounds checks for loop indices | |

## **3. System Integration & External Function Calls**

The compiler must support calling C standard library functions and other external functions following the System V ABI.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| EXT-1 | **External Function Declarations** must support: | Must |
| | - Function prototypes: `extern int printf(char* format, ...);`<br/>- Variable argument lists for functions like `printf`<br/>- Type matching between declaration and call<br/>- Standard library headers simulation | |
| EXT-2 | **Calling Convention Compliance** must: | Must |
| | - Follow System V AMD64 ABI for all external calls<br/>- Handle variadic functions correctly (AL register for SSE registers used)<br/>- Ensure stack alignment to 16 bytes before call<br/>- Preserve caller-saved registers appropriately | |
| EXT-3 | **Common Library Functions** must support: | Must |
| | - I/O: `printf`, `scanf`, `puts`, `getchar`<br/>- Memory: `malloc`, `free`, `memcpy`, `memset`<br/>- Math: `pow`, `sqrt`, `sin`, `cos`<br/>- String: `strlen`, `strcpy`, `strcmp` | |
| EXT-4 | **Linking Configuration** must: | Must |
| | - Generate assembly compatible with C linker<br/>- Support both static and dynamic linking<br/>- Handle name mangling or C name decoration<br/>- Provide necessary runtime initialization | |
| EXT-5 | **Error Handling for External Calls** must: | Should |
| | - Validate argument counts and types where possible<br/>- Handle null pointer returns from functions like `malloc`<br/>- Provide meaningful error messages for linkage failures | |

## **4. Basic Optimizations Implementation**

At least one optimization pass must be implemented to improve generated code quality.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| OPT-1 | **Constant Folding** must optimize: | Must |
| | - Arithmetic expressions with constant operands: `3 + 4 → 7`<br/>- Logical expressions with constants: `true && x → x`<br/>- Comparison of constants: `5 > 3 → true`<br/>- Type conversions of constants | |
| OPT-2 | **Constant Propagation** must: | Must |
| | - Propagate constant values through assignments<br/>- Replace variable uses with known constant values<br/>- Handle conditional branches with constant conditions<br/>- Track constants across basic blocks where safe | |
| OPT-3 | **Dead Code Elimination** must remove: | Must |
| | - Unreachable code after unconditional jumps<br/>- Unused variable assignments<br/>- Code with no side effects whose results are unused<br/>- Empty basic blocks | |
| OPT-4 | **Optimization Pipeline** must: | Should |
| | - Apply optimizations in correct order (e.g., constant folding before propagation)<br/>- Iterate until no more optimizations apply<br/>- Provide statistics on optimizations performed<br/>- Allow disabling/enabling specific optimizations | |
| OPT-5 | **Local Optimizations** may include: | Optional |
| | - Common subexpression elimination within basic blocks<br/>- Strength reduction (e.g., `x * 2 → x + x`)<br/>- Algebraic simplifications (e.g., `x + 0 → x`)<br/>- Jump threading and basic block merging | |

## **5. Non-Trivial Demo Program**

A complete, non-trivial program must be implemented to demonstrate compiler capabilities.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| DEMO-1 | **Demo Program Requirements** must: | Must |
| | - Be sufficiently complex to exercise multiple language features<br/>- Include at least one recursive function<br/>- Use arrays and/or structures<br/>- Demonstrate control flow (loops, conditionals)<br/>- Produce verifiable output | |
| DEMO-2 | **Example Programs** could include: | Must |
| | - Recursive factorial or Fibonacci with memoization<br/>- Sorting algorithm (bubble sort, quicksort)<br/>- Matrix multiplication<br/>- Simple text processing (word count, palindrome check)<br/>- Linked list or binary tree operations | |
| DEMO-3 | **Documentation** must include: | Must |
| | - Source code of the demo program<br/>- Expected output and verification method<br/>- Performance characteristics (time/space complexity)<br/>- Instructions for compiling and running | |
| DEMO-4 | **Benchmarking** should provide: | Should |
| | - Comparison of optimized vs. non-optimized code<br/>- Memory usage statistics<br/>- Execution time measurements<br/>- Code size metrics | |

## **6. Testing & Verification**

Comprehensive tests must validate array operations, external calls, and optimization correctness.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| TEST-1 | **Array Tests** must cover: | Must |
| | - Array declaration and initialization<br/>- Element access and modification<br/>- Array bounds (valid and invalid accesses)<br/>- Multi-dimensional arrays<br/>- Arrays as function parameters | |
| TEST-2 | **External Call Tests** must verify: | Must |
| | - Simple printf/scanf functionality<br/>- Memory allocation with malloc/free<br/>- Math library function calls<br/>- String manipulation functions<br/>- Error handling for invalid calls | |
| TEST-3 | **Optimization Tests** must validate: | Must |
| | - Constant folding produces correct results<br/> - Constant propagation doesn't break program semantics<br/>- Dead code elimination removes only unreachable code<br/>- Optimization doesn't introduce new bugs | |
| TEST-4 | **Integration Tests** must: | Must |
| | - Test the complete demo program end-to-end<br/>- Verify optimization improves performance or reduces code size<br/>- Ensure external library linking works correctly<br/>- Test edge cases and error conditions | |
| TEST-5 | **Performance Tests** should: | Should |
| | - Compare execution time before/after optimizations<br/>- Measure code size reduction<br/>- Profile memory usage patterns<br/>- Validate optimization effectiveness metrics | |

## **7. Stretch Goal: Simple Function Inlining**

Optional implementation of function inlining for small functions to reduce call overhead.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| INLINE-1 | **Inlining Criteria** may consider: | Optional |
| | - Function size (number of IR instructions)<br/>- Call frequency (hot functions)<br/>- Recursion depth (avoid infinite inlining)<br/>- Parameter count and complexity | |
| INLINE-2 | **Inlining Process** must: | Optional |
| | - Duplicate function body at call site<br/>- Rename local variables to avoid conflicts<br/>- Replace parameters with actual arguments<br/>- Handle return statements appropriately<br/>- Update control flow graphs | |
| INLINE-3 | **Limitations and Safety** must address: | Optional |
| | - Avoid inlining recursive functions without depth limit<br/>- Respect function visibility (private vs. external)<br/>- Handle variable arguments correctly<br/>- Preserve debugging information where possible | |
| INLINE-4 | **Performance Analysis** should: | Optional |
| | - Measure call overhead reduction<br/>- Analyze code size increase<br/>- Consider cache locality effects<br/>- Provide heuristic tuning parameters | |

**Array Implementation Examples:**

```c
// Source: Array operations
int sum_array(int arr[], int size) {
    int total = 0;
    for (int i = 0; i < size; i = i + 1) {
        total = total + arr[i];
    }
    return total;
}

// Generated x86-64 assembly for array access:
sum_array:
    push rbp
    mov rbp, rsp
    sub rsp, 32

    ; Parameters: rdi = arr pointer, rsi = size
    mov [rbp-8], rdi    ; Save arr
    mov [rbp-16], esi   ; Save size (32-bit)
    mov dword [rbp-20], 0 ; total = 0
    mov dword [rbp-24], 0 ; i = 0

.loop_start:
    mov eax, [rbp-24]   ; Load i
    cmp eax, [rbp-16]   ; Compare i with size
    jge .loop_end

    ; Calculate arr[i] address: arr + i * 4 (int size)
    mov rax, [rbp-8]    ; Load arr base address
    mov edx, [rbp-24]   ; Load i
    movsxd rdx, edx     ; Sign extend to 64-bit
    shl rdx, 2          ; Multiply by 4 (sizeof(int))
    add rax, rdx        ; arr + i*4

    ; Load arr[i] and add to total
    mov edx, [rax]      ; Load arr[i]
    add [rbp-20], edx   ; total += arr[i]

    ; Increment i
    inc dword [rbp-24]
    jmp .loop_start

.loop_end:
    mov eax, [rbp-20]   ; Return total
    mov rsp, rbp
    pop rbp
    ret
```

**External Function Call Example:**
```c
// Source: Using printf from C library
extern int printf(char* format, ...);

int main() {
    int x = 42;
    printf("The answer is %d\n", x);
    return 0;
}

// Generated x86-64 assembly with external call:
section .data
.format db "The answer is %d", 10, 0  ; 10 = newline, 0 = null terminator

section .text
extern printf
global main

main:
    push rbp
    mov rbp, rsp
    sub rsp, 16         ; Align stack + allocate locals

    mov dword [rbp-4], 42  ; x = 42

    ; Prepare printf arguments
    lea rdi, [rel .format]  ; First arg: format string
    mov esi, [rbp-4]        ; Second arg: x
    xor eax, eax            ; For variadic: AL = number of vector registers used (0 for integer args)
    call printf

    ; Return 0
    xor eax, eax
    mov rsp, rbp
    pop rbp
    ret
```

**Optimization Example:**
```c
// Source before optimization:
int compute() {
    int x = 10 + 20;        // Constant expression
    int y = x * 2;          // Uses constant x
    if (y > 50) {           // Constant condition (60 > 50)
        return 1;
    } else {
        return 0;           // Dead code after constant propagation
    }
}

// IR before optimization:
  t1 = ADD 10, 20      ; 30
  STORE [x], t1
  t2 = LOAD [x]
  t3 = MUL t2, 2       ; 60
  STORE [y], t3
  t4 = LOAD [y]
  t5 = CMP_GT t4, 50   ; true
  JUMP_IF t5, L_true
  JUMP L_false
L_true:
  RETURN 1
L_false:
  RETURN 0

// After constant folding and propagation:
  t1 = 30              ; Folded: 10 + 20
  STORE [x], 30
  t3 = 60              ; Folded: 30 * 2
  STORE [y], 60
  t5 = true            ; Folded: 60 > 50
  JUMP_IF true, L_true ; Conditional jump becomes unconditional
  JUMP L_false         ; Dead code
L_true:
  RETURN 1
L_false:               ; Unreachable
  RETURN 0

// After dead code elimination:
  RETURN 1             ; All unnecessary code eliminated
```

**Demo Program Example (Quicksort):**
```c
// demo/quicksort.src - Non-trivial demo program
extern void printf(char* format, ...);

void swap(int* a, int* b) {
    int temp = *a;
    *a = *b;
    *b = temp;
}

int partition(int arr[], int low, int high) {
    int pivot = arr[high];
    int i = low - 1;

    for (int j = low; j < high; j = j + 1) {
        if (arr[j] <= pivot) {
            i = i + 1;
            swap(&arr[i], &arr[j]);
        }
    }
    swap(&arr[i + 1], &arr[high]);
    return i + 1;
}

void quicksort(int arr[], int low, int high) {
    if (low < high) {
        int pi = partition(arr, low, high);
        quicksort(arr, low, pi - 1);
        quicksort(arr, pi + 1, high);
    }
}

void print_array(int arr[], int size) {
    for (int i = 0; i < size; i = i + 1) {
        printf("%d ", arr[i]);
    }
    printf("\n");
}

int main() {
    int arr[8] = {10, 7, 8, 9, 1, 5, 3, 6};
    int size = 8;

    printf("Original array: ");
    print_array(arr, size);

    quicksort(arr, 0, size - 1);

    printf("Sorted array: ");
    print_array(arr, size);

    return 0;
}
```

**Example Usage:**
```bash
# Compile the demo program
$ ./compiler compile --input demo/quicksort.src --output quicksort.asm --optimize

# Assemble with NASM
$ nasm -f elf64 quicksort.asm -o quicksort.o

# Link with C library (for printf)
$ gcc -no-pie -o quicksort_program quicksort.o

# Run the demo
$ ./quicksort_program
Original array: 10 7 8 9 1 5 3 6
Sorted array: 1 3 5 6 7 8 9 10

# Run optimization tests
$ ./compiler test --optimization
# or
$ make test-optimization
# or
$ cargo test optimization
# or
$ python -m pytest tests/optimization/

# View optimization statistics
$ ./compiler compile --input demo/quicksort.src --stats --optimize
Optimization Report:
  Constant folding: 15 expressions folded
  Constant propagation: 8 variables propagated
  Dead code elimination: 12 instructions removed
  Total instructions: 247 → 198 (19.8% reduction)
```

**Function Inlining Example (Stretch Goal):**
```c
// Source with small function
inline int square(int x) {
    return x * x;
}

int sum_of_squares(int a, int b) {
    return square(a) + square(b);
}

// Without inlining:
square:
    mov eax, edi
    imul eax, edi
    ret

sum_of_squares:
    push rbp
    mov rbp, rsp
    sub rsp, 16
    mov [rbp-4], edi
    mov [rbp-8], esi
    mov edi, [rbp-4]
    call square
    mov [rbp-12], eax
    mov edi, [rbp-8]
    call square
    add eax, [rbp-12]
    mov rsp, rbp
    pop rbp
    ret

// With inlining:
sum_of_squares:
    push rbp
    mov rbp, rsp

    ; Inlined square(a)
    mov eax, edi
    imul eax, edi
    mov [rbp-4], eax    ; temp1 = a*a

    ; Inlined square(b)
    mov eax, esi
    imul eax, esi       ; eax = b*b

    ; Add results
    add eax, [rbp-4]    ; eax = a*a + b*b

    mov rsp, rbp
    pop rbp
    ret
```
