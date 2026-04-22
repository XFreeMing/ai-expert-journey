"""Main entry point for code review CLI tool."""

import argparse
import json
import sys
from pathlib import Path

from shared.config import ProjectConfig


def main():
    parser = argparse.ArgumentParser(description="Intelligent Code Review Assistant")
    parser.add_argument("path", type=Path, help="Path to code file or directory")
    parser.add_argument("--lang", choices=["python", "java"], default="python")
    parser.add_argument("--output", choices=["json", "markdown"], default="markdown")
    parser.add_argument("--config", type=Path, help="Custom rules config file")
    args = parser.parse_args()

    config = ProjectConfig()
    print(f"Code Review Assistant v0.1.0")
    print(f"Scanning: {args.path}")
    print(f"Language: {args.lang}")
    print(f"\n[TODO] Implementation pending — see docs/adr/001-code-reviewer.md")


if __name__ == "__main__":
    main()
