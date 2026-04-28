# 运维手册

## 部署流程

### 本地开发

```bash
# 启动全部服务
scripts/ports.sh start

# 检查服务状态
scripts/ports.sh status

# 停止全部服务
scripts/ports.sh stop
```

### Docker Compose（全栈部署）

```bash
# 构建并启动全部服务
docker compose up -d --build

# 仅启动指定服务
docker compose up -d rag-api recsys-api

# 启动 Rust 工具（按需）
docker compose --profile tools up code-reviewer-cli vector-engine-api llm-inference-tools

# 重新构建单个服务
docker compose build rag-api

# 查看服务日志
docker compose logs -f rag-api
```

### CI/CD 流水线（GitLab）

`.gitlab-ci.yml` 定义了四个阶段：

```
lint（按项目，Python 用 uv run ruff / Rust 用 clippy） → test（按项目，80% 覆盖率） → build（shared + platform） → deploy（手动触发，仅 main 分支）
```

1. **推送到任意分支**：lint + test 自动执行
2. **推送到 main**：build 自动执行，deploy 需手动触发
3. **覆盖率报告**：以 Cobertura 格式上传为构建产物（Python 项目），Rust 项目运行 `cargo test`

## 健康检查

<!-- AUTO-GENERATED 来源：docker-compose.yml -->

| 服务名 | 健康检查方式 | 端口 | 预期结果 |
|--------|-------------|------|---------|
| Redis | `redis-cli ping` | 6379 | 返回 `PONG` |
| PostgreSQL | `pg_isready -U ai_expert` | 5432 | 退出码 0 |
| etcd | `etcdctl endpoint health` | 2379 | 返回 healthy |
| platform-gateway | `curl http://localhost:8000/` | 8000 | 200 OK（待实现 /health） |
| rag-api | `curl http://localhost:8001/` | 8001 | 200 OK（待实现 /health） |
| agent-orchestrator | `curl http://localhost:8002/` | 8002 | 200 OK（待实现 /health） |
| recsys-api | `curl http://localhost:8003/` | 8003 | 200 OK（待实现 /health） |
| multimodal-api | `curl http://localhost:8005/` | 8005 | 200 OK（待实现 /health） |
| vector-engine-api | `curl http://localhost:8006/` | 8006 | 200 OK（待实现 /health） |

### 快速健康诊断

```bash
# 快速检查所有服务端口状态
scripts/ports.sh status

# 详细容器状态
docker compose ps

# 查看资源占用
docker stats
```

<!-- /AUTO-GENERATED -->

## 常见问题排查

### 服务无法启动

```bash
# 检查端口占用情况
lsof -i :6379    # Redis
lsof -i :5432    # PostgreSQL
lsof -i :8000    # 平台网关

# 杀死占用进程
kill -9 <PID>

# 重启 Docker Compose
docker compose down && docker compose up -d
```

### Python 依赖同步失败

```bash
# 清理 venv 并重新同步
rm -rf .venv
uv venv
uv sync

# 如果 shared 模块有变更，重新安装 shared
uv sync --reinstall-package shared
```

### Rust 服务构建失败

```bash
# 确认工具链版本
rustc --version
cargo --version

# 清理并重新构建
cd projects/09-ai-platform
cargo clean
cargo build --release

# 检查 clippy 警告
cargo clippy -- -D warnings
```

### 磁盘空间不足

```bash
# 清理 Docker 缓存
docker system prune -af

# 清理模型缓存
rm -rf ~/.cache/ai-expert-journey/

# 清理 uv 缓存
uv cache clean

# 清理 Rust target 目录
find projects/ -name target -type d -exec rm -rf {} + 2>/dev/null

# 检查各数据卷大小
docker system df -v
```

### Redis 连接被拒绝

```bash
# 检查 Redis 容器状态
docker compose ps redis

# 查看 Redis 日志
docker compose logs redis

# 清理脏数据后重启
docker compose down -v
docker compose up -d redis
```

### PostgreSQL 初始化失败

```bash
# 检查数据库是否已存在
docker compose exec postgres psql -U ai_expert -l

# 重置数据库
docker compose down -v postgres-data
docker compose up -d postgres
```

## 回滚流程

### 服务回滚

```bash
# 回滚到上一个 Docker 镜像版本
docker compose up -d rag-api --image rag-api:previous-tag

# 或回退到指定提交重新构建
git checkout <上一个稳定提交>
docker compose build rag-api
docker compose up -d rag-api
```

### 数据回滚

```bash
# PostgreSQL：从备份恢复
docker exec -i $(docker compose ps -q postgres) psql -U ai_expert ai_expert < backup.sql

# Redis：从快照恢复
docker cp dump.rdb $(docker compose ps -q redis):/data/dump.rdb
docker compose restart redis
```

## 告警与升级

| 问题 | 严重等级 | 处理方式 |
|------|---------|---------|
| 服务宕机 > 5 分钟 | 严重 | 重启服务，检查日志 |
| 高延迟（P99 > 5 秒） | 高 | 检查资源占用，必要时扩容 |
| 模型精度下降 | 中 | 重新运行评测套件，必要时回滚 |
| 磁盘使用率 > 90% | 高 | 清理 Docker 缓存、修剪数据卷 |

## 监控

```bash
# 一键检查所有服务状态
scripts/ports.sh status

# 查看指定服务日志
scripts/ports.sh logs rag-api

# 查看 Docker 资源占用
docker stats

# 查看 Milvus 连接状态
docker compose exec milvus milvusctl connection list  # 如有 milvusctl
```

## 端口管理速查

```
8000  → platform-gateway     （统一入口，Rust/Axum）
8001  → rag-api              （RAG 知识系统，Python）
8002  → agent-orchestrator   （Agent 编排，Python）
8003  → recsys-api            （推荐引擎，Python）
8005  → multimodal-api        （多模态生成，Python）
8006  → vector-engine-api     （向量引擎，Rust）
6379  → Redis
5432  → PostgreSQL
19530 → Milvus
```
