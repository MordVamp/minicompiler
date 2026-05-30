#!/usr/bin/env bash
# Differential testing against GCC
# Usage: ./scripts/differential_test.sh <file.src>

set -euo pipefail

SRC=$1
BASE=$(basename "$SRC" .src)
DIR=$(dirname "$SRC")

C_FILE="/tmp/${BASE}_diff.c"
GCC_BIN="/tmp/${BASE}_gcc"
MINI_BIN="/tmp/${BASE}_mini"

# 1. Translate our language syntax to standard C syntax using sed
#    fn main() -> void {  ===>  void main() {
#    fn name(int a) -> int { ===> int name(int a) {
cat "$SRC" | sed -E 's/fn ([a-zA-Z0-9_]+)[ \t]*\((.*)\)[ \t]*->[ \t]*([a-zA-Z0-9_]+)/\3 \1(\2)/' > "$C_FILE"

# Make sure standard headers are included for C
sed -i '1i#include <stdio.h>\n#include <stdlib.h>' "$C_FILE"

# 2. Compile with GCC
gcc "$C_FILE" -o "$GCC_BIN"

# 3. Compile with minicompiler
./run.sh "$SRC" "$MINI_BIN"

# 4. Run both and capture stdout and exit codes
echo "[GCC] Running..."
GCC_OUT=$("$GCC_BIN" 2>&1 || true)
GCC_CODE=$?

echo "[MINICOMPILER] Running..."
MINI_OUT=$("$MINI_BIN" 2>&1 || true)
MINI_CODE=$?

# 5. Compare
echo "========================================"
if [ "$GCC_CODE" -eq "$MINI_CODE" ] && [ "$GCC_OUT" == "$MINI_OUT" ]; then
    echo "✅ DIFFERENTIAL TEST PASSED: $SRC"
    echo "Exit Code: $MINI_CODE"
    exit 0
else
    echo "❌ DIFFERENTIAL TEST FAILED: $SRC"
    echo "--- GCC Output (Code: $GCC_CODE) ---"
    echo "$GCC_OUT"
    echo "--- MINICOMPILER Output (Code: $MINI_CODE) ---"
    echo "$MINI_OUT"
    exit 1
fi
