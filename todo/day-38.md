# Day 38 —— 训练监控：Loss 曲线分析 + 学习率调度 + 梯度累积

> 目标：深入理解训练过程中的监控指标，掌握学习率调度和梯度累积技术。

> **硬件需求**：需要 >= 24GB 显存的 GPU。如果没有 GPU，可用 Colab Pro (A100) 或 Hugging Face Spaces 替代。

---

## Task 86：训练监控 + Loss 曲线分析

**产出物**：`projects/07-finetuning/analysis/loss_curves_analysis.md`

- [ ] 86.1 用 Day 37 的训练日志绘制 train loss 和 eval loss 曲线（matplotlib），标注过拟合/欠拟合信号（30min）
- [ ] 86.2 分析 loss 下降趋势：前 20% steps 的快速下降期、中间平稳期、末尾收敛期（20min）
- [ ] 86.3 尝试不同学习率（1e-4、2e-4、5e-4），跑 3 组 2-epoch 短训练，对比 loss 曲线（45min）
- [ ] 86.4 观察验证集 loss：判断哪组学习率效果最佳，是否存在过拟合迹象（15min）

## Task 87：学习率调度 + 梯度累积实验

**产出物**：`projects/07-finetuning/src/lr_scheduler_experiment.py`

- [ ] 87.1 实现 3 种学习率调度策略：cosine、linear、constant（20min）
- [ ] 87.2 对比不同调度策略下的 loss 曲线和最终效果，记录差异（30min）
- [ ] 87.3 实验梯度累积：batch_size=1 + accumulation=8 vs batch_size=4 + accumulation=2，对比显存占用和训练速度（30min）
- [ ] 87.4 分析：为什么大模型微调常用小 batch_size + 大 gradient_accumulation_steps？有效 batch_size 对收敛的影响（15min）
- [ ] 87.5 汇总训练最佳实践文档到 `projects/07-finetuning/docs/training-best-practices.md`（20min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| Loss 曲线分析 + 学习率对比 | ☐ |
| 学习率调度策略实验 | ☐ |
| 梯度累积实验 + 分析 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘
- 最大收获：
- 遇到的障碍：
- 明天需要调整的：
