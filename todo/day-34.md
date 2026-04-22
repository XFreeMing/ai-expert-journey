# Day 34 —— 项目7 启动：选择微调场景 + 构建 SFT 数据集

> 目标：确定微调目标场景，完成第一批 SFT 数据集构建。

> **硬件需求**：需要 >= 24GB 显存的 GPU。如果没有 GPU，可用 Colab Pro (A100) 或 Hugging Face Spaces 替代。

---

## Task 78：选择微调场景 + 技术方案

**产出物**：`projects/07-finetuning/docs/microtask-scenario.md`

- [ ] 78.1 确定微调目标场景：领域问答（如法律/医疗/代码）、指令跟随、文本生成、工具调用（30min）
- [ ] 78.2 选择基座模型：对比 Qwen2.5-7B、Llama-3-8B、Mistral-7B 在目标场景的适用性（30min）
- [ ] 78.3 撰写微调技术方案文档：场景描述、基座模型选择理由、评估指标、预期效果（30min）
- [ ] 78.4 确认训练环境：GPU 类型、显存容量、框架版本（PEFT + transformers 或 Unsloth）（15min）

## Task 79：构建 SFT 数据集（第一批）

**产出物**：`data/finetuning/sft_train_v1.jsonl`

- [ ] 79.1 设计数据格式：采用 alpaca 格式或 chatml 格式，确定 instruction/input/output 结构（20min）
- [ ] 79.2 收集/生成 500 条高质量指令数据：从公开数据集（Alpaca, OpenOrca, Dolly）筛选 + 自定义 prompt 生成（45min）
- [ ] 89.3 数据清洗：去重、过滤低质量样本、检查 token 长度分布（20min）
- [ ] 79.4 划分训练集/验证集（90/10），保存为 JSONL 格式，每行一条 JSON 记录（15min）
- [ ] 79.5 编写数据验证脚本：检查格式合法性、空字段、token 长度统计（20min）
- [ ] 79.6 保存数据集统计信息到 `data/finetuning/sft_train_v1_stats.json`（10min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| 微调场景确定 + 技术方案 | ☐ |
| 500 条 SFT 数据集 v1 | ☐ |
| 数据验证脚本 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘
- 最大收获：
- 遇到的障碍：
- 明天需要调整的：
