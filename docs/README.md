# 文档中心

## 快速导航

| 文档 | 路径 | 内容 |
|------|------|------|
| 路线图 | `README.md` | 9 个月学习计划、项目详情、验证标准 |
| 贡献指南 | `docs/CONTRIBUTING.md` | 环境搭建、可用命令、测试规范、代码风格、PR 检查清单 |
| 环境变量 | `docs/ENV.md` | 应用变量、服务编排、数据卷、共享模块配置 |
| 运维手册 | `docs/RUNBOOK.md` | 部署流程、健康检查、常见问题、回滚、告警 |
| 架构决策 | `docs/adr/` | 每个项目的 ADR 记录 |
| 架构设计 | `docs/architecture/` | 设计文档与架构图 |
| 学习笔记 | `docs/notes/` | 算法直觉笔记、实现深挖 |

## ADR（架构决策记录）

<!-- AUTO-GENERATED 来源：docs/adr/ -->

| # | 标题 | 状态 | 所属项目 |
|---|------|------|---------|
| 001 | Code Reviewer architecture | Proposed | 01-code-reviewer |
| 002 | Vector Engine HNSW + 内存优化设计 | Proposed | 02-vector-engine |
| 003 | 复杂文档RAG流水线 | Proposed | 03-rag-system |
| 004 | 多Agent业务编排架构 | Proposed | 04-multi-agent |
| 005 | Recommendation engine architecture | Proposed | 05-recsys |
| 006 | LLM Inference + 本地优先架构优化 | Proposed | 06-llm-inference |
| 007 | 领域模型微调（SFT/RLHF）架构 | Proposed | 07-finetuning |
| 008 | 多模态创意生成工作流 | Proposed | 08-multimodal |
| 009 | AI Platform gateway | Proposed | 09-ai-platform |

<!-- /AUTO-GENERATED -->

## 文档生成规则

| 文档 | 真相源 | 生成时机 |
|------|--------|---------|
| `docs/ENV.md` | `.env.example` + `docker-compose.yml` + `shared/config.py` | 环境变量变更时 |
| `docs/CONTRIBUTING.md` | `pyproject.toml` + `scripts/` + `projects/*/src/main.py` | 命令/依赖变更时 |
| `docs/RUNBOOK.md` | `docker-compose.yml` + `scripts/ports.sh` | 服务拓扑变更时 |
| `docs/README.md` | `docs/adr/` 目录 | ADR 新增时 |
