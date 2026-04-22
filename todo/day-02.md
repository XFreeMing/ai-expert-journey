# Day 2 —— AST 解析基础

> 目标：完成 tree-sitter 调研和 AST 解析最小原型，实现圈复杂度计算。
> 项目：`projects/01-code-reviewer/`

---

## Task 4：调研 tree-sitter Rust 生态

**产出物**：`docs/notes/tree-sitter-ecosystem-research.md`（对比表 + 技术选型结论）

- [ ] 4.1 查阅 `tree-sitter` 的 crates.io 页面和文档，列出核心 crate（tree-sitter、tree-sitter-language 等）及其 API 概览（30min）
- [ ] 4.2 阅读 `tree-sitter-python` 和 `tree-sitter-java` 的 README，记录各自支持的语法节点名和查询语法差异（30min）
- [ ] 4.3 编写对比表：tree-sitter vs syn vs pest，从多语言支持、AST 粒度、性能、维护活跃度四个维度评分（30min）
- [ ] 4.4 在笔记中写下项目 01 的技术选型结论和理由（15min）

## Task 5：编写 AST 解析最小原型

**产出物**：`projects/01-code-reviewer/src/ast.rs`（可运行的 parse 函数）+ 至少 2 个通过测试

- [ ] 5.1 在 `Cargo.toml` 中添加 `tree-sitter` 及相关 language crate 依赖，确认版本兼容（20min）
- [ ] 5.2 创建 `src/ast.rs`，定义 `AstNode` 结构体（包含 kind、source、children、range 等字段），在 `main.rs` 或 `lib.rs` 中注册模块（20min）
- [ ] 5.3 实现 `fn parse_python(source: &str) -> Result<AstNode, ParseError>`，使用 tree-sitter 解析 Python 源码并递归构建 AST 树（45min）
- [ ] 5.4 编写 2 个测试：解析 `def foo(): pass` 验证根节点类型和子节点数量；解析一个含 if/else 的函数验证嵌套结构（30min）

## Task 6：实现圈复杂度计算

**产出物**：`projects/01-code-reviewer/src/complexity.rs` + 至少 3 个通过测试

- [ ] 6.1 阅读圈复杂度定义（McCabe），理解决策点（if/elif/for/while/and/or/except/ternary）的计数规则（20min）
- [ ] 6.2 创建 `src/complexity.rs`，实现 `fn cyclomatic(ast: &AstNode) -> u32`，遍历 AST 树统计决策点（30min）
- [ ] 6.3 编写 3 个测试用例：空函数返回 1、含 if-else 的函数返回 2、含嵌套循环和多个条件分支的函数返回 4+（30min）
- [ ] 6.4 将圈复杂度结果附加到 `AstNode` 的 metadata 中，为后续规则引擎做准备（15min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| tree-sitter 调研笔记 + 选型结论 | ☐ |
| ast.rs 可解析 Python 代码 + 测试通过 | ☐ |
| complexity.rs 3 个测试通过 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘
- 最大收获：
- 遇到的障碍：
- 明天需要调整的：
