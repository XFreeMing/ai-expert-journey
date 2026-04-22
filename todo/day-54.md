# Day 54 —— 批量处理优化

> 目标：显存管理、并发推理、性能报告。

> **硬件需求**：需要 >= 16GB 显存的 GPU。无 GPU 可用 Colab Pro 替代。

---

## Task 124：显存管理与优化

**产出物**：`projects/08-multimodal/src/utils/memory_manager.py` + `tests/test_memory_manager.py`

- [ ] 112.1 编写显存监控函数 `track_memory()`：记录加载模型前后的显存占用，打印各阶段峰值，支持上下文管理器模式（20min）
- [ ] 112.2 实现模型卸载机制 `ModelManager`：支持 VLM 和 SD 模型的 move_to('cuda')/move_to('cpu')，自动管理生命周期（20min）
- [ ] 112.3 实验 `torch.cuda.empty_cache()` 和 `gc.collect()` 的调用时机和效果，记录显存释放量（15min）
- [ ] 112.4 编写测试：验证模型加载 -> 推理 -> 卸载 -> 重新加载的完整循环无内存泄漏（15min）

## Task 125：批量推理优化

**产出物**：`projects/08-multimodal/src/pipeline/batch_processor.py` + `tests/test_batch_processor.py`

- [ ] 113.1 实现批量 VLM 图像描述生成：将多张图片 batch 传入 VLM，对比逐张循环的耗时差异（25min）
- [ ] 113.2 实现批量 SD 图像生成：使用 SD pipeline 的 batch_size 参数，测试 1/2/4 不同批大小（20min）
- [ ] 113.3 编写 `run_batch(image_paths, **kwargs)` 方法：接收图片列表，分批处理，自动处理 OOM 回退（降低 batch_size 重试）（20min）
- [ ] 113.4 编写测试：使用 5 张测试图片验证批处理正确性和 OOM 回退机制（10min）

## Task 126：性能对比报告

**产出物**：`docs/notes/day54-batch-optimization-report.md`（批量优化性能报告）

- [ ] 114.1 设计测试方案：10 张图片的端到端 Pipeline，对比优化前/后的各项指标（10min）
- [ ] 114.2 运行优化前基线测试：记录总耗时、各阶段耗时、显存峰值（15min）
- [ ] 114.3 运行优化后测试：使用 ModelManager 分时加载 + batch_processor，同样记录指标（15min）
- [ ] 114.4 整理对比表格：吞吐量提升倍数、显存峰值降低幅度、最佳 batch_size 推荐，分析剩余瓶颈（15min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| 显存管理工具 + 测试 | ☐ |
| 批处理优化器 + 测试 | ☐ |
| 批量优化性能报告 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘（5min）
- 最大收获：
- 遇到的障碍：
- 明天需要调整的：
