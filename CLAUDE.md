# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with this repository.

## 项目概述

**ai-expert-journey** 是一个项目制 AI 算法专家成长路线图，面向有工程经验的后端程序员。包含 9 个渐进式项目，覆盖向量检索、RAG、Agent 编排、推荐系统、LLM 推理优化、模型微调、多模态生成、企业中台架构。

**目标**：不是调 API 的"调包侠"，而是能深入到模型权重、数据流、I/O 瓶颈层面的工程型算法专家。

## 目录架构

```
ai-expert-journey/
├── shared/                    # 跨项目通用模块
│   ├── config.py              # ProjectConfig（环境变量管理、路径管理）
│   ├── models/                # 核心领域模型（BaseModel、MetricResult、Issue、Severity）
│   └── utils/                 # 工具函数（@timed、@retry 装饰器）
├── projects/                  # 9 个学习项目
│   ├── 01-code-reviewer/      # CLI 工具：智能代码审查（AST + 规则引擎）
│   ├── 02-vector-engine/      # 向量检索引擎（HNSW + SIMD 优化）
│   ├── 03-rag-system/         # 复杂文档 RAG（FastAPI 服务，端口 8001）
│   ├── 04-multi-agent/        # 多 Agent 编排（FastAPI 服务，端口 8002）
│   ├── 05-recsys/             # 实时推荐引擎（FastAPI 服务，端口 8003）
│   ├── 06-llm-inference/      # LLM 推理优化（vLLM + 量化脚本）
│   ├── 07-finetuning/         # 领域模型微调（SFT/RLHF 训练任务）
│   ├── 08-multimodal/         # 多模态创意生成（FastAPI 服务，端口 8005）
│   └── 09-ai-platform/        # 企业中台网关（FastAPI 服务，端口 8000）
├── docs/                      # 文档中心
│   ├── adr/                   # 架构决策记录（ADR）
│   ├── CONTRIBUTING.md        # 贡献指南（环境搭建、命令、测试）
│   ├── ENV.md                 # 环境变量文档
│   ├── RUNBOOK.md             # 运维手册
│   └── README.md              # 文档导航
├── scripts/                   # 自动化脚本
├── docker-compose.yml         # 全部服务编排
├── .gitlab-ci.yml             # CI/CD 流水线
└── pyproject.toml             # 根项目配置
```

## 项目架构

### 共享模块（shared/）

所有项目依赖的通用模块：
- `ProjectConfig`：自动检测项目根目录、管理 `.env` 变量、提供 `openai_api_key` / `hf_token` 属性
- 领域模型：`BaseModel`（UUID + 时间戳）、`MetricResult`（评测结果）、`Issue`（问题发现记录，含 Severity 枚举）
- 工具函数：`@timed`（执行时间测量）、`@retry`（指数退避重试）

### 服务化项目 vs 脚本项目

| 类型 | 项目 | 运行方式 |
|------|------|---------|
| FastAPI 服务 | 03、04、05、08、09 | `uvicorn src.main:app --port XXXX`，有 Dockerfile |
| CLI 工具 | 01、02 | `python -m src.main <参数>`，无 Dockerfile |
| 训练脚本 | 06、07 | `python -m src.main`，离线运行，无 Dockerfile |

### Docker Compose 服务拓扑

```
platform-gateway:8000 ──┬──> rag-api:8001 ──> redis:6379, milvus:19530
                        ├──> agent-orchestrator:8002 ──> redis:6379
                        ├──> recsys-api:8003 ──> redis:6379, postgres:5432
                        ├──> multimodal-api:8005 ──> redis:6379
                        └──> vllm-server:8004 ──> GPU（需要 NVIDIA）

基础设施：redis、postgres、etcd、milvus、minio
```

### CI 流水线（.gitlab-ci.yml）

四个阶段：`lint` → `test` → `build` → `deploy`
- lint：每个项目独立执行 `ruff check` + `ruff format --check`
- test：每个项目独立执行 `pytest --cov=src/`，覆盖率产物为 Cobertura XML
- build：编译 shared 模块 + 构建 platform Docker 镜像
- deploy：仅 main 分支，手动触发，执行 `docker compose up -d`

## 常用命令

### 环境搭建

```bash
# 一键安装（创建 venv、安装依赖、复制 .env.example）
source scripts/setup.sh
source .venv/bin/activate
```

### 服务管理

```bash
scripts/ports.sh start      # 启动全部 Docker Compose 服务
scripts/ports.sh stop       # 停止全部服务
scripts/ports.sh status     # 检查端口占用状态
scripts/ports.sh logs [svc] # 查看服务日志
```

### 开发命令

```bash
# 代码规范检查
ruff check projects/XX-xxx/src/ tests/
ruff format projects/XX-xxx/src/ tests/

# 类型检查
mypy projects/XX-xxx/src/ --strict

# 运行测试
cd projects/XX-xxx && pytest tests/ --cov=src/ --cov-report=term-missing

# 运行单个项目
python -m projects/01-code-reviewer/src/main.py ./path/to/code --lang python
```

### 基准测试

```bash
scripts/benchmark.sh <project-name>   # 运行指定项目的 benchmark
```

## 编码规范

- Python >= 3.11
- 行宽 100 字符
- Lint 规则：`E`（错误）、`F`（Pyflakes）、`I`（排序）、`N`（命名）、`W`（警告）、`UP`（升级）
- 类型检查：`mypy --strict`
- 测试覆盖率：最低 80%
- 测试命名：`test_*.py` 文件，`test_*` 函数，AAA 模式

## 环境变量

必需：`OPENAI_API_KEY`、`ANTHROPIC_API_KEY`、`HF_TOKEN`
基础设施：`REDIS_URL`（默认 redis://localhost:6379）、`POSTGRES_URL`、`MILVUS_HOST`、`MILVUS_PORT`
模型配置：`DEFAULT_EMBEDDING_MODEL`、`DEFAULT_LLM_MODEL`、`DEFAULT_LLM_PROVIDER`

详见 `docs/ENV.md`。

## 关键约束

- 每个项目有独立的 `requirements.txt` 和 `src/`、`tests/`、`docs/` 目录
- 服务化项目通过 `shared/config.py` 获取配置
- Dockerfile 的 build context 是项目根目录（不是项目子目录），可访问 `shared/` 模块
- 项目 06（vLLM）直接使用官方镜像 `vllm/vllm-openai:latest`，不需要 Dockerfile
- 项目 07（微调）是离线训练任务，不作为服务运行
- `.env` 已在 `.gitignore` 中，勿提交
