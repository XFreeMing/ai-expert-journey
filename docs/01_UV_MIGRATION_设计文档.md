# Python 项目统一 uv 管理

## 背景

所有 Python 项目（03/04/05/07/08）统一从 `requirements.txt` + `pip` 迁移到 `uv` 管理。

## 架构

```
workspace (根 pyproject.toml, uv workspace)
├── shared/                  # 本地包 [tool.uv.sources]
├── projects/03-rag-system/  # pyproject.toml → depends on shared
├── projects/04-multi-agent/ # pyproject.toml → depends on shared
├── projects/05-recsys/      # pyproject.toml → depends on shared
├── projects/07-finetuning/  # pyproject.toml → depends on shared
└── projects/08-multimodal/  # pyproject.toml → depends on shared
```

## 变更清单

| 操作 | 文件 | 说明 |
|------|------|------|
| 修改 | `pyproject.toml` | 改为 uv workspace 配置 |
| 新增 | `shared/pyproject.toml` | shared 模块包声明 |
| 新增 | `projects/03-rag-system/pyproject.toml` | 项目 3 依赖声明 |
| 新增 | `projects/04-multi-agent/pyproject.toml` | 项目 4 依赖声明 |
| 新增 | `projects/05-recsys/pyproject.toml` | 项目 5 依赖声明 |
| 新增 | `projects/07-finetuning/pyproject.toml` | 项目 7 依赖声明 |
| 新增 | `projects/08-multimodal/pyproject.toml` | 项目 8 依赖声明 |
| 删除 | `projects/*/requirements.txt` (5个) | 被 pyproject.toml 替代 |
| 修改 | `projects/03/04/05/08/Dockerfile` | pip → uv sync |
| 修改 | `.gitlab-ci.yml` | venv+pip → uv sync |
| 修改 | `scripts/setup.sh` | venv+pip → uv venv+sync |
| 修改 | `docs/CONTRIBUTING.md` | 全面更新为 uv 用法 |
| 修改 | `docs/RUNBOOK.md` | 新增 Python 依赖同步排查 |
| 修改 | `CLAUDE.md` | 更新项目命令和架构说明 |

## 开发工作流

```bash
# 安装环境
uv venv && uv sync

# 运行项目
cd projects/03-rag-system && uv run uvicorn src.main:app --port 8001

# 添加依赖
cd projects/03-rag-system && uv add <package>

# 运行测试
cd projects/03-rag-system && uv run pytest tests/ --cov=src/
```
