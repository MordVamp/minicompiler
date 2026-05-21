# **Project: MiniCompiler - Technical Requirements Document (Sprint 8)**

**Sprint Goal:** Finalize the compiler with a polished interface, comprehensive error handling, thorough testing, and complete documentation for the final demonstration.

## **1. Command-Line Interface & User Experience**

A professional, user-friendly CLI must provide intuitive access to all compiler features.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| CLI-1 | **Command Structure** must follow standard Unix conventions: | Must |
| | `./mycc [options] <sourcefile>`<br/>`./mycc [options] <command> [subcommand]` | |
| CLI-2 | **Essential Options** must include: | Must |
| | - `-o <file>`: Specify output file<br/>- `-S`: Generate assembly only (don't assemble/link)<br/>- `-c`: Compile to object file<br/>- `-E`: Preprocess only<br/>- `--ast`: Output AST in specified format<br/>- `--ir`: Output intermediate representation<br/>- `--optimize` or `-O[0-3]`: Optimization levels<br/>- `--target <arch>`: Target architecture (default: x86_64)<br/>- `--verbose` or `-v`: Verbose output<br/>- `--help` or `-h`: Display help message<br/>- `--version`: Display compiler version | |
| CLI-3 | **Compilation Modes** must support: | Must |
| | - **Full compilation:** `mycc program.src -o program` (default)<br/>- **Assembly output:** `mycc -S program.src -o program.asm`<br/>- **Object file:** `mycc -c program.src -o program.o`<br/>- **Multiple files:** `mycc file1.src file2.src -o program`<br/>- **Library linking:** `mycc program.src -lm -o program` | |
| CLI-4 | **Configuration Support** should include: | Should |
| | - Configuration file (JSON/YAML) for common options<br/>- Environment variable overrides (e.g., `MYCC_OPTIONS`)<br/>- Compiler flags stored per project<br/>- Default optimization settings | |
| CLI-5 | **User Experience** must provide: | Must |
| | - Clear, colorized output (with `--color=auto` option)<br/>- Progress indicators for long operations<br/>- Tab completion for shells (optional)<br/>- Man page installation with `make install` | |

## **2. Robust Error Handling System**

A unified error reporting system must provide clear, actionable error messages across all compilation stages.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| ERR-1 | **Error Categories** must be clearly defined: | Must |
| | - **Lexical errors:** Invalid characters, malformed tokens<br/>- **Syntax errors:** Grammar violations, missing tokens<br/>- **Semantic errors:** Type mismatches, undefined symbols<br/>- **Code generation errors:** Register spill failures, unsupported features<br/>- **Linker errors:** Missing symbols, library issues<br/>- **Runtime errors:** Division by zero, null dereference (if checked) | |
| ERR-2 | **Error Message Format** must include: | Must |
| | - Error type and code (e.g., `E042: undefined variable`)<br/>- Source file, line, and column (1-indexed)<br/>- Code snippet with caret pointing to error location<br/>- Context information (function, scope)<br/>- Suggested fix when possible<br/>- Related note lines for additional context | |
| ERR-3 | **Error Recovery & Multiple Errors** must: | Must |
| | - Continue compilation after errors to report multiple issues<br/>- Maintain maximum error count before aborting (configurable)<br/>- Prevent cascading errors from single root cause<br/>- Provide error summary at end of compilation | |
| ERR-4 | **Warning System** must: | Must |
| | - Support different warning levels (`-Wall`, `-Werror`, `-Wno-...`)<br/>- Provide actionable warnings (unused variables, implicit conversions)<br/>- Allow warnings as errors option for strict compilation<br/>- Generate warning suppression comments in source | |
| ERR-5 | **Error Output Formats** must support: | Should |
| | - Human-readable (default, colorized)<br/>- JSON format for tool integration (`--format=json`)<br/>- IDE/editor compatible format (similar to gcc/clang)<br/>- Statistics on errors/warnings | |

## **3. Comprehensive Testing Suite**

A complete test suite must validate all compiler features and ensure robustness.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| TEST-1 | **Test Categories** must include: | Must |
| | ```<br/>tests/final/<br/>├── good/           # Programs that should compile and run correctly<br/>│   ├── arithmetic/<br/>│   ├── control_flow/<br/>│   ├── functions/<br/>│   ├── arrays/<br/>│   ├── structures/<br/>│   └── integration/<br/>├── bad/            # Programs that should produce specific errors<br/>│   ├── syntax_errors/<br/>│   ├── type_errors/<br/>│   ├── semantic_errors/<br/>│   └── runtime_errors/<br/>├── benchmarks/     # Performance tests<br/>└── demo/          # Final demonstration programs<br/>``` | |
| TEST-2 | **Test Execution Framework** must: | Must |
| | - Run all tests with single command (`make test-all`)<br/>- Support parallel test execution<br/>- Generate detailed test reports<br/>- Support test filtering by category or name<br/>- Provide pass/fail summary with statistics | |
| TEST-3 | **Golden Testing** must verify: | Must |
| | - Program output matches expected results<br/>- Compiler error messages match expected patterns<br/>- Generated assembly/IR matches expected patterns (optional)<br/>- Performance within acceptable bounds | |
| TEST-4 | **Regression Tests** must: | Should |
| | - Include previously fixed bugs as test cases<br/>- Test edge cases from all previous sprints<br/>- Ensure backward compatibility for language features<br/>- Test compiler self-hosting capability (optional) | |
| TEST-5 | **Integration Tests** must: | Must |
| | - Test full compilation pipeline end-to-end<br/>- Test interoperability with external tools (assembler, linker)<br/>- Test cross-module compilation (multiple source files)<br/>- Test library linking and external calls | |

## **4. Complete Documentation**

Comprehensive documentation must enable users to understand, use, and extend the compiler.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| DOC-1 | **README.md** must include: | Must |
| | - Project overview and goals<br/>- Quick start guide with examples<br/>- Complete build instructions for all platforms<br/>- Language specification summary<br/>- Command-line reference with examples<br/>- Testing instructions<br/>- Contributing guidelines<br/>- License information | |
| DOC-2 | **Language Specification Document** must detail: | Must |
| | - Complete formal grammar (lexical and syntactic)<br/>- Type system and semantics<br/>- Standard library functions<br/>- Memory model and ownership rules<br/>- Differences from similar languages (C, Java, etc.)<br/>- Implementation limits (max identifiers, recursion depth, etc.) | |
| DOC-3 | **Developer Documentation** must include: | Must |
| | - Architecture overview and module descriptions<br/>- API documentation for all public interfaces<br/>- Adding new language features guide<br/>- Debugging and profiling the compiler<br/>- Extension points for optimizations and backends | |
| DOC-4 | **User Guide** should provide: | Should |
| | - Tutorial with progressively complex examples<br/>- Common patterns and idioms<br/>- Interoperability with C and other languages<br/>- Performance tuning guide<br/>- Troubleshooting common issues | |
| DOC-5 | **Generated Documentation** should include: | Should |
| | - Auto-generated API docs (Doxygen, Rustdoc, etc.)<br/>- Man pages for command-line tools<br/>- Example code with executable snippets<br/>- Change log and version history | |

## **5. Final Demonstration Program**

A showcase program must demonstrate all key language features and compiler capabilities.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| DEMO-1 | **Program Requirements** must demonstrate: | Must |
| | - All major language features (variables, functions, control flow, arrays, structs)<br/>- Recursive algorithms<br/>- File I/O or system interaction<br/>- Use of standard library functions<br/>- Error handling and robustness<br/>- Non-trivial algorithm with measurable output | |
| DEMO-2 | **Example Programs** could be: | Must |
| | - **Mini interpreter or calculator**<br/>- **Simple games** (tic-tac-toe, snake, maze solver)<br/>- **Data processing** (CSV parser, JSON validator)<br/>- **Algorithm showcase** (multiple sorting algorithms with timing)<br/>- **Math library implementation** (matrix operations, numerical methods) | |
| DEMO-3 | **Demo Script** must include: | Must |
| | - Compilation command with all options<br/>- Execution command and expected output<br/>- Verification steps to confirm correctness<br/>- Performance measurements if applicable<br/>- Cleanup instructions | |
| DEMO-4 | **Presentation Materials** should include: | Should |
| | - Slides or markdown presentation<br/>- Live demonstration script<br/>- Architecture diagrams<br/>- Performance comparison charts<br/>- Code walkthrough of key features | |

## **6. Build & Distribution System**

The compiler must be easily buildable and distributable across different environments.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| BUILD-1 | **Build System** must support: | Must |
| | - Single-command build: `make` or `cargo build` or `python setup.py build`<br/>- Installation: `make install` (to `/usr/local/bin` or user directory)<br/>- Clean targets: `make clean`, `make distclean`<br/>- Cross-compilation support (optional)<br/>- Dependency management and fetching | |
| BUILD-2 | **Platform Support** must include: | Must |
| | - Linux (primary target)<br/>- macOS (with compatible toolchain)<br/>- Windows (via WSL2 or MinGW)<br/>- Common architectures: x86_64, arm64 (optional) | |
| BUILD-3 | **Distribution Packages** may include: | Optional |
| | - Debian/Ubuntu `.deb` package<br/>- Red Hat `.rpm` package<br/>- Arch Linux PKGBUILD<br/>- Docker container with compiler<br/>- Binary releases for common platforms | |

## **7. Stretch Goal: Performance Benchmarking**

Optional implementation of benchmarking to compare compiler output against established compilers.

| ID | Requirement Description | Priority |
| :-- | :------------------------------------------------------------------------------------------------------------------ | :------- |
| BENCH-1 | **Benchmark Suite** may include: | Optional |
| | - **Microbenchmarks:** Arithmetic operations, function calls, loops<br/>- **Macrobenchmarks:** Complete algorithms (sorting, searching)<br/>- **Real-world examples:** Small but complete programs<br/>- **Comparison targets:** Equivalent C programs compiled with gcc/clang | |
| BENCH-2 | **Metrics Collection** should measure: | Optional |
| | - Compilation time (frontend, backend, total)<br/>- Generated code size (.text, .data, .bss)<br/>- Execution time (average, minimum, maximum)<br/>- Memory usage (peak heap, stack depth)<br/>- Optimization effectiveness | |
| BENCH-3 | **Comparison Methodology** must ensure: | Optional |
| | - Fair comparison (same algorithm, same optimizations)<br/>- Statistical significance (multiple runs, confidence intervals)<br/>- Controlled environment (CPU governor, background processes)<br/>- Clear reporting of compiler versions and flags | |
| BENCH-4 | **Performance Analysis Tools** may include: | Optional |
| | - Assembly diff viewing<br/>- Execution profile generation<br/>- Cache simulation estimates<br/>- Bottleneck identification guidance | |

**Example CLI Usage:**
```bash
# Basic compilation
$ ./mycc program.src -o program
$ ./program

# Generate assembly only
$ ./mycc -S program.src -o program.asm
$ cat program.asm

# Compile with optimizations and debugging info
$ ./mycc -O2 -g program.src -o program_debug

# Show AST and IR for debugging
$ ./mycc --ast --ir program.src

# Compile multiple files
$ ./mycc main.src utils.src math.src -o app

# Compile with warnings and treat as errors
$ ./mycc -Wall -Werror program.src -o program

# Show help
$ ./mycc --help
Usage: mycc [options] file...
Options:
  -o <file>     Place output into <file>
  -S            Generate assembly only
  -c            Compile to object file (don't link)
  -O<level>     Set optimization level (0-3)
  -g            Generate debugging information
  -I<dir>       Add include directory
  -L<dir>       Add library directory
  -l<lib>       Link with library
  --ast         Output abstract syntax tree
  --ir          Output intermediate representation
  --target <arch> Set target architecture
  --verbose     Show detailed compilation steps
  --version     Display compiler version
  --help        Display this help message

# Show version
$ ./mycc --version
mycc 1.0.0
Compiler for MiniLang
Built: 2024-01-15
Target: x86_64-linux-gnu
```

**Error Reporting Example:**
```bash
$ ./mycc broken.src
broken.src:12:5: error: E042: undefined variable 'undefined_var'
   10 | int compute() {
   11 |     int x = 10;
   12 |     return x + undefined_var;
      |                ^^^^^^^^^^^^^
   13 | }
   14 |
note: did you mean 'defined_var'? (declared at line 5)

broken.src:18:1: warning: W201: unused variable 'y'
   16 | void foo() {
   17 |     int x = 5;
   18 |     int y = 10;  // Never used
      |     ^
   19 |     printf("%d\n", x);
   20 | }

2 errors, 1 warning generated.
Compilation failed.
```

**Final Demo Program Example:**
```c
// demo/raytracer.src - Simple raytracer demonstrating multiple features
extern void printf(char* format, ...);
extern double sqrt(double x);
extern double sin(double x);
extern double cos(double x);

struct Vec3 {
    double x, y, z;
};

struct Sphere {
    struct Vec3 center;
    double radius;
    int material;
};

struct Ray {
    struct Vec3 origin;
    struct Vec3 direction;
};

// Vector operations
struct Vec3 vec3_add(struct Vec3 a, struct Vec3 b) {
    struct Vec3 result = {a.x + b.x, a.y + b.y, a.z + b.z};
    return result;
}

struct Vec3 vec3_sub(struct Vec3 a, struct Vec3 b) {
    struct Vec3 result = {a.x - b.x, a.y - b.y, a.z - b.z};
    return result;
}

double vec3_dot(struct Vec3 a, struct Vec3 b) {
    return a.x * b.x + a.y * b.y + a.z * b.z;
}

double vec3_length(struct Vec3 v) {
    return sqrt(v.x * v.x + v.y * v.y + v.z * v.z);
}

// Ray-sphere intersection
int ray_intersects_sphere(struct Ray ray, struct Sphere sphere, double* t) {
    struct Vec3 oc = vec3_sub(ray.origin, sphere.center);
    double a = vec3_dot(ray.direction, ray.direction);
    double b = 2.0 * vec3_dot(oc, ray.direction);
    double c = vec3_dot(oc, oc) - sphere.radius * sphere.radius;
    double discriminant = b * b - 4 * a * c;

    if (discriminant < 0) {
        return 0;
    } else {
        *t = (-b - sqrt(discriminant)) / (2.0 * a);
        return 1;
    }
}

// Simple raytracer main function
int main() {
    struct Sphere spheres[3];
    spheres[0] = (struct Sphere){{0, 0, -5}, 1, 0};
    spheres[1] = (struct Sphere){{-2, 0, -7}, 1.5, 1};
    spheres[2] = (struct Sphere){{2, 1, -6}, 0.8, 2};

    int width = 40;
    int height = 20;

    printf("Simple Raytracer Output:\n");
    printf("=======================\n");

    for (int y = 0; y < height; y = y + 1) {
        for (int x = 0; x < width; x = x + 1) {
            struct Ray ray;
            ray.origin = (struct Vec3){0, 0, 0};

            double nx = (x - width / 2.0) / width;
            double ny = (height / 2.0 - y) / height;
            ray.direction = (struct Vec3){nx, ny, -1};

            int hit = 0;
            double closest_t = 1000000;
            int material = 0;

            for (int i = 0; i < 3; i = i + 1) {
                double t;
                if (ray_intersects_sphere(ray, spheres[i], &t)) {
                    if (t > 0 && t < closest_t) {
                        closest_t = t;
                        hit = 1;
                        material = spheres[i].material;
                    }
                }
            }

            if (hit) {
                char pixel = ' ';
                if (material == 0) pixel = '#';
                else if (material == 1) pixel = '*';
                else pixel = '.';
                printf("%c", pixel);
            } else {
                printf(" ");
            }
        }
        printf("\n");
    }

    return 0;
}
```

**Example Test Execution:**
```bash
# Run complete test suite
$ make test-all
Running lexer tests... ✓ 42/42 passed
Running parser tests... ✓ 38/38 passed
Running semantic tests... ✓ 35/35 passed
Running codegen tests... ✓ 28/28 passed
Running optimization tests... ✓ 15/15 passed
Running integration tests... ✓ 12/12 passed

All tests passed! 170/170 tests successful.

# Run specific test category
$ ./mycc test --category codegen

# Run benchmark suite
$ ./mycc benchmark --suite micro
Benchmark Results:
  fibonacci(20):
    mycc:   1.23ms ± 0.05ms
    gcc -O0: 1.45ms ± 0.06ms
    Speedup: 1.18x

  matrix_mult(64x64):
    mycc:   12.5ms ± 0.3ms
    gcc -O0: 10.2ms ± 0.2ms
    Speedup: 0.82x

# Generate test coverage report
$ make coverage
Generating coverage report...
Coverage: 87% lines, 92% functions
```

**Benchmark Comparison Example (Stretch Goal):**
```c
// benchmark/fibonacci.src vs fibonacci.c
int fib(int n) {
    if (n <= 1) {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}

int main() {
    return fib(20);  // Should return 6765
}
```

```bash
# Compile with mycc
$ ./mycc -O2 benchmark/fibonacci.src -o fib_mycc
$ time ./fib_mycc

# Compile with gcc for comparison
$ gcc -O2 benchmark/fibonacci.c -o fib_gcc
$ time ./fib_gcc

# Generate comparison report
$ ./mycc benchmark --compare fib_mycc fib_gcc
Comparison Report:
  Program: fibonacci(20)
  --------------------------
  mycc:
    Compile time: 0.45s
    Executable size: 8.2KB
    Execution time: 1.23ms

  gcc -O2:
    Compile time: 0.12s
    Executable size: 16.7KB
    Execution time: 0.89ms

  Analysis:
    mycc executable is 51% smaller
    gcc is 38% faster
    mycc compile time is 3.75x slower
```
