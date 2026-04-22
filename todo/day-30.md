# Day 30 —— KV Cache 深度理解 + 模型量化实验

> 目标：深入理解 KV Cache 机制，完成多级别模型量化实验，对比精度与速度。
> 项目：`projects/06-llm-inference/`

---

## Task 71：KV Cache 原理 + 内存分析

**产出物**：`docs/notes/kv-cache-deep-dive.md`（原理笔记 + 计算）

- [ ] 71.1 阅读 vLLM PagedAttention 论文/博客，理解 KV Cache 内存碎片问题和分页解决思路（45min）
- [ ] 71.2 推导 KV Cache 内存占用公式：`2 × num_layers × num_heads × head_dim × seq_len × batch_size × precision_bytes`（30min）
- [ ] 71.3 代入 Qwen2.5-7B 参数：32 层、28 heads、128 head_dim，计算 seq_len=4096、batch_size=1 时的 KV Cache 大小（20min）
- [ ] 71.4 画 ASCII 图或 Mermaid 图：无 Cache 时自回归生成每步重复计算 vs 有 Cache 时增量计算（20min）
- [ ] 71.5 思考：当 batch_size 增大到 32 时，KV Cache 占总显存的比例是多少？这解释了什么问题？（15min）

---

## Task 72：模型量化实验

**产出物**：`projects/06-llm-inference/benchmark/quantization_results.json` + `docs/notes/quantization-comparison.md`

- [ ] 72.1 了解 GGUF 量化方案：Q4_0、Q4_K_M、Q5_K_M、Q8_0 的区别，量化对权重精度和推理速度的影响（30min）
- [ ] 72.2 使用 llama.cpp 或 HuggingFace transformers 的 bitsandbytes，将 FP16 模型量化为至少 3 种精度（Q4_K_M、Q5_K_M、Q8_0）（30min）
- [ ] 72.3 对每个量化版本，用 Day 29 的 5 个测试 prompt 跑基准测试，记录延迟和吞吐（45min）
- [ ] 72.4 人工评估回答质量：对比 FP16 原版和各量化版本的输出，标注"质量无差异"、"轻微退化"、"明显退化"的场景（30min）
- [ ] 72.5 生成量化对比报告：精度损失 vs 速度提升 vs 显存节省的 trade-off 表（20min）
- [ ] 72.6 撰写分析：什么场景下 INT4 够用（闲聊、简单问答），什么场景必须 FP16（代码生成、数学推理、专业领域）（20min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| KV Cache 原理笔记 + 内存计算 | ☐ |
| 3+ 种量化方案性能数据 | ☐ |
| 量化对比报告 + 质量分析 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘
- 最大收获：
- 遇到的障碍：
- 明天需要调整的：
