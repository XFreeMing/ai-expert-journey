#!/usr/bin/env bash
# Environment setup script for AI Expert Journey
# Usage: source scripts/setup.sh
set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

echo "Setting up AI Expert Journey environment..."

# Create virtual environment
cd "$PROJECT_ROOT"
if [ ! -d ".venv" ]; then
    echo "Creating virtual environment..."
    python3 -m venv .venv
fi

source .venv/bin/activate

# Install dependencies
echo "Installing dependencies..."
pip install -e ".[dev]" --quiet

# Copy .env.example if .env doesn't exist
if [ ! -f ".env" ]; then
    echo "Creating .env from .env.example..."
    cp .env.example .env
    echo "WARNING: Please edit .env with your actual API keys"
fi

# Create data directories
mkdir -p data/{raw,processed,models}

# Install pre-commit hooks
if command -v pre-commit &> /dev/null; then
    echo "Installing pre-commit hooks..."
    pre-commit install
fi

echo "Setup complete! Activate with: source .venv/bin/activate"
