# Day 24 —— 项目5 启动：推荐系统架构 + 数据准备

> 目标：搭建推荐系统骨架，加载 MovieLens 数据集，完成探索性数据分析。
> 项目：`projects/05-recsys/src/`

---

## Task 60：推荐系统架构搭建

**产出物**：`projects/05-recsys/src/` 目录结构 + `projects/05-recsys/src/models/` 领域模型

- [ ] 60.1 设计推荐系统分层架构：召回层（`src/recall/`）、排序层（`src/ranking/`）、重排层（`src/reranking/`），创建目录结构（20min）
- [ ] 60.2 定义领域模型：`User`（用户画像）、`Item`（物品属性）、`Rating`（交互记录）、`Recommendation`（推荐结果）（25min）
- [ ] 60.3 设计数据管道接口：`DataLoader`（加载原始数据）→ `FeaturePipeline`（特征工程）→ `ModelTrainer`（模型训练）（20min）
- [ ] 60.4 撰写 ADR 005 初稿：记录技术栈选型（PyTorch、特征工程方案、评估指标）（20min）

---

## Task 61：MovieLens 数据集加载 + 探索性分析

**产出物**：`data/movielens/` 目录 + `projects/05-recsys/eda/exploratory_analysis.py`

- [ ] 61.1 下载 MovieLens 1M 数据集，解压到 `data/movielens/` 目录（15min）
- [ ] 61.2 加载 `ratings.csv` 和 `movies.csv`，统计：用户数、电影数、评分数、评分时间跨度（20min）
- [ ] 61.3 分析评分分布：1-5 分各有多少，平均评分是多少，是否存在评分偏差（20min）
- [ ] 61.4 计算数据稀疏性：实际评分数 / (用户数 × 电影数)，理解稀疏矩阵的挑战（20min）
- [ ] 61.5 划分训练集（80%）和测试集（20%），按用户分组确保同一用户的评分不跨集合（25min）
- [ ] 61.6 生成探索性分析报告（可视化：评分分布直方图、热门电影排行、用户活跃度分布）（30min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| 推荐系统分层架构骨架 | ☐ |
| MovieLens 数据集加载 + EDA 报告 | ☐ |
| 训练/测试集划分 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘
- 最大收获：
- 遇到的障碍：
- 明天需要调整的：
