# Day 15 —— 检索 + 排序 + 基础 RAG Pipeline

> 目标：实现向量检索、重排序、基础 RAG 问答闭环。
> 项目：`projects/03-rag-system/src/`

---

## Task 43：向量存储 + 检索

**产出物**：`projects/03-rag-system/src/vector_store.py` + `src/retriever.py` + 测试

- [ ] 43.1 选择向量存储：本地 FAISS（快速原型）或 Milvus（Docker Compose 已有）。实现 `class VectorStore`：`add(ids, vectors, metadata)` 和 `search(query_vector, top_k) -> list[Result]`（40min）
- [ ] 43.2 如果使用 FAISS：安装 `faiss-cpu`，实现 IndexFlatIP（内积/余弦相似度）索引；存储 chunk metadata 到内存 dict 中（30min）
- [ ] 43.3 实现 `class Retriever: def retrieve(query: str, top_k: int = 5) -> list[Chunk]`：使用 Embedder 将 query 向量化，调用 VectorStore 搜索，按相似度排序返回 top-K（25min）
- [ ] 43.4 编写测试：存入 5 个已知内容的 chunk，用语义相关的 query 检索，验证 top-1 返回预期 chunk（20min）

## Task 44：重排序 + LLM 问答生成

**产出物**：`projects/03-rag-system/src/reranker.py` + `src/generator.py` + 端到端测试

- [ ] 44.1 实现简单重排序：先用 BM25（`pip install rank-bm25`）对检索结果做关键词匹配重排，或使用交叉编码器（`cross-encoder/ms-marco-MiniLM-L-6-v2`）做语义重排（40min）
- [ ] 44.2 实现 `class Generator: def generate(query: str, context_chunks: list[Chunk]) -> str`：拼接 prompt（`"基于以下上下文回答问题：\n\n{context}\n\n问题：{query}"`），调用 OpenAI/Claude API 生成回答（30min）
- [ ] 44.3 实现 prompt 长度控制：当 context 总 token 数超过模型限制时，按相关度截断最少的 chunk（15min）
- [ ] 44.4 端到端测试：上传一个 Markdown 文档 → 解析 → 分块 → 向量化 → 提问 → 生成回答，验证完整流程无报错（20min）

## Task 45：基础 RAG Pipeline 整合

**产出物**：`projects/03-rag-system/src/pipeline.py`（完整 RAG 管线）+ 项目 03 第一篇笔记

- [ ] 45.1 实现 `class RAGPipeline`：串联 `Parser → Chunker → Embedder → VectorStore → Retriever → Reranker → Generator`，提供 `ingest(file_path)` 和 `query(question)` 两个核心方法（40min）
- [ ] 45.2 编写集成测试：用一个包含技术内容的 Markdown 文件（如本文档），ingest 后提出 3 个与内容相关的问题，验证回答包含关键信息（25min）
- [ ] 45.3 记录管线各步骤的耗时分布，识别瓶颈（通常是 embedding 或 LLM 调用）（15min）
- [ ] 45.4 写 `docs/notes/rag-challenges.md`：记录今天遇到的 3 个关键问题及解决方案，列出下一步优化方向（混合检索、HyDE、评测框架、FastAPI 服务化）（20min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| 向量存储 + 检索可用 | ☐ |
| 重排序 + LLM 问答生成可用 | ☐ |
| 完整 RAG Pipeline + 端到端测试 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘
- RAG 管线端到端耗时：
- 最难的部分：
- 下一步优化优先级：
