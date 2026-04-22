FROM python:3.11-slim

WORKDIR /app

# Shared modules
COPY shared/ ./shared/

# Project dependencies
COPY projects/03-rag-system/requirements.txt ./requirements.txt
RUN pip install --no-cache-dir -r requirements.txt

# Project source
COPY projects/03-rag-system/src/ ./src/

EXPOSE 8000

CMD ["uvicorn", "src.main:app", "--host", "0.0.0.0", "--port", "8000"]
