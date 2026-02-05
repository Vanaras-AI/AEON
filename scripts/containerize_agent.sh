#!/bin/bash
# AEON Containerizer Utility 📦
# Converts raw Python scripts into Safe WASM Components.

set -e

SOURCE_PY=$1
OUTPUT_WASM=$2

# 1. Validation
if [ -z "$SOURCE_PY" ] || [ -z "$OUTPUT_WASM" ]; then
    echo "Usage: ./containerize_agent.sh <source.py> <output.wasm>"
    exit 1
fi

if [ ! -f "$SOURCE_PY" ]; then
    echo "❌ Error: Source file '$SOURCE_PY' not found."
    exit 1
fi

echo "🛡️  AEON Secure Containerizer"
echo "============================"
echo "🔹 Input:  $SOURCE_PY"
echo "🔹 Output: $OUTPUT_WASM"

# 2. Check for componentize-py
if ! command -v componentize-py &> /dev/null; then
    echo "⚠️  componentize-py not found."
    echo "🛠️  Installing via pip (requires active venv)..."
    python3 -m pip install componentize-py
fi

# 3. Compile to WASM Component
echo "⚙️  Compiling to WASM Component (WASI Preview 2)..."

# Note: In a real scenario, we would link against 'agent.wit' here.
# For this sprint, we treat the python script as the main entry point.
componentize-py \
    -d mandates/agent.wit \
    -w agent-v1 \
    componentize \
    "$SOURCE_PY" \
    -o "$OUTPUT_WASM"

echo "✅ Success! Safe Agent created at: $OUTPUT_WASM"
echo "📜 Receipt: $(date -u)"
