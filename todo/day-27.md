# Day 27 —— 项目5 收尾：在线服务 + 技术博客

> 目标：搭建 FastAPI 在线推荐服务，实现冷启动策略，撰写技术博客。
> 项目：`projects/05-recsys/src/`

---

## Task 66：在线推荐服务 + 冷启动

**产出物**：`projects/05-recsys/src/main.py`（端口 8003）+ `projects/05-recsys/src/cold_start.py`

- [ ] 66.1 实现 `POST /recommend/{user_id}` 端点：召回层(UserCF/ItemCF) 产生候选集 → 排序层(Wide&Deep) 打分排序 → 输出 top-20（30min）
- [ ] 66.2 实现 `POST /recommend/batch` 批量推荐端点：支持一次请求为多个用户推荐（20min）
- [ ] 66.3 实现冷启动策略：`ColdStartStrategy`，对 0 交互新用户返回热门物品 + 基于用户画像的内容推荐（25min）
- [ ] 66.4 实现 `GET /health` 端点，记录服务启动时间和模型加载状态（10min）
- [ ] 66.5 启动服务 `uvicorn src.main:app --port 8003`，测试新用户推荐、已有用户推荐、冷启动用户推荐（30min）
- [ ] 66.6 记录端到端推荐延迟（P50/P95），分析瓶颈在召回还是排序（15min）

---

## Task 67：项目5 技术博客 + 评估

**产出物**：`docs/architecture/05-blog-recsys.md`（≥ 1000 字）+ `docs/adr/005-recsys.md` 最终版

- [ ] 67.1 整理实验数据：UserCF vs ItemCF vs Wide&Deep 的 Recall/NDCG 对比，端到端延迟（25min）
- [ ] 67.2 绘制推荐系统架构图：数据流 + 召回-排序-重排漏斗 + 服务部署拓扑（30min）
- [ ] 67.3 撰写技术博客：推荐系统漏斗架构、协同过滤实践、Wide&Deep 实现细节、冷启动策略、性能调优（45min）
- [ ] 67.4 更新 ADR 005 最终版本，记录算法选型和架构决策（20min）
- [ ] 67.5 运行项目5完整测试套件，确保覆盖率 >= 80%（20min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| 在线推荐 API（单用户 + 批量）可用 | ☐ |
| 冷启动策略可用 | ☐ |
| 技术博客 ≥ 1000 字 + 架构图 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘
- 最大收获：
- 遇到的障碍：
- 明天需要调整的：
