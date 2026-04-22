"""Main entry point for vector engine."""

from shared.config import ProjectConfig


def main():
    config = ProjectConfig()
    print("Vector Retrieval Engine v0.1.0")
    print("[TODO] HNSW implementation pending — see docs/adr/002-vector-engine.md")


if __name__ == "__main__":
    main()
