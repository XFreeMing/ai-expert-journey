# Day 39 —— QLoRA 实验：量化感知微调 + 显存对比

> 目标：使用 QLoRA（4-bit 量化 + LoRA）进行微调，对比标准 LoRA 和 QLoRA 的效果。

> **硬件需求**：需要 >= 24GB 显存的 GPU。如果没有 GPU，可用 Colab Pro (A100) 或 Hugging Face Spaces 替代。QLoRA 4-bit 量化可在 12GB GPU 上运行。

---

## Task 88：QLoRA 微调实验

**产出物**：`projects/07-finetuning/src/qlora_finetune.py`

- [ ] 88.1 配置 4-bit NF4 量化：使用 bitsandbytes 的 BitsAndBytesConfig，设置 load_in_4bit=True、bnb_4bit_quant_type="nf4"（25min）
- [ ] 88.2 在量化基座上叠加 LoRA adapter（同 Day 37 的配置），验证模型可正常前向传播（20min）
- [ ] 88.3 使用 100 条数据运行 QLoRA 微调训练，确保 loss 正常下降（45min）
- [ ] 88.4 保存训练日志和模型权重到 `projects/07-finetuning/outputs/qlora_run1/`（10min）

## Task 89：QLoRA vs LoRA 显存/效果对比

**产出物**：`docs/notes/qlora-vs-lora-comparison.md`

- [ ] 89.1 对比峰值显存占用：QLoRA（4-bit）vs 标准 LoRA（FP16），计算显存节省比例（20min）
- [ ] 89.2 对比训练速度（steps/sec）：量化引入的额外计算开销（15min）
- [ ] 89.3 用相同 5 个测试 prompt，对比 QLoRA 和标准 LoRA 的输出质量（主观评分 1-5）（30min）
- [ ] 89.4 分析：QLoRA 在什么场景下质量接近标准 LoRA？什么场景有明显差距？（20min）
- [ ] 89.5 撰写 QLoRA 实验报告：含显存对比表、速度对比、质量评估、推荐配置（20min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| QLoRA 微调脚本 + 训练 | ☐ |
| QLoRA vs LoRA 显存对比 | ☐ |
| QLoRA 实验报告 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘
- 最大收获：
- 遇到的障碍：
- 明天需要调整的：
