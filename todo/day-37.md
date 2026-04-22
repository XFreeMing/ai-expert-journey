# Day 37 —— LoRA 首次训练：100 条数据跑通微调全流程

> 目标：用少量数据验证整个微调 pipeline 可以正常运行，排除环境问题。

> **硬件需求**：需要 >= 24GB 显存的 GPU。如果没有 GPU，可用 Colab Pro (A100) 或 Hugging Face Spaces 替代。

---

## Task 84：100 条数据 + LoRA 微调脚本

**产出物**：`projects/07-finetuning/src/lora_finetune.py`

- [ ] 84.1 编写数据加载器：从 100 条 JSONL 数据构建 Dataset，实现 tokenizer 分片和 padding（30min）
- [ ] 84.2 配置 LoRA：设置 r=16, alpha=32, dropout=0.05，target_modules 为注意力层的 q_proj 和 v_proj（20min）
- [ ] 84.3 配置训练参数：learning_rate=2e-4, epochs=3, batch_size=4, gradient_accumulation_steps=4（15min）
- [ ] 84.4 编写训练脚本：使用 SFTTrainer 或自定义 TrainingArguments，整合模型、tokenizer、数据集、LoRA 配置（45min）
- [ ] 84.5 运行训练，确保 100 条数据能在 10-20 分钟内完成全部 epoch（30min）
- [ ] 84.6 保存训练日志到 `projects/07-finetuning/logs/first_run.log`（10min）

## Task 85：首次训练结果验证

**产出物**：`projects/07-finetuning/outputs/lora_first_run/`

- [ ] 85.1 加载训练好的 LoRA adapter，用 3 个测试 prompt 验证生成效果（20min）
- [ ] 85.2 对比基座模型（无微调）和 LoRA 微调后的输出差异，记录观察结果（15min）
- [ ] 85.3 检查 loss 曲线：确认 loss 在下降，没有梯度爆炸或 NaN（15min）
- [ ] 85.4 检查显存使用峰值，记录实际消耗并与理论值对比（10min）
- [ ] 85.5 撰写首次训练报告：是否跑通、loss 变化、生成质量、显存占用（15min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| LoRA 微调脚本编写 | ☐ |
| 100 条数据训练跑通 | ☐ |
| 首次训练结果验证报告 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘
- 最大收获：
- 遇到的障碍：
- 明天需要调整的：
