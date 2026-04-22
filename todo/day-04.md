# Day 4 —— 多语言支持 + 报告生成

> 目标：扩展 Java 语言支持，完善报告生成，补充代码相似度检测。
> 项目：`projects/01-code-reviewer/`

---

## Task 10：Java 语言支持

**产出物**：项目 01 可在 Java 文件上运行审查并输出有效报告

- [ ] 10.1 阅读 `tree-sitter-java` 文档，梳理 Java 关键语法节点名（method_declaration、if_statement、for_statement、try_statement 等），与 Python 节点名做映射表（30min）
- [ ] 10.2 修改 `ast.rs` 中的 parse 函数，支持根据文件扩展名自动选择语言解析器（Python / Java），处理语言切换时的边界情况（30min）
- [ ] 10.3 将 5 条规则的 matcher 抽象为语言无关的接口，确保 Java AST 节点也能被正确匹配（如 method_declaration 对应 Python 的 function_definition）（45min）
- [ ] 10.4 用一个简单 Java 文件（含长函数、深嵌套）测试审查结果，验证规则触发和行号准确（20min）

## Task 11：代码相似度检测（编辑距离 + LCS）

**产出物**：`projects/01-code-reviewer/src/similarity.rs` + 规则集成

- [ ] 11.1 创建 `src/similarity.rs`，实现 `fn edit_distance(a: &str, b: &str) -> u32`（动态规划），编写 3 个测试（相同、单字符差异、完全不同）（30min）
- [ ] 11.2 实现 `fn longest_common_subsequence(a: &str, b: &str) -> u32`（动态规划），编写 2 个测试（有公共子序列、无公共子序列）（25min）
- [ ] 11.3 实现 `fn line_similarity(file1: &str, file2: &str) -> f64`：按行分割后计算 LCS 相似度，作为规则 6（重复代码检测）的基础（25min）
- [ ] 11.4 新增规则 6：同一项目中两段代码行级相似度 > 80% 触发 Severity::MEDIUM，编写测试验证（25min）

## Task 12：完善报告生成 + AST 克隆检测调研

**产出物**：改进版 Markdown 报告 + `docs/notes/code-clone-detection-research.md`

- [ ] 12.1 改进报告格式：添加文件级汇总（每个文件的问题数、平均圈复杂度）、项目级汇总（总问题数、按严重级别分布）、趋势建议（30min）
- [ ] 12.2 增加 `--output json` 分支，输出机器可读的 JSON 格式（供后续 CI 集成使用）（20min）
- [ ] 12.3 调研代码克隆检测的 3 种方法：文本级（行匹配）、AST 级（子树同构）、基于嵌入的（向量相似度），记录优缺点和适用场景（30min）
- [ ] 12.4 在笔记中写下项目 01 选择"文本 + AST 混合"方案的理由，以及未来可扩展到 AST 克隆检测的预留设计（15min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| Java 支持可用 + 规则适配 | ☐ |
| 相似度检测 + 规则 6 集成 | ☐ |
| 报告格式改进 + 克隆检测调研 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘
- 最大收获：
- 遇到的障碍：
- 明天需要调整的：
