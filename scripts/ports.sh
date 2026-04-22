#!/usr/bin/env bash
# Port management script for all services
# Usage: scripts/ports.sh [start|stop|status|logs]
set -euo pipefail

SERVICES=(
    "redis:6379"
    "postgres:5432"
    "milvus:19530"
    "rag-api:8001"
    "agent-orchestrator:8002"
    "recsys-api:8003"
    "vllm-server:8004"
    "multimodal-api:8005"
    "platform-gateway:8000"
)

case "${1:-status}" in
    start)
        echo "Starting all services..."
        docker compose up -d
        echo "Waiting for services to be ready..."
        sleep 5
        docker compose ps
        ;;
    stop)
        echo "Stopping all services..."
        docker compose down
        ;;
    status)
        echo "Service Status:"
        echo "==============="
        for service_info in "${SERVICES[@]}"; do
            service="${service_info%%:*}"
            port="${service_info##*:}"
            if lsof -i ":$port" > /dev/null 2>&1; then
                echo "  $service: RUNNING (port $port)"
            else
                echo "  $service: STOPPED (port $port)"
            fi
        done
        ;;
    logs)
        docker compose logs -f "${2:-}"
        ;;
    *)
        echo "Usage: $0 {start|stop|status|logs [service]}"
        exit 1
        ;;
esac
