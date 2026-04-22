# Day 10 —— SIMD 优化 + 内存映射

> 目标：给向量相似度计算加 SIMD 加速，实现 mmap 索引加载。
> 项目：`projects/02-vector-engine/`

---

## Task 28：SIMD 余弦相似度

**产出物**：`projects/02-vector-engine/src/similarity.rs` SIMD 版本 + 正确性 + 性能对比

- [ ] 28.1 回顾 scalar 版本的 cosine_similarity 实现，确认其正确性作为基线（15min）
- [ ] 28.2 调研 Rust SIMD 方案：`std::simd`（nightly feature）vs `fast_simd` crate vs 手动循环展开，选择一种实现（30min）
- [ ] 28.3 实现 `fn cosine_similarity_simd(a: &[f32], b: &[f32]) -> f32`，处理向量长度不是 SIMD 宽度整数倍的尾部情况（45min）
- [ ] 28.4 编写正确性测试：对 100 对随机向量，验证 SIMD 和 scalar 结果差异 < 1e-5（20min）
- [ ] 28.5 使用 criterion 对比 scalar vs SIMD 在 128 维和 768 维向量上的性能，记录加速比（20min）

## Task 29：搜索路径的 SIMD 集成

**产出物**：search_layer 和 greedy_search 使用 SIMD 版本

- [ ] 29.1 修改 `search_layer`，在内部调用 `cosine_similarity_simd` 替代 scalar 版本（20min）
- [ ] 29.2 修改 `greedy_search` 中所有涉及距离计算的位置，统一使用 SIMD 版本（20min）
- [ ] 29.3 运行之前的端到端测试，确保 SIMD 替换后搜索结果不变（15min）
- [ ] 29.4 使用 criterion 测量搜索延迟的端到端改进（scalar vs SIMD），记录 P50/P95 差异（20min）

## Task 30：内存映射（mmap）索引加载

**产出物**：`projects/02-vector-engine/src/mmap.rs` + 调研报告

- [ ] 30.1 调研 mmap 在向量检索中的应用：零拷贝、按需加载、OS 缓存管理，记录 3 个优势和 2 个风险（页面错误、并发安全）（30min）
- [ ] 30.2 实现索引序列化：`fn save_graph(graph: &HNSWGraph, path: &Path)` 将节点向量和连接写入二进制文件（30min）
- [ ] 30.3 实现 mmap 加载：使用 `memmap2` crate 将索引文件映射到内存，直接从 mmap 区域读取向量数据（40min）
- [ ] 30.4 编写测试：构建一个小型索引 → 保存到文件 → mmap 加载 → 执行搜索验证结果正确（20min）
- [ ] 30.5 在 `docs/notes/mmap-vector-search.md` 中记录实现细节和适用场景（15min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| SIMD 相似度 + 正确性/性能对比 | ☐ |
| 搜索路径 SIMD 集成 + 端到端验证 | ☐ |
| mmap 索引序列化/加载 + 测试 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘
- SIMD 加速比：
- mmap 适用场景判断：
- 明天重点（项目 2 收尾基准测试）：
