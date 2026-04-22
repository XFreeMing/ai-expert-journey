# Day 43 —— RLHF 入门：InstructGPT 论文 + 奖励函数设计

> 目标：理解 RLHF 完整流程，设计适合目标场景的奖励函数。

> **硬件需求**：需要 >= 24GB 显存的 GPU。如果没有 GPU，可用 Colab Pro (A100) 或 Hugging Face Spaces 替代。

---

## Task 96：InstructGPT 论文精读

**产出物**：`docs/notes/instructgpt-paper-annotated.md`

- [ ] 96.1 精读 InstructGPT 论文（Ouyang et al., 2022），标注 RLHF 三阶段流程：SFT → Reward Model → PPO（40min）
- [ ] 96.2 理解每个阶段的目标：SFT 学指令跟随、RM 学人类偏好、PPO 用 RM 信号优化策略（25min）
- [ ] 96.3 分析 PPO 训练中的关键超参数：KL penalty、value function loss coefficient、PPO clip range 的作用（20min）
- [ ] 96.4 对比 InstructGPT 与其他 RLHF 实现（DPO、ORPO、SimPO），理解免 PPO 的替代方案（20min）

## Task 97：奖励函数设计

**产出物**：`docs/adr/reward-function-design.md`

- [ ] 97.1 确定奖励信号维度：有用性（helpfulness）、真实性（truthfulness）、无害性（harmlessness）（20min）
- [ ] 97.2 设计奖励函数方案：规则-based（关键词匹配）vs 模型-based（训练 Reward Model）（25min）
- [ ] 97.3 分析 DPO（Direct Preference Optimization）方案：直接用偏好数据训练，省去 PPO 的复杂度和不稳定（20min）
- [ ] 97.4 确定本项目采用的 RLHF 路径：选择 PPO 或 DPO 并说明理由（15min）
- [ ] 97.5 撰写奖励函数设计文档，含公式推导、训练流程、评估标准（15min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| InstructGPT 论文精读 | ☐ |
| RLHF 方案选型（PPO vs DPO） | ☐ |
| 奖励函数设计文档 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘
- 最大收获：
- 遇到的障碍：
- 明天需要调整的：
