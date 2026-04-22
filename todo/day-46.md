# Day 46 —— 缓冲日 + RLHF 补充

> 目标：如果 RLHF PPO 实验未完成，今天补齐。无 GPU 则读论文。

---

## Task 102：RLHF PPO 实验补齐

**产出物**：`projects/07-finetuning/outputs/rlhf-checkpoint/` 或 `docs/notes/rlhf-paper-notes.md`

- [ ] 90.1 如果有 GPU：加载 Reward Model，跑一轮简易 PPO 训练（20 个样本），保存 checkpoint（45min）
- [ ] 90.2 如果无 GPU：精读《Training Language Models to Follow Instructions with Human Feedback》，写笔记覆盖 PPO 核心公式和关键 trick（45min）

## Task 103：RLHF 结果验证 + 知识整理

**产出物**：`projects/07-finetuning/src/rlhf_verify.py` 或 `docs/notes/rlhf-paper-notes.md` 补充

- [ ] 91.1 有 GPU：用 PPO 模型对 5 条 prompt 做推理，对比基座/SFT 回答质量（30min）
- [ ] 91.2 无 GPU：整理 PPO vs DPO vs ORPO 的优缺点对比表，记录适用场景（30min）
- [ ] 91.3 更新项目 7 TODO 清单，标记已完成/待完成的里程碑（15min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| RLHF PPO 实验 / 论文笔记 | ☐ |
| 结果验证 / 知识整理 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘（5min）
- 最大收获：
- 遇到的障碍：
- 明天需要调整的：
