# Day 12 —— 缓冲日 + 项目3预习

> 目标：处理 Day 6-11 的遗留问题，预习项目 3（RAG 系统）的核心概念。
> 项目：`projects/02-vector-engine/`（收尾）+ `projects/03-rag-system/`（预习）

---

## Task 34：Day 6-11 遗留问题处理

**产出物**：待办修复清单 + 修复后的验证结果

- [ ] 34.1 回顾 Day 6-11 的完成度检查表，列出未完成的任务（20min）
- [ ] 34.2 优先修复：测试覆盖不足、benchmark 数据缺失、文档不完整等问题（45min）
- [ ] 34.3 对项目 02 运行完整的 `cargo test --all-features` 和 `cargo clippy -- -D warnings`，确保全部通过（20min）
- [ ] 34.4 如果所有任务已完成，提前准备项目 03 的环境：确认 Python venv 中已安装必要依赖（FastAPI、uvicorn 等）（20min）

## Task 35：RAG 系统架构学习

**产出物**：`docs/notes/rag-architecture-overview.md`（架构笔记）

- [ ] 35.1 阅读 2-3 篇 RAG 系统综述文章或博客，理解 RAG 的核心流水线：文档解析 → 分块 → 向量化 → 存储 → 检索 → 生成（45min）
- [ ] 35.2 重点理解每个环节的设计选择：解析器（PDF/Word/Markdown）、分块策略（固定大小/递归/语义）、向量模型（OpenAI embedding/本地 sentence-transformers）、检索（向量/混合）、生成（prompt 工程）（30min）
- [ ] 35.3 记录 RAG 系统的常见挑战：上下文窗口限制、检索召回不足、幻觉、延迟优化（20min）
- [ ] 35.4 画 ASCII 数据流图：Document → Parser → Chunker → Embedder → VectorStore → Retriever → Generator → Answer（15min）

## Task 36：项目 03 环境搭建 + 技术选型

**产出物**：`projects/03-rag-system/` 最小可运行骨架 + 技术选型笔记

- [ ] 36.1 调研文档解析方案：PyMuPDF（PDF）、python-docx（Word）、内置（Markdown），记录各方案的优缺点和依赖复杂度（30min）
- [ ] 36.2 调研 Embedding 方案：OpenAI API（高质量、需 key）vs sentence-transformers（本地、免费）vs Cohere，记录选型建议（20min）
- [ ] 36.3 创建项目 03 骨架：`src/parsers/__init__.py`、`src/chunking/__init__.py`、`src/embedding.py`、`src/vector_store.py`、`src/main.py`（FastAPI 入口）（20min）
- [ ] 36.4 确认 Docker Compose 中的 rag-api 服务配置（端口 8001、依赖 redis + milvus），理解服务拓扑（15min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| Day 6-11 遗留问题已处理 | ☐ |
| RAG 架构学习笔记完成 | ☐ |
| 项目 03 环境搭建 + 技术选型 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘
- 项目 02 总完成度：
- 对 RAG 系统最关心的技术点：
- 明天重点（文档解析）：
