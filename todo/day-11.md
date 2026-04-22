# Day 11 —— 项目2 收尾

> 目标：基准测试、与 Faiss 性能对比、项目文档，完成项目 02。
> 项目：`projects/02-vector-engine/`

---

## Task 31：准备基准测试数据集

**产出物**：10K 条 128 维向量数据集 + ground truth

- [ ] 31.1 编写脚本生成 10,000 条 128 维随机浮点向量，保存为二进制文件（自定义格式或 JSON），记录文件大小（20min）
- [ ] 31.2 生成 100 个查询向量，保存为 `data/benchmark/queries.bin`（10min）
- [ ] 31.3 使用 brute_force 计算每个查询的真实 top-10 结果，作为 ground truth 保存（30min）
- [ ] 31.4 编写数据加载函数：从二进制文件读取向量和查询，避免重复解析开销（20min）

## Task 32：运行 HNSW 基准测试

**产出物**：`projects/02-vector-engine/benchmark/hnsw-results.json`

- [ ] 32.1 使用默认配置（m=16, ef_construction=200, ef_search=50）构建 10K 向量索引，记录构建时间和内存占用（RSS）（25min）
- [ ] 32.2 执行 100 个查询，测量 P50/P95/P99 延迟，计算召回率@10（对比 ground truth）（30min）
- [ ] 32.3 测试不同 ef_search 值下的延迟-召回率权衡曲线（复用 Day 9 的实验方法，但用更大数据集）（25min）
- [ ] 32.4 测试索引序列化 + mmap 加载的耗时，验证 mmap 加载比从头构建快一个数量级（15min）

## Task 33：对比 Faiss + 项目文档

**产出物**：`projects/02-vector-engine/benchmark/comparison.md` + `projects/02-vector-engine/README.md`

- [ ] 33.1 安装 `faiss-cpu`（`pip install faiss-cpu` 或使用系统包管理器），编写 Python 脚本用同一数据集构建 Faiss IndexFlatL2 和 IndexHNSW（30min）
- [ ] 33.2 记录 Faiss 的构建时间、查询延迟、召回率、内存占用，与 Rust 实现对比（25min）
- [ ] 33.3 编写对比报告：表格形式展示 Rust HNSW vs Faiss HNSW 的各项指标，分析差距原因（Faiss 的高度优化布局、SIMD、量化技术）（25min）
- [ ] 33.4 编写项目 02 README：项目简介、安装、使用示例（构建索引/搜索/mmap 加载）、架构说明（HNSW 数据结构 → 搜索算法 → SIMD 优化）、性能指标（20min）
- [ ] 33.5 更新 ADR 记录项目 02 的技术决策（HNSW 参数选择、SIMD 方案、序列化格式）（15min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| 基准测试数据集 + ground truth | ☐ |
| HNSW 4 项指标测试完成 | ☐ |
| Faiss 对比报告 + 项目 README | ☐ |
| 今日复盘 | ☐ |

### 今日复盘
- 项目 02 总完成度：
- 与 Faiss 的差距分析：
- 如果重做一次会改进的地方：
