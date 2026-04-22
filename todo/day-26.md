# Day 26 —— 排序层：特征工程 + Wide&Deep 模型

> 目标：完成特征工程管线，实现 Wide&Deep 排序模型并训练。
> 项目：`projects/05-recsys/src/`

---

## Task 64：特征工程管线

**产出物**：`projects/05-recsys/src/features/` 目录 + 特征编码器

- [ ] 64.1 阅读推荐系统特征工程实践：稠密特征 vs 稀疏特征、特征交叉的意义（30min）
- [ ] 64.2 定义用户特征：用户 ID（嵌入）、年龄分桶、性别、历史评分均值、活跃天数（20min）
- [ ] 64.3 定义物品特征：电影 ID（嵌入）、类型（多标签）、年代、平均评分、评分人数（20min）
- [ ] 64.4 定义交叉特征：用户类型偏好 × 电影类型、用户活跃度 × 电影热度（20min）
- [ ] 64.5 实现 `FeatureEncoder`：离散特征 one-hot / embedding 化，连续特征归一化，输出 PyTorch Tensor（30min）
- [ ] 64.6 测试：对 1000 条样本做特征工程，验证输出维度和值域合理（15min）

---

## Task 65：Wide&Deep 模型实现

**产出物**：`projects/05-recsys/src/ranking/wide_and_deep.py` + 训练脚本

- [ ] 65.1 阅读 Wide&Deep 论文，理解 Wide 部分（记忆特征交叉）和 Deep 部分（泛化表示学习）的协同原理（30min）
- [ ] 65.2 实现 Wide 部分：稀疏特征的线性组合，学习显式特征交叉（20min）
- [ ] 65.3 实现 Deep 部分：Embedding 层 → 多层 MLP（ReLU → BatchNorm → Dropout）（25min）
- [ ] 65.4 合并输出：`logits = wide + deep`，用 BCEWithLogitsLoss 训练，输出点击概率（20min）
- [ ] 65.5 训练 10-20 个 epoch，记录训练集和验证集的 loss/AUC 曲线（20min）
- [ ] 65.6 评估：在测试集上计算 Recall@50、NDCG@50，与 UserCF/ItemCF 对比（25min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| 特征工程管线可用 | ☐ |
| Wide&Deep 模型训练完成 | ☐ |
| Recall@50 / NDCG@50 vs 协同过滤对比 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘
- 最大收获：
- 遇到的障碍：
- 明天需要调整的：
