####################### rust builder
FROM rust:latest AS builder

RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    &&\
    apt-get clean

COPY . /build

WORKDIR /build/dtiku-ai

RUN cargo build --release

###################### download container
FROM python:3.11-slim as downloader

RUN pip install --no-cache-dir huggingface_hub
# 设置模型路径和缓存目录
ENV HF_HOME=/hf-cache
RUN mkdir -p $HF_HOME

# 下载模型文件（snapshot 下载可选）
RUN python -c "from huggingface_hub import snapshot_download; \
    snapshot_download(repo_id='intfloat/multilingual-e5-base', cache_dir='/hf-cache/sentence-transformers'); \
    snapshot_download(repo_id='Qdrant/resnet50-onnx', cache_dir='/hf-cache/resnet');"

###################### runner container
FROM debian:bookworm-slim

# 安装必要工具和依赖
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    ca-certificates \
    unzip \
    libomp5 \
    && rm -rf /var/lib/apt/lists/*

# 设置版本
ENV ORT_VERSION=1.20.1

# 下载、解压、安装并清理
RUN curl -sL https://github.com/microsoft/onnxruntime/releases/download/v${ORT_VERSION}/onnxruntime-linux-x64-${ORT_VERSION}.tgz \
    -o /tmp/onnxruntime.tgz && \
    mkdir -p /tmp/ort && \
    tar -xzf /tmp/onnxruntime.tgz -C /tmp/ort --strip-components=1 && \
    cp -r /tmp/ort/lib/* /usr/local/lib/ && \
    cp -r /tmp/ort/include/* /usr/local/include/ && \
    ldconfig && \
    rm -rf /tmp/onnxruntime.tgz /tmp/ort

ENV RUST_LOG=info

WORKDIR /runner

COPY --from=downloader /hf-cache ./.hf-cache
COPY --from=builder /build/target/release/ai ./dtiku-ai

COPY ./static ./static
COPY ./config ./config

EXPOSE 7860

ENV SPRING_ENV=prod

ENTRYPOINT ["/runner/dtiku-ai"]