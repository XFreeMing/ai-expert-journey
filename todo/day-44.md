# Day 44 —— 奖励模型训练：偏好数据集构建 + RM 训练

> 目标：构建偏好数据集并训练奖励模型（Reward Model）。

> **硬件需求**：需要 >= 24GB 显存的 GPU。如果没有 GPU，可用 Colab Pro (A100) 或 Hugging Face Spaces 替代。

---

## Task 98：偏好数据集构建

**产出物**：`data/finetuning/preference_data.jsonl`

- [ ] 98.1 确定偏好数据格式：每条包含 {prompt, chosen_response, rejected_response}（15min）
- [ ] 98.2 生成偏好数据（至少 200 条）：
  - 用基座模型 + SFT 模型分别生成答案，人工或 LLM-as-judge 标注优劣（40min）
  - 从公开数据集（HH-RLHF、UltraFeedback）中筛选领域相关数据（20min）
- [ ] 98.3 数据质量控制：检查 chosen 确实优于 rejected，标注不一致的样本（15min）
- [ ] 98.4 划分训练集/验证集（80/20），保存为 JSONL（10min）

## Task 99：奖励模型训练

**产出物**：`projects/07-finetuning/src/reward_model.py` + 训练日志

- [ ] 99.1 选择奖励模型架构：用同一基座模型加一个分类头，或使用专门的 Reward Model 训练框架（25min）
- [ ] 99.2 编写奖励模型训练脚本：加载偏好数据，训练二元分类器（chosen vs rejected）（40min）
- [ ] 99.3 运行训练，监控 RM 的准确率（预测 chosen 正确的比例）（25min）
- [ ] 99.4 评估 RM 的泛化能力：用一组未见过的 prompt-reponse 对测试，记录排名准确率（20min）
- [ ] 99.5 保存奖励模型到 `projects/07-finetuning/outputs/reward_model_v1/`，撰写训练报告（10min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| 偏好数据集（200 条） | ☐ |
| 奖励模型训练 + 评估 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘
- 最大收获：
- 遇到的障碍：
- 明天需要调整的：
