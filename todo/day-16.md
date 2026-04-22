# Day 16 —— RAG 优化：混合检索与上下文压缩

> 目标：实现 BM25 + 向量检索的混合检索（RRF 融合），加入上下文压缩策略提升 RAG 回答质量。
> 项目：`projects/03-rag-system/src/`

---

## Task 46：混合检索 — BM25 + 向量 RRF 融合

**产出物**：`projects/03-rag-system/src/retriever/hybrid.py` + 对比测试

- [ ] 46.1 阅读 BM25 算法原理，理解 TF-IDF 的改进点（长度归一化、饱和度函数）（30min）
- [ ] 46.2 阅读 RRF（Reciprocal Rank Fusion）公式，理解多路召回融合策略（25min）
- [ ] 46.3 实现 `BM25Retriever` 类：对 corpus 建立倒排索引，支持 top-k 检索（30min）
- [ ] 46.4 实现 `HybridRetriever` 类：组合向量检索 top-20 + BM25 top-20，用 RRF 融合取 top-K（30min）
- [ ] 46.5 编写对比测试：同一 query 集，分别跑纯向量和混合检索，记录召回率差异（20min）

---

## Task 47：上下文压缩（Context Compression）

**产出物**：`projects/03-rag-system/src/rag/context_compressor.py`

- [ ] 47.1 阅读 "Lost in the Middle" 论文摘要，理解上下文窗口中位置偏差问题（30min）
- [ ] 47.2 实现 `ContextCompressor`：对检索到的 top-K chunks，用 LLM 生成精简摘要（30min）
- [ ] 47.3 实现启发式过滤：去除与 query 余弦相似度低于阈值的 chunks（20min）
- [ ] 47.4 对比实验：原始 top-10 vs 压缩后 top-10 的生成质量差异（20min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| 混合检索（RRF 融合）可用 | ☐ |
| 纯向量 vs 混合对比数据 | ☐ |
| 上下文压缩模块可用 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘
- 最大收获：
- 遇到的障碍：
- 明天需要调整的：
