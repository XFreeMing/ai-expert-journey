# Day 3 —— 规则引擎

> 目标：实现规则引擎数据结构和 5 条审查规则，CLI 可运行并输出结果。
> 项目：`projects/01-code-reviewer/`

---

## Task 7：设计规则引擎数据结构

**产出物**：`projects/01-code-reviewer/src/rules.rs`（Rule 结构体 + RuleEngine + evaluate 函数）+ 集成测试

- [ ] 7.1 定义 `struct Rule`：包含 id、name、severity（枚举：INFO/MEDIUM/HIGH/CRITICAL）、description、language（支持的语言列表）、matcher_fn（函数指针或 trait 对象）（30min）
- [ ] 7.2 定义 `struct Issue`（如 shared models 中无，则在此定义）：包含 rule_id、file_path、line_number、severity、message、suggestion（20min）
- [ ] 7.3 实现 `struct RuleEngine`：支持 `register(rule: Rule)`、`evaluate(ast: &AstNode, source: &str) -> Vec<Issue>`（30min）
- [ ] 7.4 编写 1 个集成测试：注册一条规则，对一个简单 AST 运行 evaluate，验证 Issue 被正确触发（20min）

## Task 8：实现 5 条代码坏味道规则

**产出物**：5 条规则各自的实现代码 + 各至少 1 个测试

- [ ] 8.1 规则1 — 函数过长：函数体超过 50 行触发 Severity::HIGH。遍历 AST 中函数定义节点，计算 body 行号跨度（30min）
- [ ] 8.2 规则2 — 圈复杂度过高：圈复杂度 > 10 触发 Severity::MEDIUM。复用 Day 2 的 cyclomatic 函数（20min）
- [ ] 8.3 规则3 — 嵌套过深：if/for/while 嵌套深度 > 3 触发 Severity::MEDIUM。递归遍历 AST 跟踪当前深度（30min）
- [ ] 8.4 规则4 — 参数过多：函数参数 > 5 个触发 Severity::MEDIUM。检查函数定义的参数列表节点（15min）
- [ ] 8.5 规则5 — 空 except 块：捕获异常后无任何操作触发 Severity::HIGH。检查 except 子节点是否为 pass 或空（20min）
- [ ] 8.6 为每条规则编写至少 1 个单元测试，用内联 Python 代码字符串作为测试输入（30min）

## Task 9：CLI 集成 + Markdown 报告输出

**产出物**：`cargo run -- <目录>` 可对目标目录运行审查并输出结构化 Markdown 报告

- [ ] 9.1 在 `main.rs` 中实现 CLI 参数解析：支持 `<target_path>`、`--output markdown|json`、`--rules <rule_ids>`（30min）
- [ ] 9.2 实现文件发现逻辑：递归扫描目标目录下 `.py`（和 `.java`）文件（20min）
- [ ] 9.3 实现 `fn format_markdown_report(issues: &[Issue], file_path: &str) -> String`，按严重级别分组输出（30min）
- [ ] 9.4 用项目自己的 `src/` 目录跑一遍审查，保存 Markdown 报告到 `projects/01-code-reviewer/reports/self-review.md`（15min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| rules.rs 规则引擎可用 + 集成测试通过 | ☐ |
| 5 条坏味道规则 + 各自测试通过 | ☐ |
| CLI 可输出 Markdown 审查报告 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘
- 最大收获：
- 遇到的障碍：
- 明天需要调整的：
