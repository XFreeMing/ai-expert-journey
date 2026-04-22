# Day 55 —— 领域微调

> 目标：LoRA 微调 ControlNet 到特定艺术风格。

> **硬件需求**：需要 >= 16GB 显存的 GPU。无 GPU 可用 Colab Pro 替代。训练耗时约 2-4 小时，训练期间可并行进行文档工作。

---

## Task 127：训练数据准备

**产出物**：`data/multimodal/training/`（微调训练数据集 + 配置文件）

- [ ] 115.1 确定目标风格（推荐选择：吉卜力动画风、水墨画风、赛博朋克风之一），收集 50-100 张代表性图片（45min）
- [ ] 115.2 为每张图片生成对应的条件图（Canny 或 Depth），使用 Day 51 的 condition_processors 工具（20min）
- [ ] 115.3 为每张图片编写简短 prompt 描述（可先用 VLM 自动生成初稿，再人工修正）（20min）
- [ ] 115.4 将数据组织为训练所需格式：`{image_path, condition_path, prompt}` 三元组的 JSON/CSV 清单（15min）

## Task 128：LoRA 训练脚本

**产出物**：`projects/08-multimodal/scripts/train_lora_controlnet.py` + `projects/08-multimodal/training/configs/lora_config.yaml`

- [ ] 116.1 搭建训练循环：加载预训练 SD + ControlNet，使用 `peft` 库注入 LoRA 层，冻结主干权重（30min）
- [ ] 116.2 配置训练参数：learning_rate=1e-4、batch_size=1、epochs=100、gradient_accumulation=4、lr_scheduler=cosine（15min）
- [ ] 116.3 添加验证环节：每 10 个 epoch 用固定 prompt + 条件图生成验证图，保存到 `data/multimodal/training/checkpoints/val_images/`，观察风格收敛（15min）
- [ ] 116.4 添加 checkpoint 保存和 LoRA 权重导出（safetensors 格式），添加 TensorBoard logging（15min）

## Task 129：微调效果评估

**产出物**：`docs/notes/day55-lora-finetune-report.md`（LoRA 微调实验报告）

- [ ] 117.1 加载训练好的 LoRA 权重，与原始 ControlNet 在同一 prompt + 条件下对比生成效果（20min）
- [ ] 117.2 评估风格一致性：用 5 张未见过的条件图生成新图像，主观评分（1-5 分）（15min）
- [ ] 117.3 分析过拟合风险：对比训练集图片和生成图片的相似度，检查是否出现 memorization（15min）
- [ ] 117.4 记录训练耗时、最终 loss 曲线、LoRA 权重大小，撰写实验总结（15min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| 训练数据集（50-100 张 + 条件图） | ☐ |
| LoRA 训练脚本 + 训练启动 | ☐ |
| 微调效果评估报告 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘（5min）
- 最大收获：
- 遇到的障碍：
- 明天需要调整的：
