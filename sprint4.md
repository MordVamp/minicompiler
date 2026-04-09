# **Project: MiniCompiler - Technical Requirements Document (Sprint 4)**

**Sprint Goal:** Design and implement an Intermediate Representation (IR) in Static Single Assignment (SSA) form and generate simple code sequences from the decorated AST.

## **1. Project Structure & Repository Hygiene**

The codebase must be extended with IR generation components while maintaining existing structure and quality.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| STR-1 | All requirements from Sprint 3 (STR-1 to STR-3) **must** still be met. | Must |
| STR-2 | New source code **must** be integrated into the existing structure: | Must |
| | ```<br/>compiler-project/<br/>├── src/<br/>│ ├── lexer/          # Sprint 1<br/>│ ├── parser/         # Sprint 2<br/>│ ├── semantic/       # Sprint 3<br/>│ ├── ir/             # New directory for Sprint 4<br/>│ │ ├── ir_generator.cpp/rs/py<br/>│ │ ├── ir_instructions.cpp/rs/py<br/>│ │ ├── basic_block.cpp/rs/py<br/>│ │ └── control_flow.cpp/rs/py<br/>│ ├── codegen/        # Future sprint for assembly<br/>│ ├── utils/<br/>│ └── main.cpp/rs/py<br/>└── ...<br/>``` | |
| STR-3 | The `README.md` **must** be updated to include: | Must |
| | - Documentation for the IR format and instruction set<br/>- New command-line options for IR generation<br/>- Examples of source code → IR transformation<br/>- Instructions for viewing/analyzing IR output | |

## **2. Intermediate Representation Design**

A platform-independent IR must be designed to represent the program at an abstraction level suitable for optimization and code generation.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| IR-1 | **IR Instruction Set** must define a three-address code format in Static Single Assignment (SSA) form: | Must |
| | - **Arithmetic:** `ADD dest, src1, src2`, `SUB`, `MUL`, `DIV`, `MOD`<br/>- **Logical:** `AND`, `OR`, `NOT`, `XOR`<br/>- **Comparison:** `CMP_EQ`, `CMP_NE`, `CMP_LT`, `CMP_LE`, `CMP_GT`, `CMP_GE`<br/>- **Memory:** `LOAD dest, addr`, `STORE addr, src`<br/>- **Control Flow:** `JUMP label`, `JUMP_IF cond, label`, `JUMP_IF_NOT cond, label`<br/>- **Function:** `CALL dest, func, args...`, `RETURN value`, `PARAM index, value`<br/>- **Data Movement:** `MOVE dest, src`, `PHI dest, (val1, block1), (val2, block2)...` | |
| IR-2 | **Operand Types** must support strict SSA versioning: | Must |
| | - **Temporaries/Variables:** `t1`, `x_1`, `y_2` (all variables and temporaries must be versioned to strictly follow SSA properties)<br/>- **Literals:** `42`, `3.14`, `true`<br/>- **Labels:** `L1`, `L2`, ... (basic block labels)<br/>- **Memory Locations:** `[t1]`, `[t2+4]` | |
| IR-3 | **Basic Block Structure** must organize code into: | Must |
| | - **Entry Block:** Single entry point<br/>- **Basic Blocks:** Sequential instructions ending with control flow<br/>- **Exit Block:** Single or multiple exit points<br/>- **Control Flow Graph (CFG)** edges between blocks | |
| IR-4 | **Function Representation** must include: | Must |
| | - Function name and return type<br/>- Parameter list with types<br/>- Local variable declarations<br/>- Basic blocks forming the CFG<br/>- Symbol table mapping for variables → temporaries | |
| IR-5 | **Type Information** must be preserved: | Should |
| | - Type annotations on temporaries and operations<br/>- Size information for memory operations<br/>- Sign information for integer operations | |

## **3. IR Generator Implementation**

An IR generator must traverse the decorated AST to produce the intermediate representation.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| GEN-1 | **Generator Interface** must provide: | Must |
| | - `IRGenerator(symbol_table, type_system)` constructor<br/>- `generate(ast: ProgramNode)`: produce IR for entire program<br/>- `get_function_ir(name)`: retrieve IR for specific function<br/>- `get_all_ir()`: return complete IR program | |
| GEN-2 | **Expression Translation** must convert: | Must |
| | - Literals → constant operands<br/>- Variables → loads from memory or temporaries<br/>- Arithmetic expressions → sequence of 3-address operations<br/>- Function calls → CALL instructions with parameter setup<br/>- Field access → address calculation and load | |
| GEN-3 | **Statement Translation** must handle: | Must |
| | - Assignments → computation + store operations<br/>- Variable declarations → memory allocation + optional initialization<br/>- If statements → conditional jumps + basic blocks<br/>- While loops → loop header + body + back edge<br/>- Return statements → value computation + RETURN instruction | |
| GEN-4 | **Control Flow & SSA Construction** must construct: | Must |
| | - Basic blocks for linear code sequences<br/>- Proper block ordering for structured control flow and dominance frontier calculation<br/>- Jump instructions for conditional branches<br/>- PHI nodes for values merging at join points (using SSA construction algorithms) | |
| GEN-5 | **Variable Management (SSA)** must: | Must |
| | - Allocate uniquely versioned temporaries for expression results<br/>- Map source variables to SSA-versioned variables (e.g., `x_1`, `x_2`) instead of raw memory locations for locals<br/>- Ensure every variable is assigned exactly once<br/>- Track live ranges of SSA variables (optional for optimization) | |

## **4. IR Output & Visualization**

The IR must be serializable to human-readable formats for inspection and debugging.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| OUT-1 | **Textual Dump** must output IR in readable format: | Must |
| | - One instruction per line<br/>- Basic blocks clearly marked with labels<br/>- Comments showing source code correspondence<br/>- Example format:<br/>```<br/>function main: void ()<br/>  entry:<br/>    t1 = MUL 2, 3          # 2 * 3<br/>    t2 = ADD t1, 4        # + 4<br/>    STORE [x], t2         # x = result<br/>    JUMP L1<br/>  L1:<br/>    ...<br/>``` | |
| OUT-2 | **Graphviz Output** must generate CFG visualization: | Must |
| | - Nodes for basic blocks with instructions<br/>- Edges showing control flow<br/>- Color coding for different block types<br/>- Example: `./compiler ir --input program.src --format dot --output cfg.dot` | |
| OUT-3 | **JSON Output** should support: | Should |
| | - Machine-readable IR representation<br/>- Complete program structure<br/>- Type annotations and metadata<br/>- Integration with analysis tools | |
| OUT-4 | **Statistics** should report: | Should |
| | - Number of instructions by type<br/>- Number of basic blocks<br/>- Number of temporaries used<br/>- Maximum stack depth estimate | |

## **5. Testing & Verification**

Comprehensive tests must validate IR generation correctness and structural properties.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| TEST-1 | **Unit Tests** must cover: | Must |
| | - Expression translation to 3-address code<br/>- Control flow translation (if, while)<br/>- Function call translation<br/>- Variable access and assignment<br/>- Basic block construction and linking | |
| TEST-2 | **Test Categories** must include: | Must |
| | ```<br/>tests/ir/generation/<br/>├── expressions/<br/>├── control_flow/<br/>├── functions/<br/>└── integration/<br/><br/>tests/ir/validation/<br/>├── structural_checks/<br/>├── type_consistency/<br/>└── optimization_ready/<br/>``` | |
| TEST-3 | **Golden Testing** must compare: | Must |
| | - Expected vs. actual IR for given source programs<br/>- Structural properties (block count, instruction count)<br/>- Data flow properties (def-use chains where applicable) | |
| TEST-4 | **Validation Checks** must verify: | Should |
| | - Every basic block ends with control flow instruction<br/>- All jumps target valid labels<br/>- No undefined temporaries are used<br/>- Type consistency in operations<br/>- Dominator tree properties (if implemented) | |
| TEST-5 | **Integration Tests** must: | Should |
| | - Test full pipeline: source → AST → IR<br/>- Verify IR preserves semantics of original program<br/>- Test round-trip: IR → text → parse → compare (if IR parser implemented) | |

## **6. Stretch Goal: Basic Peephole Optimization**

Simple local optimizations may be implemented on the IR to improve code quality.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| OPT-1 | **Peephole Optimizer** may implement: | Optional |
| | - **Algebraic simplifications:** `x + 0 → x`, `x * 1 → x`, `x * 0 → 0`<br/>- **Constant folding:** `3 + 4 → 7` at compile time<br/>- **Strength reduction:** `x * 2 → x + x`, `x / 2 → x >> 1`<br/>- **Dead code elimination:** Remove unused assignments<br/>- **Jump chaining:** `JUMP L1; L1: JUMP L2 → JUMP L2` | |
| OPT-2 | **Optimization Interface** should provide: | Optional |
| | - `PeepholeOptimizer(ir_program)` constructor<br/>- `optimize()`: apply all optimizations<br/>- `get_optimization_report()`: list of changes made<br/>- Configurable optimization levels | |
| OPT-3 | **Optimization Safety** must ensure: | Optional |
| | - Optimizations preserve program semantics<br/>- No optimization introduces errors<br/>- Optimizations can be disabled for debugging<br/>- Each optimization is independently testable | |
| OPT-4 | **Metrics** should track: | Optional |
| | - Number of instructions removed/added<br/>- Reduction in temporary count<br/>- Improvement in estimated execution cost<br/>- Optimization iteration count | |

**Example IR Instruction Set:**
```
# Arithmetic Operations
ADD  t1, t2, t3       # t1 = t2 + t3
SUB  t1, t2, t3       # t1 = t2 - t3
MUL  t1, t2, t3       # t1 = t2 * t3
DIV  t1, t2, t3       # t1 = t2 / t3
MOD  t1, t2, t3       # t1 = t2 % t3
NEG  t1, t2           # t1 = -t2

# Logical Operations
AND  t1, t2, t3       # t1 = t2 && t3
OR   t1, t2, t3       # t1 = t2 || t3
NOT  t1, t2           # t1 = !t2
XOR  t1, t2, t3       # t1 = t2 ^ t3

# Comparisons
CMP_EQ   t1, t2, t3   # t1 = (t2 == t3)
CMP_NE   t1, t2, t3   # t1 = (t2 != t3)
CMP_LT   t1, t2, t3   # t1 = (t2 < t3)
CMP_LE   t1, t2, t3   # t1 = (t2 <= t3)
CMP_GT   t1, t2, t3   # t1 = (t2 > t3)
CMP_GE   t1, t2, t3   # t1 = (t2 >= t3)

# Memory Operations
LOAD     t1, [t2]     # t1 = *t2
STORE    [t1], t2     # *t1 = t2
ALLOCA   t1, size     # Allocate local memory
GEP      t1, t2, idx  # t1 = &t2[idx] (get element pointer)

# Control Flow
JUMP        L1         # Unconditional jump to L1
JUMP_IF     t1, L1     # Jump to L1 if t1 != 0
JUMP_IF_NOT t1, L1     # Jump to L1 if t1 == 0
LABEL       L1         # Mark location L1
PHI         t1, (v1, B1), (v2, B2)  # t1 = v1 if coming from B1, v2 if from B2

# Function Operations
CALL    t1, func, args...  # t1 = func(args...)
RETURN  t1                 # Return t1 from function
PARAM   idx, value         # Set parameter for call
```

**Example Source → IR Transformation:**
```c
// Source: factorial function
int factorial(int n) {
    if (n <= 1) {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}

// Generated IR:
function factorial: int (int n_0)
  entry:
    t1 = CMP_LE n_0, 1
    JUMP_IF t1, L_true
    JUMP L_false

  L_true:
    RETURN 1

  L_false:
    t2 = SUB n_0, 1
    PARAM 0, t2
    t3 = CALL factorial, 1
    t4 = MUL n_0, t3
    RETURN t4

  exit:
    # Implicit return for void functions
```

**Example Usage:**
```bash
# Generate IR from source
$ ./compiler ir --input examples/factorial.src --output factorial.ir

# Generate IR with optimization
$ ./compiler ir --input examples/factorial.src --optimize --output factorial_opt.ir

# Generate control flow graph visualization
$ ./compiler ir --input examples/factorial.src --format dot --output factorial_cfg.dot
$ dot -Tpng factorial_cfg.dot -o factorial_cfg.png

# Display IR statistics
$ ./compiler ir --input examples/factorial.src --stats

# Run IR tests
$ ./compiler test --ir
# or
$ make test-ir
# or
$ cargo test ir
# or
$ python -m pytest tests/ir/
```

**Example IR Output (Text Format):**
```
# Program: test.src
# Generated IR: 2024-01-15 10:30:00

.global main

function main: void ()
  # Basic block: entry
  entry:
    # int x = 2 * 3 + 4;
    t1 = MUL 2, 3        # 2 * 3
    t2 = ADD t1, 4       # + 4
    x_1 = MOVE t2        # x = result

    # if (x > 5) {
    t4 = CMP_GT x_1, 5
    JUMP_IF t4, L_then
    JUMP L_else

  # Basic block: then
  L_then:
    # y = 10;
    y_1 = MOVE 10
    JUMP L_endif

  # Basic block: else
  L_else:
    # y = 20;
    y_2 = MOVE 20
    JUMP L_endif

  # Basic block: endif (join point)
  L_endif:
    y_3 = PHI (y_1, L_then), (y_2, L_else)  # y value
    # return y;
    RETURN y_3

# Symbol table mapping:
# x_1 -> local variable x (SSA version 1)
# y_1 -> local variable y (SSA version 1)
# y_2 -> local variable y (SSA version 2)
# y_3 -> local variable y (SSA phi resolution)
```

**Optimization Example:**
```
# Before optimization:
  t1 = ADD x_1, 0
  t2 = MUL t1, 1
  t3 = CMP_GT t2, 5
  JUMP_IF t3, L1
  JUMP L2

# After peephole optimization:
  t1 = CMP_GT x_1, 5    # x_1 + 0 → x_1, x_1 * 1 → x_1
  JUMP_IF t1, L1
  JUMP L2
```
