# Day 53 —— 多模态 Pipeline

> 目标：VLM 理解 -> Prompt 转换 -> 图像生成串联，构建端到端多模态创意工作流。

> **硬件需求**：需要 >= 16GB 显存的 GPU。无 GPU 可用 Colab Pro 替代。显存不足时使用分时加载方案：VLM 推理完卸载，再加载 SD。

---

## Task 121：Pipeline 核心编排模块

**产出物**：`projects/08-multimodal/src/pipeline/multimodal_pipeline.py` + `tests/test_pipeline.py`

- [ ] 109.1 设计 `MultimodalPipeline` 类：接受输入图片路径，内部依次调用 VLM 和 SD+ControlNet，支持配置各阶段开关（25min）
- [ ] 109.2 实现 `describe_image(image_path)` 方法：调用 VLM 生成详细的图像描述文本，支持自定义 VLM prompt 模板（15min）
- [ ] 109.3 实现 `generate_from_description(description, condition_image=None)` 方法：将描述转为 SD prompt，可选搭配 ControlNet 条件图（20min）
- [ ] 109.4 实现 `run(image_path, **kwargs)` 方法：串联完整流程，返回包含各阶段中间产物的字典（15min）
- [ ] 109.5 编写测试：mock VLM 和 SD 组件，验证 Pipeline 数据流转正确（15min）

## Task 122：Prompt 转换与增强

**产出物**：`projects/08-multimodal/src/pipeline/prompt_enhancer.py`

- [ ] 110.1 编写 `PromptEnhancer` 类：将 VLM 生成的描述性文本转换为适合 SD 的 prompt 格式（15min）
- [ ] 110.2 添加正向 prompt 模板："masterpiece, best quality, {description}, detailed, 4k, professional photography"（5min）
- [ ] 110.3 添加负向 prompt："worst quality, low quality, blurry, deformed, bad anatomy, text, watermark, signature"（5min）
- [ ] 110.4 测试 3 条 VLM 描述经转换后的生成效果，根据结果调整模板措辞（20min）

## Task 123：端到端 Demo + 性能记录

**产出物**：`projects/08-multimodal/scripts/demo_pipeline.py` + `docs/notes/day53-pipeline-performance.json`

- [ ] 111.1 编写 Demo 脚本：读取输入图片，运行完整 Pipeline（VLM 描述 -> Prompt 转换 -> SD 生成）（15min）
- [ ] 111.2 输出各阶段中间产物：VLM 描述文本、SD prompt、条件图（如有）、最终生成图，保存到 `data/multimodal/outputs/day53/`（10min）
- [ ] 111.3 用 3 张不同场景图片测试 Pipeline 稳定性，记录完整流程和每阶段耗时（15min）
- [ ] 111.4 生成性能报告 JSON：记录各阶段耗时、显存峰值（分时加载 vs 同时加载的对比）（10min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| 多模态 Pipeline 核心模块 + 测试 | ☐ |
| Prompt 增强器 | ☐ |
| 端到端 Demo + 性能报告 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘（5min）
- 最大收获：
- 遇到的障碍：
- 明天需要调整的：
