# Day 14 —— 分块策略 + 向量化

> 目标：实现两种分块策略，接入 Embedding 模型，完成文档向量化。
> 项目：`projects/03-rag-system/src/`

---

## Task 40：递归分块策略

**产出物**：`projects/03-rag-system/src/chunking/recursive.py` + 测试

- [ ] 40.1 实现 `class RecursiveChunker: def chunk(text: str, chunk_size: int = 500, overlap: int = 50) -> list[Chunk]`，其中 Chunk 包含 text、metadata、source_id（30min）
- [ ] 40.2 按分隔符优先级递归分割：`\n\n`（段落）→ `\n`（行）→ `。` / `.`（句子）→ ` `（词），确保不超过 chunk_size（30min）
- [ ] 40.3 处理 overlap：相邻 chunk 之间保留 overlap 个字符的重叠，避免信息在边界处丢失（20min）
- [ ] 40.4 编写测试：用 1000 字中文文本，chunk_size=500、overlap=50，验证 chunk 数量合理、平均长度接近 chunk_size、边界无截断单词/句子（20min）

## Task 41：结构化分块策略

**产出物**：`projects/03-rag-system/src/chunking/structural.py` + 测试

- [ ] 41.1 实现 `class StructuralChunker: def chunk(document: Document) -> list[Chunk]`，利用解析器提取的 section 层级作为天然分块边界（30min）
- [ ] 41.2 按 section 分块：每个 section 及其子 section 合并为一个 chunk；如果 chunk 超过 max_size，递归分割子 section（25min）
- [ ] 41.3 为每个 chunk 附加元数据：来源 section 标题、层级、页码（PDF）或行号范围（Markdown）（15min）
- [ ] 41.4 编写测试：用一个带 3 层标题的文档，验证 chunk 保持了 section 层级关系，边界在标题处而非文中截断（20min）

## Task 42：Embedding 模型接入

**产出物**：`projects/03-rag-system/src/embedding.py` + 向量化测试

- [ ] 42.1 在项目根目录执行 `uv add sentence-transformers`，阅读其 API 文档（20min）
- [ ] 42.2 实现 `class Embedder`：`__init__` 加载模型（默认 `all-MiniLM-L6-v2`，支持配置）；`embed(texts: list[str]) -> np.ndarray` 批量编码（30min）
- [ ] 42.3 编写测试：encode 3 句已知语义的文本，验证输出维度正确（3, 384）；计算 "猫" 和 "狗" 的余弦相似度（预期 > 0.3）；计算 "猫" 和 "汽车" 的相似度（预期 < 0.1）（20min）
- [ ] 42.4 端到端串联：用一个 Markdown 文件 → Parser → Chunker → Embedder，验证从文件到向量矩阵的完整管线，记录每步耗时（25min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| 递归分块器 + 测试通过 | ☐ |
| 结构化分块器 + 测试通过 | ☐ |
| Embedding 模型 + 端到端管线 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘
- 两种分块策略的适用场景：
- Embedding 模型首次加载耗时：
- 明天重点（向量存储 + 检索）：
