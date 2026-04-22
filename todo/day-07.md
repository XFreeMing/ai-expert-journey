# Day 7 —— 项目2 启动：HNSW 核心数据结构

> 目标：实现 HNSW 的基础数据结构和余弦相似度计算。
> 项目：`projects/02-vector-engine/`

---

## Task 19：HNSW 数据结构设计

**产出物**：`projects/02-vector-engine/src/hnsw.rs`（HNSWNode + HNSWGraph 结构体）+ 至少 2 个测试

- [ ] 19.1 设计 `HNSWNode` 结构体：包含 node_id、vector（`Vec<f32>`）、neighbors（`Vec<Vec<usize>>`，每层一个邻居列表），考虑使用 `Arena` 或 `Vec` 存储节点以避免 Rc/RefCell 开销（30min）
- [ ] 19.2 设计 `HNSWGraph` 结构体：包含 nodes 存储（`Vec<HNSWNode>` 或 `Arena`）、max_level、m（每层最大连接数）、m_construction（构建时最大连接数）、ef_construction（构建时搜索范围）（25min）
- [ ] 19.3 实现 `HNSWGraph::new(m: usize, ef_construction: usize) -> Self` 构造函数，创建空图（15min）
- [ ] 19.4 编写 2 个测试：创建空图验证初始状态；创建一个包含 3 个节点的图，验证结构正确（20min）

## Task 20：余弦相似度计算

**产出物**：`projects/02-vector-engine/src/similarity.rs` + scalar 版本 + 测试

- [ ] 20.1 实现 `fn cosine_similarity(a: &[f32], b: &[f32]) -> f32`（scalar 版本），处理零向量边界情况（20min）
- [ ] 20.2 编写 3 个测试：相同向量（相似度=1.0）、正交向量（相似度=0.0）、反向向量（相似度=-1.0）（20min）
- [ ] 20.3 实现 `fn cosine_distance(a: &[f32], b: &[f32]) -> f32`（1.0 - similarity），作为搜索时的距离度量（10min）
- [ ] 20.4 使用 `criterion` crate 配置基准测试框架，写一个 128 维向量的相似度基准（20min）

## Task 21：项目 02 测试基础设施

**产出物**：测试工具函数 + 基准测试配置

- [ ] 21.1 创建测试工具模块：`fn random_vector(dim: usize) -> Vec<f32>` 生成随机单位向量；`fn random_vectors(n: usize, dim: usize) -> Vec<Vec<f32>>` 批量生成（20min）
- [ ] 21.2 实现 `fn brute_force_topk(query: &[f32], vectors: &[Vec<f32>], k: usize) -> Vec<(usize, f32)>` 作为 ground truth 参考（25min）
- [ ] 21.3 在 `Cargo.toml` 中配置 `criterion` 作为 dev-dependency，编写基准测试入口（15min）
- [ ] 21.4 运行 `cargo test` 和 `cargo bench` 确认全部通过，记录基线性能数字（15min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| HNSW 数据结构 + 测试通过 | ☐ |
| 余弦相似度 scalar 版 + 测试通过 | ☐ |
| 测试基础设施 + benchmark 配置就绪 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘
- 最大收获：
- 遇到的障碍：
- 明天重点（插入算法）：
