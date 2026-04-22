# Day 3 —— 项目1规则引擎 + 代码审查工具成型

> 目标：实现规则引擎和 3 条代码坏味道规则，CLI 输出 Markdown 报告，完成项目1阶段复盘。

---

## Task 9：设计规则引擎数据结构

**产出物**：`projects/01-code-reviewer/src/rules.rs`（Rule 结构体 + evaluate 函数）

- [ ] 9.1 定义 `struct Rule { id, severity, description, matcher }`（3min）
- [ ] 9.2 定义 `enum MatcherKind { AST, Pattern, Threshold }`（2min）
- [ ] 9.3 写 `fn evaluate(rule: &Rule, ast: &AstNode) -> Vec<Issue>` 接口（4min）
- [ ] 9.4 写 1 个集成测试：验证"函数过长"规则触发（1min）

## Task 10：实现 3 条代码坏味道规则

**产出物**：3 条规则代码 + 各自测试 + 集成测试输出

- [ ] 10.1 规则1：函数体超过 50 行 → Severity::HIGH（3min）
- [ ] 10.2 规则2：圈复杂度 > 10 → Severity::MEDIUM（3min）
- [ ] 10.3 规则3：if 嵌套深度 > 3 → Severity::MEDIUM（3min）
- [ ] 10.4 集成测试：用一个真实文件跑 3 条规则，验证输出（1min）

## Task 11：CLI 输出格式化为 Markdown 报告

**产出物**：`cargo run -- projects/01-code-reviewer` 输出结构化 Markdown 报告

- [ ] 11.1 在 `main.rs` 中实现 `--output markdown` 和 `--output json` 分支（3min）
- [ ] 11.2 写 `fn format_report(issues: &[Issue]) -> String` 格式化函数（4min）
- [ ] 11.3 用项目自己的源码跑一遍，生成 Markdown 报告到 stdout，截图保存（3min）

## Task 12：项目1阶段复盘

**产出物**：`docs/notes/01-code-reviewer-retro.md`（≤ 300 字）+ 更新的 ADR

- [ ] 12.1 列出已完成的功能（AST/圈复杂度/规则引擎/CLI/报告）（2min）
- [ ] 12.2 列出还没做的（代码相似度检测/Java支持/误报率优化）（2min）
- [ ] 12.3 写下今天最大的"啊哈时刻"（1min）
- [ ] 12.4 更新 `docs/adr/001-code-reviewer.md` 填入决策内容（5min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| rules.rs 规则引擎可用 | ☐ |
| 3 条坏味道规则 + 测试通过 | ☐ |
| CLI 可输出 Markdown 报告 | ☐ |
| 阶段复盘笔记 + ADR 更新 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘（5min）
- 最大收获：
- 遇到的障碍：
- 明天需要调整的：
