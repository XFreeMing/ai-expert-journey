# Day 2 —— 项目1 AST 核心

> 目标：完成 AST 解析最小原型和圈复杂度计算，写第一篇技术博客草稿。

---

## Task 5：调研 tree-sitter Rust 生态

**产出物**：`docs/notes/tree-sitter-ecosystem-research.md`（对比表）

- [ ] 5.1 搜索 `tree-sitter` crates.io 页面，列出 top 5 相关 crate（2min）
- [ ] 5.2 阅读 `tree-sitter` crate README，提取关键 API 列表（3min）
- [ ] 5.3 阅读 `tree-sitter-java` 和 `tree-sitter-python` 文档，记录语法名差异（3min）
- [ ] 5.4 写对比表：tree-sitter vs syn vs pest（决策矩阵）（2min）

## Task 6：编写 AST 解析最小原型

**产出物**：`projects/01-code-reviewer/src/ast.rs`（可运行的 parse 函数）+ 测试通过

- [ ] 6.1 在 `src/` 下创建 `ast.rs`，写 `mod ast;` 到 `lib.rs` 或 `main.rs`（1min）
- [ ] 6.2 写 `struct AstNode { kind: String, children: Vec<AstNode> }`（3min）
- [ ] 6.3 写 `fn parse(source: &str) -> Result<AstNode, Error>` 最小实现（4min）
- [ ] 6.4 写测试：解析 `fn foo() {}` 验证 AST 树结构正确（2min）

## Task 7：编写圈复杂度计算函数

**产出物**：`projects/01-code-reviewer/src/complexity.rs` + 3 个通过测试

- [ ] 7.1 理解圈复杂度直觉：决策点数 + 1，写 50 字解释到笔记（3min）
- [ ] 7.2 创建 `src/complexity.rs`，写 `fn cyclomatic(ast: &AstNode) -> u32`（4min）
- [ ] 7.3 写 3 个测试：空函数(1)、if-else(2)、嵌套循环(4+)（3min）

## Task 8：写项目1的第一篇技术博客草稿

**产出物**：`docs/architecture/01-blog-draft-ast.md`（≥ 400 字）

- [ ] 8.1 列博客大纲（背景、AST是什么、圈复杂度、为什么用Rust）（3min）
- [ ] 8.2 写"AST是什么"部分（用自己的话，不用 AI 生成）（4min）
- [ ] 8.3 写"圈复杂度的工程直觉"部分（3min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| tree-sitter 调研笔记完成 | ☐ |
| ast.rs 可解析简单代码 | ☐ |
| complexity.rs 3 个测试通过 | ☐ |
| 技术博客草稿 ≥ 400 字 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘（5min）
- 最大收获：
- 遇到的障碍：
- 明天需要调整的：
