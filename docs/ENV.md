# 环境变量文档

<!-- AUTO-GENERATED 来源：.env.example + docker-compose.yml + shared/config.py -->

## 应用级变量（.env）

| 变量名 | 必需 | 描述 | 示例值 | 默认值 |
|--------|------|------|--------|--------|
| `OPENAI_API_KEY` | 是 | OpenAI API 密钥 | `sk-xxx` | — |
| `ANTHROPIC_API_KEY` | 是 | Anthropic API 密钥（Claude） | `sk-ant-xxx` | — |
| `HF_TOKEN` | 是 | Hugging Face 模型下载令牌 | `hf_xxx` | — |
| `REDIS_URL` | 是 | Redis 连接字符串 | `redis://localhost:6379` | `redis://localhost:6379` |
| `POSTGRES_URL` | 是 | PostgreSQL 连接字符串 | `postgresql://user:pass@host:5432/db` | `postgresql://ai_expert:changeme@localhost:5432/ai_expert` |
| `MILVUS_HOST` | 否 | Milvus 向量数据库主机名 | `localhost` | `localhost` |
| `MILVUS_PORT` | 否 | Milvus 向量数据库端口 | `19530` | `19530` |
| `DEFAULT_EMBEDDING_MODEL` | 否 | 默认 Embedding 模型 | `sentence-transformers/all-MiniLM-L6-v2` | `sentence-transformers/all-MiniLM-L6-v2` |
| `DEFAULT_LLM_MODEL` | 否 | 默认 LLM 生成模型 | `gpt-4o` | `gpt-4o` |
| `DEFAULT_LLM_PROVIDER` | 否 | 默认 LLM 提供商 | `openai`、`anthropic` | `openai` |

## Docker Compose 服务编排

### 基础设施服务

| 服务名 | 外部端口 | 镜像 | 健康检查 |
|--------|---------|------|---------|
| `redis` | 6379 | redis:7-alpine | `redis-cli ping` → `PONG` |
| `postgres` | 5432 | postgres:16-alpine | `pg_isready -U ai_expert` → 退出码 0 |
| `etcd` | 2379 | quay.io/coreos/etcd:v3.5.10 | `etcdctl endpoint health` → healthy |
| `milvus` | 19530 | milvusdb/milvus:v2.4.0 | — |
| `minio` | — | minio/minio:latest | — |

### 应用服务

| 服务名 | 外部端口 | 内部端口 | 依赖服务 |
|--------|---------|---------|---------|
| `rag-api` | 8001 | 8000 | redis、milvus |
| `agent-orchestrator` | 8002 | 8000 | redis |
| `recsys-api` | 8003 | 8000 | redis、postgres |
| `vllm-server` | 8004 | 8000 | NVIDIA GPU（需 GPU 驱动） |
| `multimodal-api` | 8005 | 8000 | redis |
| `platform-gateway` | 8000 | 8000 | rag-api、agent-orchestrator、recsys-api、multimodal-api、vllm-server |

### 服务级环境变量

| 变量名 | 所属服务 | 描述 |
|--------|---------|------|
| `HUGGING_FACE_HUB_TOKEN` | vllm-server | HF 模型下载令牌（从 ${HF_TOKEN} 注入） |
| `REDIS_URL` | rag-api、agent-orchestrator、recsys-api、multimodal-api | Redis 连接地址 |
| `MILVUS_HOST` | rag-api | Milvus 服务主机名 |
| `MILVUS_PORT` | rag-api | Milvus 服务端口 |
| `POSTGRES_URL` | recsys-api | PostgreSQL 连接地址 |
| `RAG_API_URL` | platform-gateway | RAG 服务回调地址 |
| `AGENT_API_URL` | platform-gateway | Agent 编排器回调地址 |
| `RECSYS_API_URL` | platform-gateway | 推荐引擎回调地址 |
| `MULTIMODAL_API_URL` | platform-gateway | 多模态服务回调地址 |
| `VLLM_URL` | platform-gateway | vLLM 推理服务地址 |

### 数据卷

| 卷名 | 挂载服务 | 用途 |
|------|---------|------|
| `redis-data` | redis | Redis 持久化数据 |
| `postgres-data` | postgres | PostgreSQL 数据库文件 |
| `milvus-data` | milvus | Milvus 向量索引数据 |
| `minio-data` | minio | MinIO 对象存储数据 |
| `model-cache` | vllm-server | HuggingFace 模型缓存 |

## 共享模块配置

`shared/config.py` — `ProjectConfig` 类提供统一配置访问：

```python
config = ProjectConfig()
config.openai_api_key      # 从环境变量读取 OPENAI_API_KEY
config.hf_token            # 从环境变量读取 HF_TOKEN
config.project_root        # 自动检测项目根目录
config.data_dir            # ./data/ 目录（自动创建）
config.model_cache_dir     # ~/.cache/ai-expert-journey/ 模型缓存
```

### shared/models — 核心领域模型

| 模型名 | 字段 | 用途 |
|--------|------|------|
| `BaseModel` | id、created_at、updated_at | 所有模型的基类 |
| `MetricResult` | name、value、threshold、passed | 通用评测结果 |
| `Issue` | severity、title、description、location、line_number、suggestion | 通用问题/发现记录 |

### Severity 枚举

| 级别 | 值 | 含义 |
|------|-----|------|
| INFO | `"info"` | 提示信息 |
| LOW | `"low"` | 轻微问题 |
| MEDIUM | `"medium"` | 中等问题 |
| HIGH | `"high"` | 严重问题 |
| CRITICAL | `"critical"` | 关键问题 |

### shared/utils/timing — 工具函数

| 装饰器 | 功能 |
|--------|------|
| `@timed` | 测量函数执行时间，记录到日志 |
| `@retry(max_attempts, delay, backoff)` | 指数退避重试，失败自动重试 |

<!-- /AUTO-GENERATED -->
