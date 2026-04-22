# Day 51 —— ControlNet 环境搭建

> 目标：部署 SD + ControlNet，完成三种控制模式对比。

> **硬件需求**：需要 >= 16GB 显存的 GPU。无 GPU 可用 Colab Pro 替代。

---

## Task 115：Stable Diffusion + ControlNet 部署

**产出物**：`projects/08-multimodal/src/image_gen/sd_pipeline.py`

- [ ] 103.1 下载 SD 1.5 基础模型到 `data/multimodal/models/sd1.5/`（约 4GB）（15min）
- [ ] 103.2 下载 3 个 ControlNet 预训练权重（Canny、Depth、OpenPose）到 `data/multimodal/models/controlnet/`（15min）
- [ ] 103.3 编写 `SDControlNetPipeline` 类：封装 `diffusers.StableDiffusionControlNetPipeline`，实现 `generate(prompt, condition_image, **kwargs)`（30min）
- [ ] 103.4 测试基础生成：传入 prompt + Canny 边缘图，验证输出图像保存到 `data/multimodal/outputs/day51/`（10min）

## Task 116：三种控制模式对比

**产出物**：`data/multimodal/outputs/day51/conditions/`（3 种条件图对比）

- [ ] 104.1 选取一张风格多样的输入图片（含人物、文字、场景）（5min）
- [ ] 104.2 用同一张图片分别生成 Canny、Depth、OpenPose 三种条件图（15min）
- [ ] 104.3 用相同 prompt 和 seed，分别跑 3 种 ControlNet 生成，保存对比结果（20min）
- [ ] 104.4 对每种控制模式评估：结构保持度、创意自由度、生成稳定性（1-5 分），记录适用场景（15min）

## Task 117：条件图像预处理工具

**产出物**：`projects/08-multimodal/src/image_gen/condition_processors.py` + `tests/test_condition_processors.py`

- [ ] 105.1 实现 Canny 边缘检测预处理：cv2.Canny，可调低/高阈值，返回 PIL Image（15min）
- [ ] 105.2 实现 Depth 深度图预处理：使用 `controlnet-aux` 的 `MidasDepthDetector`（15min）
- [ ] 105.3 实现 OpenPose 姿态图预处理：使用 `controlnet-aux` 的 `OpenposeDetector`（10min）
- [ ] 105.4 编写测试脚本：对同一张输入图生成 3 种条件图，保存对比到 `data/multimodal/outputs/day51/conditions/`（10min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| SD + ControlNet 推理管线 | ☐ |
| 三种控制模式对比评估 | ☐ |
| 条件图像预处理工具 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘（5min）
- 最大收获：
- 遇到的障碍：
- 明天需要调整的：
