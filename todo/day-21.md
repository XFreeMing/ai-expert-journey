# Day 21 —— 多 Agent 协调：任务分解 + Agent 间通信

> 目标：实现多 Agent 编排器，支持 DAG 工作流和 Agent 间消息传递。
> 项目：`projects/04-multi-agent/src/`

---

## Task 55：Agent 编排器

**产出物**：`projects/04-multi-agent/src/orchestrator.py`

- [ ] 55.1 阅读多 Agent 编排的常见模式（流水线、星型、图结构），理解适用场景（30min）
- [ ] 55.2 实现 `Orchestrator` 类：管理 Agent 实例、工作流拓扑、消息路由（25min）
- [ ] 55.3 实现 DAG 执行引擎：按拓扑顺序依次调用 Agent，前一个输出作为后一个上下文（30min）
- [ ] 55.4 实现共享上下文（`SharedContext`）：各 Agent 在统一黑板上读写中间结果（20min）
- [ ] 55.5 编写集成测试：用 3 个 Agent 跑一个任务拆解，验证 DAG 执行顺序正确（30min）

---

## Task 56：任务分解策略

**产出物**：`projects/04-multi-agent/src/orchestrator/task_decomposer.py`

- [ ] 56.1 研究任务分解模式：固定分解（预定义子任务）vs 动态分解（LLM 按需生成）（30min）
- [ ] 56.2 实现 `TaskDecomposer`：接收用户输入，解析并拆分为子任务列表（25min）
- [ ] 56.3 实现依赖分析：子任务间的先后关系（哪些可并行、哪些必须串行）（25min）
- [ ] 56.4 实现冲突检测：当 Agent 输出矛盾时触发仲裁（25min）
- [ ] 56.5 端到端测试：输入一个复杂任务，观察完整处理链（30min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| Orchestrator DAG 执行可用 | ☐ |
| 共享上下文 + 消息路由 | ☐ |
| 任务分解 + 冲突仲裁 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘
- 最大收获：
- 遇到的障碍：
- 明天需要调整的：
