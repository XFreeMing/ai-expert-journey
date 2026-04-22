# Day 49 —— 项目8 启动

> 目标：多模态场景调研，技术选型 ADR，基础结构搭建。

> **硬件需求**：需要 >= 16GB 显存的 GPU。无 GPU 可用 Colab Pro 替代。

---

## Task 109：多模态场景深度调研

**产出物**：`docs/notes/day49-multimodal-survey.md`（技术调研报告 1000+ 字）

- [ ] 97.1 调研 VLM 模型：Qwen2.5-VL-7B、LLaVA-1.5-7B、MiniCPM-V-2.6，对比模型规模、显存需求、支持能力（45min）
- [ ] 97.2 调研图像生成方案：Stable Diffusion 1.5 vs SDXL vs FLUX.1，对比 ControlNet 支持、显存需求、生成质量（45min）
- [ ] 97.3 调研视频理解方案：Qwen2.5-VL Video、Video-LLaVA，评估是否纳入本项目范围（建议不纳入，聚焦图文）（20min）
- [ ] 97.4 整理对比表格，给出推荐结论和备选方案（20min）

## Task 110：技术选型 ADR

**产出物**：`docs/adr/008-multimodal-tech-stack.md`（技术选型架构决策）

- [ ] 98.1 确定 VLM 模型：推荐 Qwen2.5-VL-7B（中文支持好、视觉分辨率自适应），备选 LLaVA-1.5-7B（15min）
- [ ] 98.2 确定图像生成方案：Stable Diffusion 1.5 + ControlNet（兼顾显存和控制精度），备选 SDXL（15min）
- [ ] 98.3 撰写 ADR 文档：决策理由、备选方案分析、权衡因素（显存、速度、效果、生态成熟度）（30min）

## Task 111：项目8 基础结构搭建

**产出物**：`projects/08-multimodal/src/config.py` + 完整目录结构

- [ ] 99.1 创建完整目录结构：`src/{vlm,image_gen,pipeline,utils}/`、`scripts/`、`tests/`、`data/multimodal/{models,benchmark,outputs,evals}`（10min）
- [ ] 99.2 编写 `src/config.py`：管理模型路径、设备选择、显存限制、默认参数（15min）
- [ ] 99.3 编写 `src/__init__.py` 和各子模块 `__init__.py`，确认包结构正确（5min）
- [ ] 99.4 编写 `tests/test_config.py`：验证配置加载、默认值、环境变量覆盖（15min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| 多模态技术调研报告 | ☐ |
| ADR-008 技术选型文档 | ☐ |
| 项目 8 目录结构 + 配置模块 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘（5min）
- 最大收获：
- 遇到的障碍：
- 明天需要调整的：
