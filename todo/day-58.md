# Day 58 —— 项目9 启动

> 目标：网关架构设计 ADR，API 设计，数据模型。

---

## Task 136：架构设计与 ADR

**产出物**：`docs/adr/009-gateway-architecture.md`（网关架构决策记录）

- [ ] 124.1 回顾项目 9 目标：Rust + Axum 实现 AI 模型网关，统一接入前端/客户端请求，支持多模型路由、限流、认证、计费（10min）
- [ ] 124.2 绘制架构图（mermaid 或手绘）：客户端 -> 网关（认证 -> 限流 -> 路由 -> 转发）-> 后端模型服务（vLLM、SD API、embedding 服务）-> 聚合响应（20min）
- [ ] 124.3 记录架构决策：为什么选 Axum（Tokio 生态、类型安全、异步优先、tower 兼容）、后端服务协议选择（HTTP/JSON vs gRPC）（20min）
- [ ] 124.4 定义网关职责边界：认证鉴权、路由分发、限流、日志、请求追踪、不处理业务推理逻辑（10min）

## Task 137：路由规划与 API 设计

**产出物**：`docs/notes/day58-api-design.md`（API 设计规范 + 路由表）

- [ ] 125.1 设计路由表：`/v1/chat/completions`（LLM 对话，OpenAI 兼容）、`/v1/embeddings`（向量嵌入）、`/v1/images/generations`（图像生成）、`/v1/models`（模型列表）（20min）
- [ ] 125.2 为每个 endpoint 定义 request/response schema（JSON 格式，兼容 OpenAI API 格式）（25min）
- [ ] 125.3 设计错误码规范：业务错误码前缀 + HTTP 状态码映射（400/401/403/429/500/502/503）（10min）
- [ ] 125.4 设计 API 版本管理策略（URL 路径版本 `/v1/`）和向后兼容原则（10min）

## Task 138：数据模型 + 配置模块

**产出物**：`projects/09-ai-platform/src/models/`（数据模型）+ `src/config.rs`（配置模块）

- [ ] 126.1 编写 `src/models/request.rs`：定义 ChatCompletionRequest、EmbeddingRequest、ImageGenerationRequest 结构体（带 serde 注解和验证）（20min）
- [ ] 126.2 编写 `src/models/response.rs`：定义 ChatCompletionResponse、ErrorResponse（统一的 `{error: {code, message}}` 格式）（15min）
- [ ] 126.3 编写 `src/config.rs`：从环境变量/config 文件加载配置（监听地址、后端服务列表、限流阈值、JWT secret）（15min）
- [ ] 126.4 编写 `src/models/backend.rs`：定义 BackendConfig 结构体（模型名称、后端 URL、权重、健康状态、超时设置）（10min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| 架构设计 ADR-009 | ☐ |
| API 设计规范文档 | ☐ |
| 数据模型 + 配置模块 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘（5min）
- 最大收获：
- 遇到的障碍：
- 明天需要调整的：
