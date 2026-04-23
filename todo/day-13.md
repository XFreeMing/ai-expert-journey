# Day 13 —— 项目3 启动：复杂文档解析

> 目标：实现 PDF 和 Markdown 解析器，统一 Document 数据模型。
> 项目：`projects/03-rag-system/src/`

---

## Task 37：Document 数据模型 + PDF 解析器

**产出物**：`projects/03-rag-system/src/models.py` + `src/parsers/pdf.py`

- [ ] 37.1 定义统一 Document 模型：`class Document` 包含 text（完整文本）、metadata（来源、页数、作者等）、sections（层级结构列表，每项含 title、level、text、page_range）（30min）
- [ ] 37.2 在项目根目录执行 `uv add pymupdf`，阅读其基础 API 文档（页面提取、文本提取、表格提取）（20min）
- [ ] 37.3 实现 `class PDFParser: def parse(file_path: Path) -> Document`，提取全文文本、按页分割、提取页面级 metadata（40min）
- [ ] 37.4 编写测试：用一个真实 PDF 文件（至少 3 页），验证 text 非空、sections 按页面正确分割、metadata 包含页数信息（20min）

## Task 38：Markdown 解析器

**产出物**：`projects/03-rag-system/src/parsers/markdown.py` + 测试

- [ ] 38.1 实现 `class MarkdownParser: def parse(file_path: Path) -> Document`，使用正则表达式或 `markdown` 库解析 Markdown 文件（30min）
- [ ] 38.2 解析 H1/H2/H3/H4 标题为 section 层级结构，提取每个 section 的 title、level、text 内容（30min）
- [ ] 38.3 特殊处理：将代码块（``` ... ```）提取为 `ChunkType.CODE` 类型，将列表和表格标记为特殊 section（20min）
- [ ] 38.4 编写测试：用一个包含多级标题、代码块、列表的 Markdown 文件，验证 section 层级和内容提取正确（20min）

## Task 39：解析器工厂 + 统一接口

**产出物**：`projects/03-rag-system/src/parsers/factory.py` + 端到端测试

- [ ] 39.1 定义 `ParserProtocol`（Protocol 或 ABC）：`def parse(file_path: Path) -> Document` 抽象接口（15min）
- [ ] 39.2 实现 `ParserFactory`：根据文件扩展名（.pdf / .md / .docx）自动选择对应解析器，未知格式抛出清晰的 `UnsupportedFormatError`（25min）
- [ ] 39.3 编写端到端测试：用 PDF 和 Markdown 各一个文件，通过 factory 解析，验证都返回有效的 Document 对象（20min）
- [ ] 39.4 记录两种格式的解析差异：PDF 缺少结构化标题信息、Markdown 保留了层级，为后续分块策略的选择做铺垫（15min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| Document 模型 + PDF 解析器 + 测试 | ☐ |
| Markdown 解析器 + 测试 | ☐ |
| 解析器工厂 + 端到端验证 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘
- 最大收获：
- PDF vs Markdown 解析质量对比：
- 明天重点（分块策略）：
