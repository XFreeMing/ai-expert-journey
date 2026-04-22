# Day 61 —— 集成测试日

> 目标：docker compose 全服务启动，端到端链路测试，CI 流水线验证。

---

## Task 145：全服务启动与健康检查

**产出物**：`docs/notes/day61-integration-status.md`（集成测试状态报告）

- [ ] 133.1 执行 `docker compose up -d` 启动全部服务（redis、postgres、etcd、milvus、minio、各 API 服务）（20min）
- [ ] 133.2 逐一 curl 所有 /health 端点：platform-gateway:8000、rag-api:8001、agent-orchestrator:8002、recsys-api:8003、vllm-server:8004、multimodal-api:8005、vector-engine-api:8006（15min）
- [ ] 133.3 验证 docker compose health check 全部 passing：`docker compose ps` 确认所有服务状态为 healthy/running（10min）
- [ ] 133.4 记录每个服务的启动时间、健康状态、端口占用情况、容器资源占用（CPU/内存）（10min）
- [ ] 133.5 如果有服务启动失败：查看 docker logs、排查端口冲突或资源不足、记录修复步骤（20min）

## Task 146：端到端请求链路测试

**产出物**：`scripts/integration-test.sh`（集成测试脚本）+ 测试输出日志

- [ ] 134.1 编写测试脚本：依次请求每个服务的关键 endpoint（30min）
  - `/v1/chat` -> gateway -> vllm-server（对话链路）
  - `/v1/search?q=xxx` -> rag-api（RAG 检索链路）
  - `/v1/recommend?user_id=1` -> recsys-api（推荐链路）
  - `/v1/images/generate` -> multimodal-api（图像生成链路）
- [ ] 134.2 验证 gateway 路由分发：请求 `/v1/chat` 确认 gateway 正确转发到 vllm-server（15min）
- [ ] 134.3 验证 gateway 限流：快速发送超过限制的请求，确认返回 429 Too Many Requests（10min）
- [ ] 134.4 验证 gateway 认证：不带 API Key 请求受保护端点，确认返回 401 Unauthorized（10min）
- [ ] 134.5 验证跨服务数据流：通过 gateway 请求 embedding，确认向量写入 milvus 并可检索（15min）
- [ ] 134.6 整理测试报告：所有通过的测试、失败的测试、已知问题、响应延迟数据（10min）

## Task 147：CI 流水线验证

**产出物**：CI 流水线通过截图/日志 + `docs/notes/day61-ci-status.md`

- [ ] 135.1 检查 `.gitlab-ci.yml` 配置：确认全部项目（01-09）的 lint 和 test job 都已定义，stage 顺序正确（15min）
- [ ] 135.2 本地运行 CI 预检：`ruff check` + `ruff format --check` 全部 Python 项目 + `cargo fmt --check` + `cargo clippy -- -D warnings` 全部 Rust 项目（30min）
- [ ] 135.3 运行全部测试：`pytest --cov=src/` 全部 Python 项目 + `cargo test --all-features` 全部 Rust 项目（20min）
- [ ] 135.4 修复 CI 失败项（如有），确保 lint + test 全部通过（20min）
- [ ] 135.5 验证 Docker build：`docker compose build` 确认全部服务可成功编译（15min）
- [ ] 135.6 撰写集成测试总结：整体架构是否可用、最深 3 个踩坑点、下一步优化建议（15min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| 全服务启动 + 健康检查通过 | ☐ |
| 端到端请求链路验证 | ☐ |
| CI 流水线全部通过 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘（5min）
- 最大收获：
- 遇到的障碍：
- 明天需要调整的：
