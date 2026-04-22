# Day 8 —— HNSW 构建算法

> 目标：实现 HNSW 的增量插入算法，支持动态添加向量到索引中。
> 项目：`projects/02-vector-engine/`

---

## Task 22：单层搜索算法

**产出物**：`fn search_layer()` 函数 + 测试

- [ ] 22.1 复习 HNSW 论文中单层搜索的逻辑：从 entry point 出发，在邻居中贪心地寻找更近的点，直到无法改进（30min）
- [ ] 22.2 实现 `fn search_layer(graph: &HNSWGraph, entry_id: usize, query: &[f32], level: usize) -> Vec<usize>`，返回该层最近的候选节点列表（45min）
- [ ] 22.3 编写测试：构建一个 5 节点的小型图，手动指定邻居连接，验证 search_layer 在给定 entry 和 query 下返回正确结果（20min）

## Task 23：层间贪心搜索

**产出物**：`fn greedy_search()` 函数 + 测试

- [ ] 23.1 实现 `fn greedy_search(graph: &HNSWGraph, query: &[f32], ef: usize) -> Vec<usize>`：从最高层 entry 开始，逐层向下搜索，每层调用 search_layer 找到该层最近点，到底层时返回 ef 个最近邻（45min）
- [ ] 23.2 关键细节处理：entry point 的更新逻辑、每层候选集的维护、ef 参数的作用（20min）
- [ ] 23.3 编写测试：插入 10 个 32 维向量后，随机选一个作为 query 执行 greedy_search，验证返回的最近邻与 brute_force 结果一致（20min）

## Task 24：增量插入算法

**产出物**：`fn insert()` 函数 + 端到端测试

- [ ] 24.1 实现 `fn random_level(max_level: usize, m: usize) -> usize`：按概率 1/m 的指数分布生成节点层数（20min）
- [ ] 24.2 实现 `fn insert(graph: &mut HNSWGraph, vector: Vec<f32>) -> usize`：确定随机层数 → 从当前最高层找到 entry → 逐层下降到目标层 → 在每层用 search_layer 找近邻 → 更新邻居列表（选择最近邻并截断到 m）（60min）
- [ ] 24.3 编写端到端测试：插入 50 个 32 维向量，随机选 5 个执行 greedy_search，验证 top-1 召回率 >= 80%（30min）
- [ ] 24.4 记录：插入 50 个向量的总耗时、图的层数分布、每层平均连接数（15min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| search_layer 单层搜索可用 | ☐ |
| greedy_search 层间搜索可用 | ☐ |
| insert 增量插入 + 端到端测试 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘
- 最大收获：
- 遇到的障碍（HNSW 插入是项目 2 最复杂的部分之一）：
- 明天重点（搜索优化 + 批量插入）：
