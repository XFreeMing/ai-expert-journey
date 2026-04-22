# Day 52 —— 文本生成图像

> 目标：5 prompt 测试，条件控制效果评估，参数调优。

> **硬件需求**：需要 >= 16GB 显存的 GPU。无 GPU 可用 Colab Pro 替代。

---

## Task 118：文本生成图像实验

**产出物**：`data/multimodal/outputs/day52/text2img/`（生成结果）+ `data/multimodal/evals/day52-text2img-eval.json`

- [ ] 106.1 设计 5 个覆盖不同场景的 prompt（风景、人物、建筑、抽象、产品），保存到 `data/multimodal/prompts/day52-prompts.json`（15min）
- [ ] 106.2 编写实验脚本：对每个 prompt 生成 4 张变体（不同 seed），统一参数：512x512、CFG 7.5、50 steps（15min）
- [ ] 106.3 运行实验，输出到 `data/multimodal/outputs/day52/text2img/`，按 prompt 分目录（15min）
- [ ] 106.4 设计评分维度（主体一致性、细节丰富度、构图合理性、风格匹配度），对每个 prompt 最佳输出主观评分（1-5）（20min）

## Task 119：条件控制效果评估

**产出物**：`data/multimodal/outputs/day52/controlnet/`（9 组对比输出）+ `data/multimodal/evals/day52-controlnet-comparison.json`

- [ ] 107.1 选取 3 张风格各异的输入图片（线稿风格照片、风景照、人物肖像）（5min）
- [ ] 107.2 对每张图片分别生成 Canny、Depth、OpenPose 三种条件图（15min）
- [ ] 107.3 使用相同 prompt 和种子，分别跑 3 种 ControlNet 生成（共 9 次推理），保存到 `data/multimodal/outputs/day52/controlnet/`（20min）
- [ ] 107.4 对每种控制模式评估：结构保持度、创意自由度、生成稳定性（1-5 分），记录适用场景（20min）

## Task 120：ControlNet 参数调优

**产出物**：`docs/notes/day52-controlnet-tuning-guide.md`（调参笔记）

- [ ] 108.1 实验 `controlnet_conditioning_scale` 参数（0.5 / 0.8 / 1.0 / 1.5），观察对结构保持和创意自由度的影响（25min）
- [ ] 108.2 记录各参数的效果变化，整理为调参参考表（15min）
- [ ] 108.3 总结多 ControlNet 叠加使用方法（如 Canny + Depth 同时控制），实验 `MultiControlNetModel`（20min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| 5 prompt 文本生成 + 评估 | ☐ |
| 9 组 ControlNet 对比 + 评估 | ☐ |
| ControlNet 调参指南 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘（5min）
- 最大收获：
- 遇到的障碍：
- 明天需要调整的：
