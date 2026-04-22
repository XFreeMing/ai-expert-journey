# Day 6 —— 缓冲日 + 项目2预习

> 目标：处理 Day 1-5 的遗留问题，预习项目 2（HNSW 向量检索）的核心概念。
> 项目：`projects/01-code-reviewer/`（收尾）+ `projects/02-vector-engine/`（预习）

---

## Task 16：Day 1-5 遗留问题处理

**产出物**：待办修复清单 + 修复后的验证结果

- [ ] 16.1 回顾 Day 1-5 的完成度检查表，列出未完成或质量不达标的任务（20min）
- [ ] 16.2 按优先级修复：优先处理编译警告、测试覆盖不足、文档缺失等问题（45min）
- [ ] 16.3 对项目 01 运行完整的 `cargo test --all-features` 和 `cargo clippy -- -D warnings`，确保全部通过（20min）
- [ ] 16.4 如果所有任务已完成且质量达标，提前开始项目 2 的环境准备：创建 `projects/02-vector-engine/` 的 `Cargo.toml`，添加基础依赖（20min）

## Task 17：HNSW 论文学习 + 直觉建立

**产出物**：`docs/notes/hnsw-intuition.md`（学习笔记，含 ASCII 图解）

- [ ] 17.1 阅读 HNSW 论文（"Efficient and robust approximate nearest neighbor search using Hierarchical Navigable Small World graphs"）的摘要和引言部分，理解核心思想（45min）
- [ ] 17.2 重点理解"多层高速公路"直觉：为什么分层能加速搜索？每层的作用是什么？为什么高层稀疏、底层密集？用自己的话写 300 字解释（30min）
- [ ] 17.3 画 ASCII 图：一个 3 层 HNSW 结构，标注 entry point、每层节点、跨层连接（20min）
- [ ] 17.4 记录论文中的关键参数：m（每层最大连接数）、efConstruction（构建时搜索范围）、efSearch（查询时搜索范围），写下各自的物理意义（15min）

## Task 18：项目 02 环境搭建 + 项目预习

**产出物**：`projects/02-vector-engine/` 最小可编译骨架

- [ ] 18.1 初始化项目 02：`cargo init --name vector-engine projects/02-vector-engine`，配置 `Cargo.toml`（edition 2021、必要的 crate 依赖）（20min）
- [ ] 18.2 创建基础模块骨架：`src/hnsw.rs`（空模块）、`src/similarity.rs`（空模块）、`src/lib.rs`（模块声明）（15min）
- [ ] 18.3 预习项目 02 的 README，提取关键技术点（HNSW 索引构建、贪心搜索、SIMD 优化、内存映射），标注已知/未知（20min）
- [ ] 18.4 调研 Rust SIMD 生态：`std::simd`（nightly）vs `packed_simd` vs 手动 unrolling，记录选型建议到笔记（20min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| Day 1-5 遗留问题已处理 | ☐ |
| HNSW 论文学习笔记 + ASCII 图 | ☐ |
| 项目 02 环境搭建 + 预习完成 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘
- 项目 01 总完成度：
- 对项目 02 的担忧/期待：
- 明天重点：
