# Day 36 —— LoRA 原理深入学习 + 环境准备

> 目标：掌握 LoRA 核心原理，完成微调框架安装与环境验证。

> **硬件需求**：需要 >= 24GB 显存的 GPU。如果没有 GPU，可用 Colab Pro (A100) 或 Hugging Face Spaces 替代。

---

## Task 82：LoRA 原理论文精读

**产出物**：`docs/notes/lora-paper-annotated.md`

- [ ] 82.1 精读 LoRA 论文（Hu et al., 2021），标注关键公式：低秩分解 W = W₀ + BA 的推导（30min）
- [ ] 82.2 理解秩 r 的选取逻辑：为什么 r=8 或 r=16 在大多数场景够用？秩与效果的关系（20min）
- [ ] 82.3 分析 LoRA 的训练 vs 推理开销：为什么 LoRA 不增加推理延迟？参数合并机制（20min）
- [ ] 82.4 绘制 LoRA 训练架构图：哪些层被冻结、哪些层插入低秩适配器、梯度如何流动（20min）
- [ ] 82.5 对比全量微调 vs LoRA 的参数比例：7B 模型全量 ~14GB，LoRA (r=16) ~0.05% 参数（15min）

## Task 83：微调环境准备 + 框架选型

**产出物**：`projects/07-finetuning/pyproject.toml` 依赖确认 + 环境验证脚本

- [ ] 83.1 确定框架栈：PEFT + transformers + accelerate（基础路线）或 Unsloth（加速路线），对比优劣（20min）
- [ ] 83.2 确认项目 07 `pyproject.toml` 中已声明 PEFT、transformers、accelerate、bitsandbytes、trl 依赖（15min）
- [ ] 83.3 安装环境，验证 GPU 可达性、CUDA 版本、显存容量（15min）
- [ ] 83.4 编写环境验证脚本：加载基座模型 + 打印参数信息 + 测试前向传播（20min）
- [ ] 83.5 下载基座模型到本地缓存，确认模型文件完整（15min）
- [ ] 83.6 记录环境配置到 `projects/07-finetuning/docs/environment-setup.md`（10min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| LoRA 论文精读笔记 | ☐ |
| 微调环境安装 + 验证 | ☐ |
| 框架选型文档 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘
- 最大收获：
- 遇到的障碍：
- 明天需要调整的：
