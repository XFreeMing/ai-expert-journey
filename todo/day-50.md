# Day 50 —— VLM 图像理解

> 目标：部署 Qwen-VL/LLaVA，完成 5 类任务基准测试。

> **硬件需求**：需要 >= 16GB 显存的 GPU。无 GPU 可用 Colab Pro 替代。

---

## Task 112：VLM 模型部署

**产出物**：`projects/08-multimodal/src/vlm/loader.py` + `data/multimodal/models/qwen-vl/`

- [ ] 100.1 下载 Qwen2.5-VL-7B 模型权重到 `data/multimodal/models/qwen-vl/`（使用 huggingface-cli 或镜像）（30min）
- [ ] 100.2 编写 `src/vlm/loader.py`：模型加载、device 自动选择、4-bit 量化可选开关（20min）
- [ ] 100.3 编写 `src/vlm/inference.py`：统一推理接口 `predict(image, prompt, **kwargs)` 返回文本（20min）
- [ ] 100.4 单图快速验证：用一张测试图跑通推理流程，确认输出格式正确（15min）

## Task 113：5 类基准测试数据集

**产出物**：`data/multimodal/benchmark/vlm-benchmark-set/`（5 类 x 5 张 = 25 张测试图）

- [ ] 101.1 准备 5 类测试图像，每类 5 张：OCR 文字识别、物体检测、场景理解、图表分析、多物体关系（30min）
- [ ] 101.2 为每张图编写标准问题和参考答案，保存为 `benchmark/vlm-benchmark-set/questions.json`（25min）

## Task 114：基准测试运行 + 报告

**产出物**：`projects/08-multimodal/scripts/run_vlm_benchmark.py` + `data/multimodal/evals/vlm-benchmark-results.json`

- [ ] 102.1 编写基准测试脚本：遍历 25 张图，记录每类任务的正确率、平均响应时间、显存峰值（30min）
- [ ] 102.2 运行基准测试，收集全部结果（15min）
- [ ] 102.3 生成报告：5 类任务准确率柱状图 + 延迟分布 + 典型失败案例分析（20min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| VLM 模型部署 + 推理接口 | ☐ |
| 25 张基准测试数据集 | ☐ |
| 基准测试报告 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘（5min）
- 最大收获：
- 遇到的障碍：
- 明天需要调整的：
