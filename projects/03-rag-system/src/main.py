"""RAG System API entry point."""

from shared.config import ProjectConfig


def main():
    config = ProjectConfig()
    print("RAG Knowledge System v0.1.0")
    print("[TODO] RAG pipeline implementation pending — see docs/adr/003-rag-system.md")


if __name__ == "__main__":
    main()
