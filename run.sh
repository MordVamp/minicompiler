#!/usr/bin/env bash
# run.sh — compile a .src file to native x86-64 and execute it
#
# Usage:
#   ./run.sh <source.src>           # compile & run (binary auto-named from source)
#   ./run.sh <source.src> mybin     # compile & run, binary named 'mybin'
#   ./run.sh <source.src> - --keep  # keep intermediate .asm and .o files
#
# Examples:
#   ./run.sh examples/sprint7/test_extern.src
#   ./run.sh examples/sprint6/sprint6_control.src out --keep
#   ./run.sh examples/quick_sort.src - --time
#   ./run.sh examples/quick_sort.src - --perf

set -euo pipefail

# ── helpers ──────────────────────────────────────────────────────────────────
die() { echo "❌ $*" >&2; exit 1; }
need() { command -v "$1" &>/dev/null || die "Required tool not found: $1 (install with: $2)"; }

# ── argument parsing ──────────────────────────────────────────────────────────
SRC="${1:-}"
BIN_NAME="${2:-}"
KEEP=0
USE_TIME=0
USE_PERF=0

for arg in "$@"; do
    [[ "$arg" == "--keep" ]] && KEEP=1
    [[ "$arg" == "--time" ]] && USE_TIME=1
    [[ "$arg" == "--perf" ]] && USE_PERF=1
done

[[ -z "$SRC" ]] && { echo "Usage: $0 <source.src> [binary_name] [--keep] [--time] [--perf]"; exit 1; }
[[ -f "$SRC" ]] || die "Source file not found: $SRC"

# ── derive file names ─────────────────────────────────────────────────────────
BASE="$(basename "${SRC%.src}")"
DIR="$(dirname "$SRC")"
ASM_FILE="${DIR}/${BASE}.asm"
OBJ_FILE="${DIR}/${BASE}.o"
[[ -z "$BIN_NAME" || "$BIN_NAME" == "--keep" ]] && BIN_NAME="${DIR}/${BASE}"

# ── check dependencies ────────────────────────────────────────────────────────
need nasm  "sudo apt install nasm"
need gcc   "sudo apt install gcc"

# ── step 1: compile .src → .asm ──────────────────────────────────────────────
echo "🔨 [1/4] Compiling  ${SRC}  →  ${ASM_FILE}"
if [[ -f "./minicompiler.exe" ]]; then
    ./minicompiler.exe compile -i "$SRC" -o "$ASM_FILE"
elif [[ -f "./minicompiler" ]]; then
    ./minicompiler compile -i "$SRC" -o "$ASM_FILE"
else
    cargo run --quiet --bin minicompiler -- compile -i "$SRC" -o "$ASM_FILE"
fi

# ── step 2: assemble .asm → .o ───────────────────────────────────────────────
echo "🔩 [2/4] Assembling ${ASM_FILE}  →  ${OBJ_FILE}"
nasm -f elf64 "$ASM_FILE" -o "$OBJ_FILE"

# ── step 3: link .o → native binary ──────────────────────────────────────────
# -nostartfiles : skip CRT's _start — our ASM provides its own _start
# -no-pie       : absolute addressing (no position-independent overhead)
# -lc           : still link libc so printf/malloc/free are available
echo "🔗 [3/4] Linking    ${OBJ_FILE}  →  ${BIN_NAME}"
gcc "$OBJ_FILE" -o "$BIN_NAME" -no-pie -nostartfiles -lc

# ── step 4: run ───────────────────────────────────────────────────────────────
echo "🚀 [4/4] Running    ${BIN_NAME}"
echo "─────────────────────────────────────"

if [[ "$USE_PERF" -eq 1 ]]; then
    need perf "sudo apt-get install linux-tools-common linux-tools-generic"
    perf stat "$BIN_NAME"
    EXIT_CODE=$?
elif [[ "$USE_TIME" -eq 1 ]]; then
    time "$BIN_NAME"
    EXIT_CODE=$?
else
    "$BIN_NAME"
    EXIT_CODE=$?
fi
echo "─────────────────────────────────────"
echo "✅ Exit code: ${EXIT_CODE}"

# ── cleanup ───────────────────────────────────────────────────────────────────
if [[ "$KEEP" -eq 0 ]]; then
    rm -f "$OBJ_FILE"
else
    echo "📁 Kept: ${ASM_FILE}  ${OBJ_FILE}"
fi
