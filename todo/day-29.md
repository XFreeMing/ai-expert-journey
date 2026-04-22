# Day 29 —— 项目6 启动：vLLM 部署 + 推理基准测试

> 目标：部署 vLLM 推理服务，建立推理性能基线。
> 项目：`projects/06-llm-inference/`

---

## Task 69：vLLM 服务部署

**产出物**：vLLM 服务在 localhost:8004 运行 + `projects/06-llm-inference/benchmark/vllm_setup.json`

- [ ] 69.1 阅读 vLLM 文档，理解启动参数：`--model`、`--tensor-parallel-size`、`--max-model-len`、`--enforce-eager`（30min）
- [ ] 69.2 使用 `docker compose up vllm-server` 或手动部署 vLLM，加载 Qwen2.5-7B-Instruct 模型（30min）
- [ ] 69.3 验证服务健康：`curl http://localhost:8004/health`，确认模型加载成功（10min）
- [ ] 69.4 用 OpenAI 兼容 API 发一个测试请求，验证推理输出合理（15min）
- [ ] 69.5 记录模型加载时间（从启动到可接受请求）、模型显存占用量（nvidia-smi 或 docker stats）（15min）

---

## Task 70：推理基准测试

**产出物**：`projects/06-llm-inference/benchmark/baseline_results.json`

- [ ] 70.1 设计 5 个测试 prompt：短问答（50字输入）、长文档理解（2000字输入）、代码生成、数学推理、创意写作（20min）
- [ ] 70.2 编写基准测试脚本：对每个 prompt 发送请求，自动记录首 Token 延迟（TTFT）、总生成时间、Token 吞吐量（tokens/s）（30min）
- [ ] 70.3 跑完整基准测试（每个 prompt 重复 3 次取中位数），收集数据（30min）
- [ ] 70.4 计算统计指标：P50/P95/P99 首 Token 延迟，平均 Token 吞吐（20min）
- [ ] 70.5 分析瓶颈：首 Token 延迟中 KV Cache 分配占比、生成阶段 GPU 利用率（20min）
- [ ] 70.6 记录结论：当前配置下是否满足"首 Token < 500ms"、"Token 吞吐 > 50 tokens/s"的目标（10min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| vLLM 服务部署 + 健康检查通过 | ☐ |
| 5 个测试 prompt 基准数据（含 P50/P95/P99） | ☐ |
| 瓶颈分析报告 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘
- 最大收获：
- 遇到的障碍：
- 明天需要调整的：
