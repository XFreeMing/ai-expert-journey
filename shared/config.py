"""Project configuration and environment management."""

import os
from dataclasses import dataclass, field
from pathlib import Path


@dataclass
class ProjectConfig:
    """Shared project configuration."""
    project_root: Path = field(default_factory=lambda: Path(__file__).parent.parent.parent)
    data_dir: Path = field(default_factory=lambda: Path(__file__).parent.parent.parent / "data")
    model_cache_dir: Path = field(default_factory=lambda: Path.home() / ".cache" / "ai-expert-journey")

    # Common defaults
    default_embedding_model: str = "sentence-transformers/all-MiniLM-L6-v2"
    default_llm_provider: str = "openai"
    default_max_tokens: int = 4096
    default_temperature: float = 0.7

    def __post_init__(self):
        self.data_dir.mkdir(parents=True, exist_ok=True)
        self.model_cache_dir.mkdir(parents=True, exist_ok=True)

    def get_env(self, key: str, default: str = "") -> str:
        return os.getenv(key, default)

    @property
    def openai_api_key(self) -> str:
        return self.get_env("OPENAI_API_KEY", "")

    @property
    def hf_token(self) -> str:
        return self.get_env("HF_TOKEN", "")
