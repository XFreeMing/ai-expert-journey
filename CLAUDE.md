# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with this repository.

## 项目概述

**ai-expert-journey** 是一个项目制 AI 算法专家成长路线图，面向有工程经验的后端程序员。包含 9 个渐进式项目，覆盖向量检索、RAG、Agent 编排、推荐系统、LLM 推理优化、模型微调、多模态生成、企业中台架构。

**目标**：不是调 API 的"调包侠"，而是能深入到模型权重、数据流、I/O 瓶颈层面的工程型算法专家。

## 目录架构

```
ai-expert-journey/
├── shared/                    # 跨项目通用模块
│   ├── pyproject.toml         # shared 包声明
│   ├── config.py              # ProjectConfig（环境变量管理、路径管理）
│   ├── models/                # 核心领域模型（BaseModel、MetricResult、Issue、Severity）
│   └── utils/                 # 工具函数（@timed、@retry 装饰器）
├── projects/                  # 9 个学习项目
│   ├── 01-code-reviewer/      # Rust CLI 工具：智能代码审查（tree-sitter + 规则引擎）
│   ├── 02-vector-engine/      # Rust 向量检索引擎（HNSW + SIMD 优化）
│   ├── 03-rag-system/         # Python 复杂文档 RAG（FastAPI 服务，端口 8001）
│   ├── 04-multi-agent/        # Python 多 Agent 编排（FastAPI 服务，端口 8002）
│   ├── 05-recsys/             # Python 实时推荐引擎（FastAPI 服务，端口 8003）
│   ├── 06-llm-inference/      # Rust LLM 推理优化（GGUF 解析 + 推理工具）
│   ├── 07-finetuning/         # Python 领域模型微调（SFT/RLHF 训练任务）
│   ├── 08-multimodal/         # Python 多模态创意生成（FastAPI 服务，端口 8005）
│   └── 09-ai-platform/        # Rust 企业中台网关（Axum 服务，端口 8000）
├── docs/                      # 文档中心
│   ├── adr/                   # 架构决策记录（ADR）
│   ├── CONTRIBUTING.md        # 贡献指南（环境搭建、命令、测试）
│   ├── ENV.md                 # 环境变量文档
│   ├── RUNBOOK.md             # 运维手册
│   └── README.md              # 文档导航
├── scripts/                   # 自动化脚本
├── docker-compose.yml         # 全部服务编排
├── .gitlab-ci.yml             # CI/CD 流水线
└── pyproject.toml             # uv workspace 配置、ruff、mypy、pytest
```

## 项目架构

### 共享模块（shared/）

所有 Python 项目依赖的通用模块：
- `ProjectConfig`：自动检测项目根目录、管理 `.env` 变量、提供 `default_embedding_model` 等属性
- 领域模型：`BaseModel`（UUID + 时间戳）、`MetricResult`（评测结果）、`Issue`（问题发现记录，含 Severity 枚举）
- 工具函数：`@timed`（执行时间测量）、`@retry`（指数退避重试）

### 语言分布

| 类型 | 项目 | 语言 | 运行方式 |
|------|------|------|---------|
| Rust CLI 工具 | 01、02、06 | Rust | `cargo run -- <参数>`，有 Dockerfile（cargo-chef） |
| Rust 服务 | 09 | Rust (Axum) | `cargo run -- --bind 0.0.0.0:8000`，有 Dockerfile |
| FastAPI 服务 | 03、04、05、08 | Python | `cd projects/XX-xxx && uv run uvicorn src.main:app --port XXXX`，有 Dockerfile（uv） |
| 训练脚本 | 07 | Python | `cd projects/07-finetuning && uv run python -m src.main`，离线运行，无 Dockerfile |

### Docker Compose 服务拓扑

```
platform-gateway:8000 (Rust) ──┬──> rag-api:8001 ──> redis:6379, milvus:19530
                               ├──> agent-orchestrator:8002 ──> redis:6379
                               ├──> recsys-api:8003 ──> redis:6379, postgres:5432
                               ├──> multimodal-api:8005 ──> redis:6379
                               └──> vector-engine-api:8006 (Rust)

基础设施：redis、postgres、etcd、milvus、minio
按需工具：code-reviewer-cli (Rust)、llm-inference-tools (Rust)
```

### CI 流水线（.gitlab-ci.yml）

四个阶段：`lint` → `test` → `build` → `deploy`
- lint：Python 项目执行 `uv run ruff check` + `uv run ruff format --check`，Rust 项目执行 `cargo fmt --check` + `cargo clippy -- -D warnings`
- test：Python 项目执行 `uv run pytest --cov=src/`，Rust 项目执行 `cargo test --all-features`
- build：编译 shared 模块 + 构建 platform Docker 镜像
- deploy：仅 main 分支，手动触发，执行 `docker compose up -d`

## 常用命令

### 环境搭建

```bash
# 一键安装（创建 venv、安装依赖、复制 .env.example）
source scripts/setup.sh
source .venv/bin/activate

# 确认 Rust 工具链
rustup show
```

### 服务管理

```bash
scripts/ports.sh start      # 启动全部 Docker Compose 服务
scripts/ports.sh stop       # 停止全部服务
scripts/ports.sh status     # 检查端口占用状态
scripts/ports.sh logs [svc] # 查看服务日志
```

### Python 项目开发

```bash
# 代码规范检查
uv run ruff check projects/XX-xxx/src/ tests/
uv run ruff format projects/XX-xxx/src/ tests/

# 类型检查
uv run mypy projects/XX-xxx/src/ --strict

# 运行测试
cd projects/XX-xxx && uv run pytest tests/ --cov=src/ --cov-report=term-missing

# 添加依赖
cd projects/XX-xxx && uv add <package>
```

### Rust 项目开发

```bash
# 代码格式化
cargo fmt
cargo fmt --check

# Lint
cargo clippy
cargo clippy -- -D warnings

# 构建
cargo build --release

# 运行
cargo run -- <参数>

# 测试
cargo test --all-features
```

### 基准测试

```bash
scripts/benchmark.sh <project-name>   # 运行指定项目的 benchmark
```

## 编码规范

### Python
- Python >= 3.11
- 行宽 100 字符
- Lint 规则：`E`（错误）、`F`（Pyflakes）、`I`（排序）、`N`（命名）、`W`（警告）、`UP`（升级）
- 类型检查：`mypy --strict`
- 测试覆盖率：最低 80%
- 测试命名：`test_*.py` 文件，`test_*` 函数，AAA 模式

### Rust
- Rust >= 1.75 (edition 2021)
- 使用 `cargo fmt` 格式化
- 使用 `cargo clippy -- -D warnings` lint
- 测试覆盖率：最低 80%
- 错误处理：使用 `thiserror` 定义错误类型，使用 `Result<T, E>` 传播错误
- 不可变性：默认 `let` 绑定不可变，需要变更时显式使用 `mut`

## 环境变量

基础设施：`REDIS_URL`（默认 redis://localhost:6379）、`POSTGRES_URL`、`MILVUS_HOST`、`MILVUS_PORT`
模型配置：`DEFAULT_EMBEDDING_MODEL`

详见 `docs/ENV.md`。

## 关键约束

- 项目 01、02、06、09 是 Rust 项目，有独立的 `Cargo.toml` 和 `src/main.rs`
- 项目 03、04、05、07、08 是 Python 项目，有独立的 `pyproject.toml`，通过 uv workspace 共享 `shared/` 模块
- 服务化项目通过 `shared/config.py` 获取配置（仅 Python 项目）
- Dockerfile 的 build context 是项目根目录（不是项目子目录），可访问 `shared/` 模块
- Rust Dockerfile 使用 `cargo-chef` 多阶段构建优化缓存
- Python Dockerfile 使用 `uv sync --frozen` 安装依赖
- 项目 07（微调）是离线训练任务，不作为服务运行
- `.env` 已在 `.gitignore` 中，勿提交
- `Cargo.lock` 已在 `.gitignore` 中
- `uv.lock` 已提交，确保依赖版本一致
