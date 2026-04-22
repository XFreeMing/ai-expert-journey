# Day 41 —— 模型评估：BLEU/ROUGE 评分 + 人工评估

> 目标：使用自动评估指标和人工评估对 Day 40 训练的模型进行全面评测。

> **硬件需求**：需要 >= 24GB 显存的 GPU。如果没有 GPU，可用 Colab Pro (A100) 或 Hugging Face Spaces 替代。

---

## Task 92：自动评估指标（BLEU/ROUGE）

**产出物**：`projects/07-finetuning/evaluation/auto_metrics.json`

- [ ] 92.1 准备测试集：从验证集中选 50 条高质量问答对，确保覆盖不同领域和难度（20min）
- [ ] 92.2 安装评估工具：nltk（BLEU）、rouge-score（ROUGE-L）、sacrebleu（15min）
- [ ] 92.3 用基座模型（微调前）对测试集生成答案，保存为基线结果（20min）
- [ ] 92.4 用 SFT 微调后模型对同一测试集生成答案（30min）
- [ ] 92.5 计算 BLEU-1/2/4、ROUGE-L 分数，对比基线和微调后的提升幅度（30min）
- [ ] 92.6 分析：BLEU/ROUGE 的局限性——为什么这些指标不能完全反映 LLM 生成质量？（15min）

## Task 93：人工评估

**产出物**：`projects/07-finetuning/evaluation/human_eval_results.csv`

- [ ] 93.1 设计评估表：50 条测试样本，每条列基座输出和 SFT 输出，盲评打分（15min）
- [ ] 93.2 评估维度：相关性（是否回答了指令）、准确性（事实是否正确）、流畅性（语言是否自然）、完整性（是否遗漏关键信息）（15min）
- [ ] 93.3 完成 50 条样本的人工评估，按 1-5 分制打分（45min）
- [ ] 93.4 统计各维度平均分、胜率（SFT 优于基座的比例）、显著差异的样本（15min）
- [ ] 93.5 撰写人工评估报告到 `docs/notes/human-eval-report.md`，含典型案例（20min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| BLEU/ROUGE 自动评估 | ☐ |
| 50 条人工评估 | ☐ |
| 评估报告 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘
- 最大收获：
- 遇到的障碍：
- 明天需要调整的：
