# Runbook

## Deployment Procedures

### Local Development

```bash
# Start all services
scripts/ports.sh start

# Check status
scripts/ports.sh status

# Stop all services
scripts/ports.sh stop
```

### Docker Compose (Full Stack)

```bash
# Build and start all services
docker compose up -d --build

# Start specific services only
docker compose up -d rag-api recsys-api

# Rebuild a single service
docker compose build rag-api

# View logs
docker compose logs -f rag-api
```

### CI/CD Pipeline (GitLab)

The `.gitlab-ci.yml` defines four stages:

```
lint (per project) → test (per project, 80% coverage) → build (shared + platform) → deploy (manual, main only)
```

1. **Push to any branch**: lint + test jobs run automatically
2. **Push to main**: build + deploy available (manual trigger)
3. **Coverage reports**: uploaded as artifacts in Cobertura format

## Health Check Endpoints

<!-- AUTO-GENERATED from docker-compose.yml health checks -->

| Service | Health Check | Expected Response |
|---------|-------------|-------------------|
| Redis | `redis-cli ping` | `PONG` |
| PostgreSQL | `pg_isready -U ai_expert` | Exit code 0 |
| etcd | `etcdctl endpoint health` | `healthy` |
| rag-api | `http://localhost:8001/health` | 200 OK (TODO) |
| agent-orchestrator | `http://localhost:8002/health` | 200 OK (TODO) |
| recsys-api | `http://localhost:8003/health` | 200 OK (TODO) |
| vllm-server | `http://localhost:8004/health` | 200 OK |
| multimodal-api | `http://localhost:8005/health` | 200 OK (TODO) |
| platform-gateway | `http://localhost:8000/health` | 200 OK (TODO) |

<!-- /AUTO-GENERATED -->

## Common Issues

### Services Won't Start

```bash
# Check port conflicts
lsof -i :6379   # Redis
lsof -i :5432   # PostgreSQL
lsof -i :8000   # Platform gateway

# Kill conflicting process
kill -9 <PID>

# Restart docker compose
docker compose down && docker compose up -d
```

### Out of Disk Space

```bash
# Clean Docker cache
docker system prune -af

# Clean model cache
rm -rf ~/.cache/ai-expert-journey/

# Check volume sizes
docker system df -v
```

### vLLM GPU Not Detected

```bash
# Verify NVIDIA runtime
docker run --gpus all nvidia/cuda:12.1.0-base-ubuntu22.04 nvidia-smi

# Check docker-compose.yml has:
#   deploy.resources.reservations.devices[0].driver: nvidia

# Ensure HUGGING_FACE_HUB_TOKEN is set
echo $HF_TOKEN
```

### Model Download Fails

```bash
# Retry with HF_ENDPOINT for China access
export HF_ENDPOINT=https://hf-mirror.com
docker compose up -d vllm-server

# Or download manually and mount
huggingface-cli download Qwen/Qwen2.5-7B-Instruct --local-dir ./models/qwen2.5-7b
# Update docker-compose.yml to mount ./models:/models
```

### Redis Connection Refused

```bash
# Check if Redis is running
docker compose ps redis

# Check Redis logs
docker compose logs redis

# Clear stale data
docker compose down -v
docker compose up -d redis
```

## Rollback Procedures

### Service Rollback

```bash
# Rollback to previous Docker image
docker compose up -d rag-api --image rag-api:previous-tag

# Or rebuild from a specific commit
git checkout <previous-commit>
docker compose build rag-api
docker compose up -d rag-api
```

### Data Rollback

```bash
# PostgreSQL: restore from dump
docker exec -i $(docker compose ps -q postgres) psql -U ai_expert ai_expert < backup.sql

# Redis: restore from snapshot
docker cp dump.rdb $(docker compose ps -q redis):/data/dump.rdb
docker compose restart redis
```

## Alerting and Escalation

| Issue | Severity | Action |
|-------|----------|--------|
| Service down > 5 min | Critical | Restart service, check logs |
| High latency (P99 > 5s) | High | Check resource usage, scale if needed |
| Model accuracy degradation | Medium | Re-run evaluation suite, rollback if needed |
| Disk space > 90% | High | Clean Docker cache, prune volumes |
| API key expired | High | Rotate key in .env, restart services |

## Monitoring

```bash
# Quick health check for all services
scripts/ports.sh status

# Detailed service status
docker compose ps

# Resource usage
docker stats

# Per-service logs
scripts/ports.sh logs rag-api
```
