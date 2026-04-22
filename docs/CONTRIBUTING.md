# 贡献指南

## 开发环境搭建

### 前置要求

| 依赖 | 版本 | 用途 |
|------|------|------|
| Python | >= 3.11 | 运行时 |
| Docker + Compose | v24+ / v2.20+ | 服务编排 |
| Git | 2.30+ | 版本控制 |
| (可选) NVIDIA GPU | RTX 3060+ (12GB 显存) | GPU 加速训练/推理 |
| (可选) Apple Silicon | M1/M2/M3/M4 (16GB+) | MLX/GGUF 本地推理 |

### 安装步骤

```bash
# 1. 克隆并初始化
git clone <repo-url>
cd ai-expert-journey

# 2. 一键安装（创建 venv、安装依赖、复制 .env）
source scripts/setup.sh

# 3. 激活虚拟环境
source .venv/bin/activate

# 4. 配置 API 密钥
cp .env.example .env
# 编辑 .env，填入真实的 API 密钥
```

### 目录结构

```
ai-expert-journey/
├── shared/                    # 跨项目通用模块
│   ├── config.py              # ProjectConfig（环境管理、路径）
│   ├── models/                # BaseModel、MetricResult、Issue、Severity
│   └── utils/                 # @timed、@retry 装饰器
├── projects/
│   ├── 01-code-reviewer/      # 智能代码审查（CLI 工具）
│   ├── 02-vector-engine/      # 高性能向量检索（HNSW + SIMD）
│   ├── 03-rag-system/         # 复杂文档 RAG（FastAPI 服务）
│   ├── 04-multi-agent/        # 多 Agent 编排（FastAPI 服务）
│   ├── 05-recsys/             # 实时推荐引擎（FastAPI 服务）
│   ├── 06-llm-inference/      # LLM 推理优化（vLLM + 量化脚本）
│   ├── 07-finetuning/         # 领域模型微调（SFT/RLHF 训练）
│   ├── 08-multimodal/         # 多模态创意生成（FastAPI 服务）
│   └── 09-ai-platform/        # 企业中台网关（FastAPI 服务）
├── docs/
│   ├── adr/                   # 架构决策记录
│   ├── architecture/          # 设计文档与架构图
│   └── notes/                 # 个人学习笔记
├── scripts/
│   ├── setup.sh               # 环境引导脚本
│   ├── ports.sh               # 服务管理（启动/停止/状态/日志）
│   └── benchmark.sh           # 单项目基准测试
├── docker-compose.yml         # 全部服务 + 基础设施编排
├── .gitlab-ci.yml             # CI 流水线（lint/test/build/deploy）
└── pyproject.toml             # 根项目配置、ruff、mypy、pytest
```

## 可用命令

<!-- AUTO-GENERATED -->

### 根级别命令

| 命令 | 描述 |
|------|------|
| `source scripts/setup.sh` | 引导环境（创建 venv、安装依赖、复制 .env） |
| `source .venv/bin/activate` | 激活虚拟环境 |
| `scripts/ports.sh start` | 启动全部 Docker Compose 服务 |
| `scripts/ports.sh stop` | 停止全部服务 |
| `scripts/ports.sh status` | 检查服务状态（基于端口） |
| `scripts/ports.sh logs [服务名]` | 查看指定服务日志 |
| `scripts/benchmark.sh <项目名>` | 运行指定项目的基准测试 |
| `ruff check projects/XX-xxx/src/ tests/` | 检查指定项目的代码规范 |
| `ruff format projects/XX-xxx/src/ tests/` | 格式化指定项目代码 |
| `mypy projects/XX-xxx/src/` | 类型检查指定项目 |
| `pytest projects/XX-xxx/tests/ --cov=src/` | 运行指定项目测试并统计覆盖率 |

### 各项目入口点

| 项目 | 启动方式 | 描述 | 依赖包数 |
|------|---------|------|---------|
| `01-code-reviewer` | `python -m src.main <路径>` | CLI 代码审查工具（支持 --lang、--output） | 9 |
| `02-vector-engine` | `python -m src.main` | 向量检索引擎入口 | 11 |
| `03-rag-system` | `uvicorn src.main:app --port 8001` | RAG 知识系统 API 服务 | 16 |
| `04-multi-agent` | `uvicorn src.main:app --port 8002` | 多 Agent 编排器服务 | 10 |
| `05-recsys` | `uvicorn src.main:app --port 8003` | 实时推荐引擎 API | 14 |
| `06-llm-inference` | `python -m src.main` | LLM 推理优化（vLLM + 量化） | 11 |
| `07-finetuning` | `python -m src.main` | 领域模型微调训练（SFT/RLHF） | 13 |
| `08-multimodal` | `uvicorn src.main:app --port 8005` | 多模态创意生成 API | 14 |
| `09-ai-platform` | `uvicorn src.main:app --port 8000` | 企业中台网关服务 | 12 |

<!-- /AUTO-GENERATED -->

## 测试

### 运行测试

```bash
# 测试单个项目
cd projects/03-rag-system
pytest tests/ --cov=src/ --cov-report=term-missing

# 测试全部项目
for proj in projects/*/; do
  cd "$proj" && pytest tests/ --cov=src/ --cov-report=xml || exit 1
  cd -
done
```

### 编写测试

遵循 AAA 模式（Arrange-Act-Assert）：

```python
def test_rag_retrieves_correct_document():
    # Arrange（准备）
    config = ProjectConfig()
    rag = RagSystem(config)

    # Act（执行）
    result = rag.query("带宽要求是多少?")

    # Assert（断言）
    assert "10Gbps" in result.answer
    assert result.confidence > 0.8
```

### 覆盖率目标：最低 80%

## 代码风格

### 代码规范检查

```bash
ruff check projects/XX-xxx/src/ tests/      # 检查规范问题
ruff format projects/XX-xxx/src/ tests/     # 自动格式化
ruff check --fix projects/XX-xxx/src/       # 自动修复安全问题
```

配置在 `pyproject.toml` 中：
- 规则集：`E`（错误）、`F`（Pyflakes）、`I`（排序）、`N`（命名）、`W`（警告）、`UP`（升级）
- 行宽限制：100
- Python 目标版本：3.11

### 类型检查

```bash
mypy projects/XX-xxx/src/ --strict
```

### Pre-commit（可选）

```bash
pre-commit install
```

## 提交 PR 检查清单

- [ ] 代码规范检查通过（`ruff check`）
- [ ] 类型检查通过（`mypy`）
- [ ] 测试覆盖率 ≥ 80%
- [ ] 架构决策已创建/更新（ADR）
- [ ] 新增环境变量已同步到 `.env.example`
- [ ] 新增服务已更新 `docker-compose.yml`
- [ ] 新增依赖已更新 `requirements.txt`
- [ ] 提交记录中无硬编码密钥或敏感信息
