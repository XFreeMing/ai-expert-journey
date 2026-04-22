# Environment Variables

<!-- AUTO-GENERATED from .env.example and docker-compose.yml -->

## Application Variables (.env)

| Variable | Required | Description | Example | Default |
|----------|----------|-------------|---------|---------|
| `OPENAI_API_KEY` | Yes | OpenAI API key for LLM calls | `sk-xxx` | — |
| `ANTHROPIC_API_KEY` | Yes | Anthropic API key for Claude calls | `sk-ant-xxx` | — |
| `HF_TOKEN` | Yes | Hugging Face token for model access | `hf_xxx` | — |
| `REDIS_URL` | Yes | Redis connection string | `redis://localhost:6379` | `redis://localhost:6379` |
| `POSTGRES_URL` | Yes | PostgreSQL connection string | `postgresql://user:pass@host:5432/db` | `postgresql://ai_expert:changeme@localhost:5432/ai_expert` |
| `MILVUS_HOST` | No | Milvus vector DB hostname | `localhost` | `localhost` |
| `MILVUS_PORT` | No | Milvus vector DB port | `19530` | `19530` |
| `DEFAULT_EMBEDDING_MODEL` | No | Default embedding model | `sentence-transformers/all-MiniLM-L6-v2` | `sentence-transformers/all-MiniLM-L6-v2` |
| `DEFAULT_LLM_MODEL` | No | Default LLM for generation | `gpt-4o` | `gpt-4o` |
| `DEFAULT_LLM_PROVIDER` | No | Default LLM provider | `openai`, `anthropic` | `openai` |

## Docker Compose Service Variables

### Infrastructure Services

| Service | Port | Image | Health Check |
|---------|------|-------|--------------|
| `redis` | 6379 | redis:7-alpine | `redis-cli ping` |
| `postgres` | 5432 | postgres:16-alpine | `pg_isready -U ai_expert` |
| `etcd` | 2379 | quay.io/coreos/etcd:v3.5.10 | `etcdctl endpoint health` |
| `milvus` | 19530 | milvusdb/milvus:v2.4.0 | — |
| `minio` | — | minio/minio:latest | — |

### Application Services

| Service | External Port | Internal Port | Dependencies |
|---------|---------------|---------------|--------------|
| `rag-api` | 8001 | 8000 | redis, milvus |
| `agent-orchestrator` | 8002 | 8000 | redis |
| `recsys-api` | 8003 | 8000 | redis, postgres |
| `vllm-server` | 8004 | 8000 | NVIDIA GPU |
| `multimodal-api` | 8005 | 8000 | redis |
| `platform-gateway` | 8000 | 8000 | rag-api, agent-orchestrator, recsys-api, multimodal-api, vllm-server |

### Service Environment

| Variable | Service | Description |
|----------|---------|-------------|
| `HUGGING_FACE_HUB_TOKEN` | vllm-server | HF token for model download |
| `REDIS_URL` | rag-api, agent-orchestrator, recsys-api, multimodal-api | Redis connection |
| `MILVUS_HOST` | rag-api | Milvus hostname |
| `MILVUS_PORT` | rag-api | Milvus port |
| `POSTGRES_URL` | recsys-api | PostgreSQL connection |
| `RAG_API_URL` | platform-gateway | RAG service URL |
| `AGENT_API_URL` | platform-gateway | Agent orchestrator URL |
| `RECSYS_API_URL` | platform-gateway | Recommendation API URL |
| `MULTIMODAL_API_URL` | platform-gateway | Multimodal API URL |
| `VLLM_URL` | platform-gateway | vLLM inference URL |

## Shared Module Configuration

`shared/config.py` — `ProjectConfig` class provides:

```python
config = ProjectConfig()
config.openai_api_key      # Reads OPENAI_API_KEY from env
config.hf_token            # Reads HF_TOKEN from env
config.project_root        # Auto-detected root path
config.data_dir            # ./data/ (auto-created)
config.model_cache_dir     # ~/.cache/ai-expert-journey/
```

<!-- /AUTO-GENERATED -->
