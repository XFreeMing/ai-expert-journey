# Day 20 —— Agent 工具调用：Function Calling 框架 + 工具实现

> 目标：完善 Agent 的工具调用能力，实现 3 个实用工具并接入 ReAct 循环。
> 项目：`projects/04-multi-agent/src/`

---

## Task 53：工具调用框架

**产出物**：`projects/04-multi-agent/src/agent/tools/` + `Tool` 抽象类 + 注册表

- [ ] 53.1 定义 `Tool` 抽象类：`name, description, parameters_schema, execute(**kwargs)` 接口（20min）
- [ ] 53.2 实现工具注册表 `ToolRegistry`：支持按名称查找、按标签过滤（20min）
- [ ] 53.3 阅读 OpenAI Function Calling 规范，理解 JSON Schema 格式的参数描述（25min）

---

## Task 54：3 个工具实现 + 集成测试

**产出物**：`projects/04-multi-agent/src/agent/tools/knowledge_search.py` + `compliance_checker.py` + `cost_estimator.py`

- [ ] 54.1 实现 `search_knowledge_base(query)` — 连接项目3的 RAG 检索器（25min）
- [ ] 54.2 实现 `check_compliance(requirement_text)` — 基于规则的合规检查（25min）
- [ ] 54.3 实现 `calculate_cost_estimate(specs)` — 简单的成本估算器（20min）
- [ ] 54.4 集成到 ReActAgent：让 Agent 使用工具链完成一个多步任务（30min）
- [ ] 54.5 编写测试：验证 Agent 能正确选择工具、传递参数、处理工具返回结果（20min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| Tool 抽象 + 注册表可用 | ☐ |
| 3 个工具实现可用 | ☐ |
| ReActAgent 工具链集成测试通过 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘
- 最大收获：
- 遇到的障碍：
- 明天需要调整的：
