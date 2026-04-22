# Day 25 —— 召回层：协同过滤（UserCF + ItemCF）

> 目标：实现 UserCF 和 ItemCF 两种经典协同过滤算法，构建推荐系统召回层。
> 项目：`projects/05-recsys/src/recall/`

---

## Task 62：UserCF 实现

**产出物**：`projects/05-recsys/src/recall/user_cf.py` + 单元测试

- [ ] 62.1 复习 UserCF 算法：构建用户-物品评分矩阵，计算用户间余弦相似度（30min）
- [ ] 62.2 实现 `UserCFRecommender` 类：`fit(ratings)` 构建评分矩阵和相似度矩阵（30min）
- [ ] 62.3 实现 `recommend(user_id, top_k)` 方法：找到最相似 N 个用户，聚合他们喜欢但目标用户未交互的物品（30min）
- [ ] 62.4 优化：使用稀疏矩阵（scipy.sparse）存储评分矩阵，避免 O(n²) 内存爆炸（25min）
- [ ] 62.5 测试：选取一个活跃用户，推荐 top-10 物品，人工验证推荐结果合理性（15min）

---

## Task 63：ItemCF 实现 + 对比

**产出物**：`projects/05-recsys/src/recall/item_cf.py` + 对比报告

- [ ] 63.1 复习 ItemCF 算法：构建物品-用户评分矩阵，计算物品间余弦相似度（30min）
- [ ] 63.2 实现 `ItemCFRecommender` 类：`fit(ratings)` 构建共现矩阵和相似度矩阵（30min）
- [ ] 63.3 实现 `recommend(user_id, top_k)` 方法：基于用户历史评分物品的相似物品推荐（25min）
- [ ] 63.4 对比实验：在同一测试集上跑 UserCF 和 ItemCF，计算 Recall@10、NDCG@10（30min）
- [ ] 63.5 分析差异：记录两种算法的计算耗时、推荐结果的多样性（覆盖的物品数量）（15min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| UserCF 召回器可用 | ☐ |
| ItemCF 召回器可用 | ☐ |
| Recall@10 / NDCG@10 对比数据 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘
- 最大收获：
- 遇到的障碍：
- 明天需要调整的：
