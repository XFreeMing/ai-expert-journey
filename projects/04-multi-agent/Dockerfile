FROM python:3.11-slim

# Install uv
COPY --from=ghcr.io/astral-sh/uv:latest /uv /usr/local/bin/uv

WORKDIR /app

# Copy workspace root and shared
COPY pyproject.toml ./
COPY shared/pyproject.toml ./shared/pyproject.toml

# Copy project pyproject.toml and lock file
COPY projects/04-multi-agent/pyproject.toml ./projects/04-multi-agent/pyproject.toml
COPY projects/04-multi-agent/uv.lock ./projects/04-multi-agent/uv.lock

# Install dependencies with lock file
WORKDIR /app/projects/04-multi-agent
RUN uv sync --frozen --no-dev

# Copy project source and shared modules
COPY projects/04-multi-agent/src/ ./src/
COPY shared/ /app/shared/

EXPOSE 8000

CMD ["uv", "run", "uvicorn", "src.main:app", "--host", "0.0.0.0", "--port", "8000"]
