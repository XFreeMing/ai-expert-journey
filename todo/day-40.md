# Day 40 —— 全量 SFT 训练：完整数据集训练 + 记录指标

> 目标：用 Day 35 的 3 组配比数据之一，完成一次完整的全参数 SFT 训练。

> **硬件需求**：需要 >= 24GB 显存的 GPU。如果没有 GPU，可用 Colab Pro (A100) 或 Hugging Face Spaces 替代。

---

## Task 90：完整数据集 SFT 训练

**产出物**：`projects/07-finetuning/src/full_sft.py`

- [ ] 90.1 选择一组配比数据（建议先用 A 组：通用 70% + 领域 30%），加载完整数据集（500 条）（10min）
- [ ] 90.2 配置全量 SFT 训练参数：learning_rate=2e-5, epochs=3, batch_size=4, max_seq_len=512（20min）
- [ ] 90.3 编写全量微调脚本（冻结模型大部分参数或仅训练最后几层，避免显存溢出）（30min）
- [ ] 90.4 启动训练，预估训练时间并设置 checkpoint 保存（每 500 steps 保存一次）（45min）
- [ ] 90.5 训练完成后保存完整模型到 `projects/07-finetuning/outputs/full_sft_v1/`（10min）

## Task 91：训练指标记录 + 初步分析

**产出物**：`projects/07-finetuning/outputs/full_sft_v1/metrics.json` + 分析文档

- [ ] 91.1 从训练日志中提取完整 metrics：每个 step 的 train_loss、eval_loss、learning_rate（20min）
- [ ] 91.2 绘制完整的 loss 曲线（train + eval），标注最佳 checkpoint 位置（20min）
- [ ] 91.3 计算训练前后在领域测试题上的准确率差异（至少 10 道题）（30min）
- [ ] 91.4 记录训练时长、峰值显存、最终模型大小（15min）
- [ ] 91.5 撰写全量 SFT 训练报告到 `docs/notes/full-sft-training-report.md`（20min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| 完整数据集 SFT 训练 | ☐ |
| Loss 曲线 + 指标提取 | ☐ |
| 全量 SFT 训练报告 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘
- 最大收获：
- 遇到的障碍：
- 明天需要调整的：
