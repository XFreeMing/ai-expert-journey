# Day 19 —— 项目4 启动：Agent 架构设计 + ReAct 模式

> 目标：完成项目4整体架构设计，学习并实现 ReAct Agent 核心循环。
> 项目：`projects/04-multi-agent/src/`

---

## Task 51：Agent 架构设计

**产出物**：`docs/notes/agent-architecture-design.md`（架构图 + 设计决策）+ `projects/04-multi-agent/src/models/` 领域模型

- [ ] 51.1 阅读项目4 README，理解业务场景和验收标准（20min）
- [ ] 51.2 设计 4-5 个 Agent 角色及其职责边界，画出角色关系图（30min）
- [ ] 51.3 定义 Agent 间消息格式：`class AgentMessage: sender, receiver, content, metadata`（20min）
- [ ] 51.4 设计 DAG 工作流拓扑：确定 Agent 执行顺序和依赖关系（20min）
- [ ] 51.5 撰写 ADR 004 初稿：记录 Agent 架构选型（LangGraph vs 自编排 vs CrewAI）（20min）

---

## Task 52：ReAct Agent 核心实现

**产出物**：`projects/04-multi-agent/src/agent/react_agent.py`

- [ ] 52.1 阅读 2-3 篇 ReAct 模式的实现教程，理解 Thought-Action-Observation 循环（45min）
- [ ] 52.2 实现 `ReActAgent` 基类：`__init__(name, system_prompt, tools, llm)`，定义 run(task) 接口（20min）
- [ ] 52.3 实现核心循环：Thought（推理）→ Action（选择工具）→ Observation（获取结果）→ 循环直到收敛（30min）
- [ ] 52.4 实现收敛条件：最大迭代次数、重复 Action 检测、Final Answer 模式识别（20min）
- [ ] 52.5 编写集成测试：给 Agent 一个需要多步推理的任务，验证循环正确执行（20min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| Agent 架构设计文档 + 角色定义 | ☐ |
| ADR 004 初稿 | ☐ |
| ReAct Agent 基类可用 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘
- 最大收获：
- 遇到的障碍：
- 明天需要调整的：
