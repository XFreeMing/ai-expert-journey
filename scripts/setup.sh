#!/usr/bin/env bash
# Environment setup script for AI Expert Journey
# Usage: source scripts/setup.sh
set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

echo "Setting up AI Expert Journey environment..."

# Install uv if not present
cd "$PROJECT_ROOT"
if ! command -v uv &> /dev/null; then
    echo "Installing uv..."
    pip install uv
fi

# Create virtual environment and sync dependencies
echo "Creating virtual environment and syncing dependencies..."
uv venv --python 3.14 --allow-existing
uv sync

# Copy .env.example if .env doesn't exist
if [ ! -f ".env" ]; then
    echo "Creating .env from .env.example..."
    cp .env.example .env
    echo "WARNING: Please edit .env with your actual API keys"
fi

# Create data directories
mkdir -p data/{raw,processed,models}

# Install pre-commit hooks (if available in environment)
if command -v pre-commit &> /dev/null; then
    echo "Installing pre-commit hooks..."
    pre-commit install
fi

echo "Setup complete! Activate with: source .venv/bin/activate"
