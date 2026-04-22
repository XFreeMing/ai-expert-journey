# Day 1 —— 环境验证 + 项目1热身

> 目标：确保开发环境可用，跑通 Rust 项目 01 的编译和测试，建立学习基线。

---

## Task 1：验证开发环境可用

**产出物**：环境验证报告（保存到 `todo/day1-env-check.md` 或截图）

- [ ] 1.1 `rustup show` 确认工具链版本（2min）
- [ ] 1.2 `python --version` 确认 >= 3.11（1min）
- [ ] 1.3 `source scripts/setup.sh` 创建 venv + 安装依赖（5min）
- [ ] 1.4 `cp .env.example .env` 并填入真实 API key（2min）

## Task 2：跑通 Rust 项目 01 编译

**产出物**：`cargo build --release` 通过的终端输出截图或日志

- [ ] 2.1 `cd projects/01-code-reviewer`（1min）
- [ ] 2.2 `cargo check` 看是否有编译错误，记录错误清单（2min）
- [ ] 2.3 修复编译报错（如有）（3min）
- [ ] 2.4 `cargo fmt --check && cargo clippy` 通过 lint（2min）
- [ ] 2.5 `cargo build --release` 生成二进制，记录产物路径（2min）

## Task 3：跑通 Rust 项目 01 测试

**产出物**：`cargo test` 通过的终端输出（至少 1 个测试）

- [ ] 3.1 阅读 `tests/mod.rs` 现有占位测试，理解结构（2min）
- [ ] 3.2 `cargo test` 运行测试，记录结果（2min）
- [ ] 3.3 如果测试为 0，写第一个空测试 `#[test] fn it_works()`（3min）
- [ ] 3.4 记录覆盖率基线数字（3min）

## Task 4：阅读 README 项目1需求并写笔记

**产出物**：`docs/notes/01-code-reviewer-需求拆解.md`（≤ 500 字）

- [ ] 4.1 重读 README 中项目1的核心算法/技术，标注 4 个技术点（5min）
- [ ] 4.2 对每个技术点写 2-3 句"我已知"和"我未知"（5min）
- [ ] 4.3 整理为 Markdown 笔记文件（3min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| 环境可用 | ☐ |
| 项目1可编译 | ☐ |
| 项目1测试通过（≥1个） | ☐ |
| 需求拆解笔记完成 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘（5min）
- 最大收获：
- 遇到的障碍：
- 明天需要调整的：
