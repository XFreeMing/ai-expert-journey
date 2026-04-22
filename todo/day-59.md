# Day 59 —— Axum 基础框架

> 目标：路由、中间件、错误处理、认证骨架。

---

## Task 139：路由系统搭建

**产出物**：`projects/09-ai-platform/src/routes/mod.rs` + `src/handlers/`（路由 + Handler 模块）

- [ ] 127.1 编写 `src/routes/mod.rs`，使用 Router 定义全部路由组：`/v1/chat/completions`、`/v1/embeddings`、`/v1/images/generations`、`/v1/models`、`/health`（20min）
- [ ] 127.2 为每个路由编写 placeholder handler：返回 "not implemented" 或 mock 数据（15min）
- [ ] 127.3 使用 Router::merge 组合子路由，挂载到 AppState，验证路由注册正确（10min）
- [ ] 127.4 编写单元测试：使用 axum::test_utils 验证各路由返回预期 HTTP 状态码和 response 格式（15min）

## Task 140：中间件链

**产出物**：`projects/09-ai-platform/src/middleware/mod.rs`（中间件模块）

- [ ] 128.1 实现 Request ID 中间件：为每个请求生成 UUID 作为 X-Request-ID，加入 request extensions 和 response header（20min）
- [ ] 128.2 实现请求日志中间件：使用 tracing 记录请求方法、路径、X-Request-ID、耗时、状态码（15min）
- [ ] 128.3 实现 CORS 中间件：使用 tower-http 的 CorsLayer，配置允许的 origin、method、header（10min）
- [ ] 128.4 在 Router 中按正确顺序挂载中间件：CORS -> Request ID -> 日志 -> 路由，验证中间件链生效（10min）

## Task 141：统一错误处理 + 认证骨架

**产出物**：`projects/09-ai-platform/src/error.rs` + `src/middleware/auth_skeleton.rs`

- [ ] 129.1 使用 thiserror 定义 AppError 枚举：NotFound、BadRequest、Unauthorized、Forbidden、InternalError、ServiceUnavailable、BackendTimeout（15min）
- [ ] 129.2 实现 IntoResponse for AppError：统一错误响应格式 `{"error": {"code": "xxx", "message": "xxx", "request_id": "xxx"}}`（15min）
- [ ] 129.3 实现 From trait：将 serde_json::Error、reqwest::Error、tower::timeout::error 等转换为 AppError（10min）
- [ ] 129.4 编写认证中间件骨架：定义 AuthExtractor placeholder（从 Authorization header 提取 token，暂时跳过验证，返回 mock user）（15min）
- [ ] 129.5 编写测试：触发各类型错误（404 路由不存在、500 内部错误），验证响应格式和 HTTP 状态码正确（10min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| 路由系统 + Handler + 测试 | ☐ |
| 中间件链（Request ID + 日志 + CORS） | ☐ |
| 统一错误处理 + 认证骨架 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘（5min）
- 最大收获：
- 遇到的障碍：
- 明天需要调整的：
