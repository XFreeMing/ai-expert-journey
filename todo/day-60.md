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

## Task 144：请求代理 Handler

**产出物**：`projects/09-ai-platform/src/handlers/proxy.rs`

- [ ] 132.4 编写代理 Handler：接收客户端请求 -> 路由到后端 -> 转发请求 -> 返回后端响应（20min）
- [ ] 132.5 编写集成测试脚本 `scripts/test_proxy.sh`：用 curl 测试完整代理流程（10min）

---

## 今日完成度

| 维度 | 状态 |
|------|------|
| 路由分发服务 + 健康检查 | ☐ |
| 负载均衡器 + 限流中间件 | ☐ |
| 代理 Handler + 集成测试 | ☐ |
| 今日复盘 | ☐ |

### 今日复盘（5min）
- 最大收获：
- 遇到的障碍：
- 明天需要调整的：
