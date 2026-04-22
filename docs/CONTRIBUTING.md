# Contributing Guide

## Development Environment Setup

### Prerequisites

| Requirement | Version | Purpose |
|-------------|---------|---------|
| Python | >= 3.11 | Runtime |
| Docker + Compose | v24+ / v2.20+ | Service orchestration |
| Git | 2.30+ | Version control |
| (Optional) NVIDIA GPU | RTX 3060+ (12GB VRAM) | GPU-accelerated training/inference |
| (Optional) Apple Silicon | M1/M2/M3/M4 (16GB+) | MLX/GGUF local inference |

### Installation

```bash
# 1. Clone and setup
git clone <repo-url>
cd ai-expert-journey

# 2. Install dependencies (creates venv, installs packages, copies .env)
source scripts/setup.sh

# 3. Activate environment
source .venv/bin/activate

# 4. Configure API keys
cp .env.example .env
# Edit .env with your actual API keys
```

### Directory Structure

```
ai-expert-journey/
├── shared/                    # Cross-project utilities and models
│   ├── config.py              # ProjectConfig (env management, paths)
│   ├── models/                # BaseModel, MetricResult, Issue, Severity
│   └── utils/                 # @timed, @retry decorators
├── projects/
│   ├── 01-code-reviewer/      # CLI tool (AST + rules)
│   ├── 02-vector-engine/      # HNSW index (Python + SIMD)
│   ├── 03-rag-system/         # Complex document RAG (FastAPI service)
│   ├── 04-multi-agent/        # Agent orchestration (FastAPI service)
│   ├── 05-recsys/             # Recommendation engine (FastAPI service)
│   ├── 06-llm-inference/      # vLLM + quantization (batch scripts)
│   ├── 07-finetuning/         # SFT/RLHF (training jobs)
│   ├── 08-multimodal/         # VLM + image gen (FastAPI service)
│   └── 09-ai-platform/        # Gateway + orchestration (FastAPI service)
├── docs/
│   ├── adr/                   # Architecture Decision Records
│   ├── architecture/          # Design docs and diagrams
│   └── notes/                 # Personal study notes
├── scripts/
│   ├── setup.sh               # Environment bootstrap
│   ├── ports.sh               # Service management (start/stop/status/logs)
│   └── benchmark.sh           # Per-project benchmark runner
├── docker-compose.yml         # All services + infrastructure
├── .gitlab-ci.yml             # CI pipeline (lint/test/build/deploy)
└── pyproject.toml             # Root project config, ruff, mypy, pytest
```

## Available Commands

<!-- AUTO-GENERATED -->

### Root Commands

| Command | Description |
|---------|-------------|
| `source scripts/setup.sh` | Bootstrap environment (venv, deps, .env) |
| `source .venv/bin/activate` | Activate virtual environment |
| `scripts/ports.sh start` | Start all Docker Compose services |
| `scripts/ports.sh stop` | Stop all services |
| `scripts/ports.sh status` | Check service status (port-based) |
| `scripts/ports.sh logs [svc]` | Tail logs for a specific service |
| `scripts/benchmark.sh <project>` | Run benchmarks for a specific project |
| `ruff check projects/XX-xxx/src/ tests/` | Lint a specific project |
| `ruff format projects/XX-xxx/src/ tests/` | Format a specific project |
| `mypy projects/XX-xxx/src/` | Type-check a specific project |
| `pytest projects/XX-xxx/tests/ --cov=src/` | Test a specific project with coverage |

### Per-Project Entry Points

| Project | Command | Description |
|---------|---------|-------------|
| `01-code-reviewer` | `python -m src.main <path>` | CLI code review tool |
| `02-vector-engine` | `python -m src.main` | Vector retrieval engine entry |
| `03-rag-system` | `uvicorn src.main:app --port 8001` | RAG API service |
| `04-multi-agent` | `uvicorn src.main:app --port 8002` | Agent orchestrator service |
| `05-recsys` | `uvicorn src.main:app --port 8003` | Recommendation API |
| `06-llm-inference` | `python -m src.main` | Inference optimization scripts |
| `07-finetuning` | `python -m src.main` | Fine-tuning training jobs |
| `08-multimodal` | `uvicorn src.main:app --port 8005` | Multimodal generation API |
| `09-ai-platform` | `uvicorn src.main:app --port 8000` | Platform gateway |

<!-- /AUTO-GENERATED -->

## Testing

### Running Tests

```bash
# Test a specific project
cd projects/03-rag-system
pytest tests/ --cov=src/ --cov-report=term-missing

# Test all projects (via CI template)
for proj in projects/*/; do
  cd "$proj" && pytest tests/ --cov=src/ --cov-report=xml || exit 1
  cd -
done
```

### Writing Tests

Follow AAA pattern (Arrange-Act-Assert):

```python
def test_rag_retrieves_correct_document():
    # Arrange
    config = ProjectConfig()
    rag = RagSystem(config)

    # Act
    result = rag.query("带宽要求是多少?")

    # Assert
    assert "10Gbps" in result.answer
    assert result.confidence > 0.8
```

### Coverage Target: 80% minimum

## Code Style

### Linting

```bash
ruff check projects/XX-xxx/src/ tests/      # Check for issues
ruff format projects/XX-xxx/src/ tests/     # Auto-format
ruff check --fix projects/XX-xxx/src/       # Auto-fix safe issues
```

Configured in `pyproject.toml`:
- Rules: `E`, `F`, `I`, `N`, `W`, `UP`
- Line length: 100
- Python target: 3.11

### Type Checking

```bash
mypy projects/XX-xxx/src/ --strict
```

### Pre-commit (Optional)

```bash
pre-commit install
```

## PR Submission Checklist

- [ ] All linting passes (`ruff check`)
- [ ] Type checking passes (`mypy`)
- [ ] Tests pass with 80%+ coverage
- [ ] ADR created/updated for architectural decisions
- [ ] `.env.example` updated if new env vars added
- [ ] `docker-compose.yml` updated if new services added
- [ ] `requirements.txt` updated with new dependencies
- [ ] No hardcoded secrets or API keys in commits
