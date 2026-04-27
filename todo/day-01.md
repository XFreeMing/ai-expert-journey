# Day 1 —— 环境验证 + 项目1热身

> 目标：确保开发环境可用，跑通 Rust 项目 01 的编译和测试，建立学习基线。
> 项目：`projects/01-code-reviewer/`

---

## Task 1：验证开发环境可用

**产出物**：`docs/notes/day1-env-check.md`（环境验证报告，含版本号和关键依赖清单）

- [ ] 1.1 运行 `rustup show`，确认 Rust 工具链版本 >= 1.75，记录 edition（15min）
- [ ] 1.2 运行 `uv python list`（或 `python3 --version`），确认 >= 3.11；运行 `source scripts/setup.sh` 创建 venv 并同步依赖（30min）
- [ ] 1.3 复制 `.env.example` 为 `.env`，填入真实 API key；验证 `shared/config.py` 能正确读取环境变量（20min）
- [ ] 1.4 将环境信息整理为 Markdown 报告，记录 OS、工具链版本、依赖安装中遇到的问题及解决方式（15min）

## Task 2：跑通项目 01 编译与 lint

**产出物**：`cargo build --release` 成功生成的二进制文件 + lint 通过的终端输出

- [ ] 2.1 `cd projects/01-code-reviewer`，阅读 `Cargo.toml` 理解依赖结构和 bin/lib 配置（20min）
- [ ] 2.2 执行 `cargo check`，如有编译错误，逐一排查并修复，记录错误清单和解决方案（30min）
- [ ] 2.3 执行 `cargo fmt --check && cargo clippy -- -D warnings`，修复所有 lint 告警（20min）
- [ ] 2.4 执行 `cargo build --release` 生成二进制，记录产物路径和文件大小（10min）

## Task 3：跑通项目 01 测试 + 需求拆解笔记

**产出物**：`cargo test` 通过输出 + `docs/notes/01-code-reviewer-需求拆解.md`

- [ ] 3.1 阅读 `tests/` 目录下的现有测试，理解项目测试结构；执行 `cargo test` 记录结果（20min）
- [ ] 3.2 如果测试不足，编写至少 2 个有意义的单元测试（如验证配置加载、基础字符串处理等）（30min）
- [ ] 3.3 重读项目 01 的 README，提取核心技术点（tree-sitter、规则引擎、多语言、报告生成），对每个标注"已知/未知/待验证"（30min）
- [ ] 3.4 整理为 Markdown 笔记，不超过 500 字（20min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| 环境验证报告完成 | ☐ |
| 项目 01 可编译 + lint 通过 | ☐ |
| 测试通过（≥2个有意义的测试） | ☐ |
| 需求拆解笔记完成 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘
- 最大收获：
- 遇到的障碍：
- 明天需要调整的：
