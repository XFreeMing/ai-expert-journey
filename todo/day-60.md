# Day 60 —— 多模型路由 + 限流

> 目标：请求分发、负载均衡、API Key 认证。

---

## Task 142：请求路由与后端管理

**产出物**：`projects/09-ai-platform/src/services/router_service.rs`（路由分发服务）

- [ ] 130.1 完善 BackendConfig：支持从配置文件（TOML/YAML）加载后端列表，包含模型名称、URL、权重、超时设置（20min）
- [ ] 130.2 实现 ModelRouter：根据请求中的 model 字段查找对应后端列表，model 不存在时返回 400（15min）
- [ ] 130.3 实现后端健康检查：定期 ping 后端 /health endpoint，标记不健康后端为 offline（20min）
- [ ] 130.4 将后端配置加载到 Axum State，支持热重载（POST /admin/backends/reload）（15min）

## Task 143：负载均衡 + 限流中间件

**产出物**：`projects/09-ai-platform/src/services/load_balancer.rs` + `src/middleware/rate_limiter.rs`

- [ ] 131.1 实现轮询（Round Robin）策略：按顺序分发到后端，跳过 offline 后端（15min）
- [ ] 131.2 实现权重轮询（Weighted Round Robin）：按配置的权重分配请求比例（15min）
- [ ] 131.3 实现 Token Bucket 限流器：支持设置 RPS 和桶容量，使用 AtomicU64 保证线程安全（20min）
- [ ] 131.4 编写限流中间件：请求超限时返回 429 Too Many Requests，附带 Retry-After header（15min）
- [ ] 131.5 编写集成测试：模拟多个请求验证负载均衡分发结果符合预期；快速发送超过限制的请求验证限流生效（15min）

## Task 144：API Key 认证 + 后端代理

**产出物**：`projects/09-ai-platform/src/services/api_key_store.rs` + `src/handlers/proxy.rs`

- [ ] 132.1 定义 ApiKey 结构体：key 值（SHA256 哈希存储）、拥有者、权限列表（可访问的模型）、创建时间、过期时间、限流配置（20min）
- [ ] 132.2 实现 ApiKeyStore：内存存储 API Key，支持增删查，key 值以哈希形式存储（不存明文）（15min）
- [ ] 132.3 编写 API Key 认证 Extractor：从 Authorization: Bearer <key> 或 X-API-Key header 提取并验证，无效返回 401（20min）
- [ ] 132.4 编写代理 Handler：接收客户端请求 -> 验证 API Key -> 路由到后端 -> 转发请求 -> 返回后端响应（20min）
- [ ] 132.5 编写端到端测试脚本 `scripts/test_auth.sh`：用 curl 测试完整认证 -> 路由 -> 限流流程（10min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| 路由分发服务 + 健康检查 | ☐ |
| 负载均衡器 + 限流中间件 | ☐ |
| API Key 认证 + 代理 Handler | ☐ |
| 今日复盘 | ☐ |

### 今日复盘（5min）
- 最大收获：
- 遇到的障碍：
- 明天需要调整的：
