# Day 47 —— 项目7 最终收尾

> 目标：vLLM 部署微调模型，撰写技术博客，更新 ADR-007 + CI 配置。

> **硬件需求**：需要 >= 16GB 显存的 GPU。无 GPU 可用 Colab Pro 替代。

---

## Task 104：vLLM 部署微调模型

**产出物**：`projects/07-finetuning/deploy/` 目录完整（Dockerfile、启动脚本、验证脚本）

- [ ] 92.1 将最佳 LoRA adapter 合并到基座模型，导出 HF 格式到 `outputs/merged-model/`（20min）
- [ ] 92.2 编写 vLLM Dockerfile：多阶段构建，设置 MODEL_PATH 环境变量，暴露 8000 端口（20min）
- [ ] 92.3 编写部署脚本 `deploy/run_vllm.sh`：检查 GPU、拉取镜像、启动容器、健康检查（15min）
- [ ] 92.4 运行验证：发送 3 条测试 prompt（领域问答、通用对话、边界测试），记录响应质量和延迟（25min）
- [ ] 92.5 记录部署指标：模型加载时间、首 token 延迟、吞吐量（tokens/sec）、显存峰值（10min）

## Task 105：技术博客撰写

**产出物**：`docs/notes/sft-finetuning-blog.md`（技术博客正文 2000+ 字）

- [ ] 93.1 编写博客大纲：背景 → 数据准备 → LoRA 原理 → SFT 训练 → RLHF → 部署 → 总结（15min）
- [ ] 93.2 撰写引言和数据处理部分：领域选择、数据收集、清洗流程、Tokenization 策略（25min）
- [ ] 93.3 撰写训练和部署部分：LoRA 参数、超参调优、loss 曲线、vLLM 部署方案、性能指标（30min）
- [ ] 93.4 撰写总结和最佳实践：踩坑清单、推荐配置、适用场景（15min）

## Task 106：ADR-007 更新 + CI 配置

**产出物**：`docs/adr/007-domain-finetuning.md`（更新）+ `.gitlab-ci.yml` 补充项目 7 lint/test

- [ ] 94.1 更新 ADR-007：补充最终决策（基座模型、LoRA rank、训练框架、RLHF 是否纳入）（20min）
- [ ] 94.2 在 `.gitlab-ci.yml` 中添加项目 7 的 lint + 数据验证 + 模型导出检查阶段（20min）
- [ ] 94.3 整理项目 7 全部产出物清单，更新 `projects/07-finetuning/README.md`（10min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| vLLM 部署 + 验证通过 | ☐ |
| 技术博客正文 | ☐ |
| ADR-007 更新 + CI 配置 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘（5min）
- 最大收获：
- 遇到的障碍：
- 明天需要调整的：
