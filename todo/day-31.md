# Day 31 —— GGUF 本地推理测试 + 云端 vs 本地对比

> 目标：完成 GGUF 格式的本地推理全流程测试，并与云端 API 做性能/质量对比。

---

## Task 73：GGUF 本地推理全流程测试

**产出物**：`projects/06-llm-inference/tests/test_gguf_inference.py`

- [ ] 73.1 编写 GGUF 模型加载器：解析 GGUF 文件头，读取元数据（模型名、架构、token 列表）（30min）
- [ ] 73.2 实现文本生成 pipeline：tokenizer encode → 推理循环 → tokenizer decode，支持 temperature/top_p 参数（45min）
- [ ] 73.3 编写 5 个端到端测试用例：简单问答、代码生成、多轮对话、长文本摘要、中文理解（30min）
- [ ] 73.4 添加性能测量：首 token 延迟（TTFT）、总生成时间、tokens/sec、峰值内存（20min）
- [ ] 73.5 保存推理结果到 `projects/06-llm-inference/benchmark/gguf_inference_results.json`（15min）

## Task 74：云端 API vs 本地推理对比

**产出物**：`docs/notes/cloud-vs-local-inference.md`

- [ ] 74.1 选取同一组 5 个测试 prompt，分别调用云端 API（OpenAI/Anthropic）和本地 GGUF 模型（30min）
- [ ] 74.2 对比维度：延迟（TTFT、总时间）、吞吐量、回答质量（主观评分 1-5）、成本（每千 token）（30min）
- [ ] 74.3 分析本地推理的优势场景（隐私、离线、成本可控）和劣势场景（复杂推理、多语言）（20min）
- [ ] 74.4 绘制对比表格和折线图（可用 matplotlib 生成），标注 trade-off 结论（20min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| GGUF 本地推理测试用例 | ☐ |
| 云端 vs 本地对比报告 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘
- 最大收获：
- 遇到的障碍：
- 明天需要调整的：
