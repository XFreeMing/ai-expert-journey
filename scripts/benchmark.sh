#!/usr/bin/env bash
# Run benchmarks for a specific project
# Usage: scripts/benchmark.sh <project-name>
set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
PROJECT_NAME="${1:?Usage: $0 <project-name>}"
PROJECT_DIR="$PROJECT_ROOT/projects/$PROJECT_NAME"

if [ ! -d "$PROJECT_DIR" ]; then
    echo "Error: Project '$PROJECT_NAME' not found in projects/"
    echo "Available projects:"
    ls "$PROJECT_ROOT/projects/" | sed 's/^/  - /'
    exit 1
fi

cd "$PROJECT_DIR"

echo "Running benchmarks for $PROJECT_NAME..."

# Activate venv if exists
if [ -f "$PROJECT_ROOT/.venv/bin/activate" ]; then
    source "$PROJECT_ROOT/.venv/bin/activate"
fi

# Install project deps via uv workspace
if [ -f "$PROJECT_ROOT/pyproject.toml" ]; then
    cd "$PROJECT_ROOT" && uv sync --quiet
    cd "$PROJECT_DIR"
fi

# Run benchmark if it exists
if [ -f "benchmark.py" ]; then
    python benchmark.py
elif [ -f "tests/test_benchmark.py" ]; then
    pytest tests/test_benchmark.py -v --tb=short
else
    echo "No benchmark found. Create benchmark.py or tests/test_benchmark.py"
    exit 1
fi
