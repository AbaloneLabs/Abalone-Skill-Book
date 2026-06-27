FROM rust:1-trixie AS builder

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libssl-dev pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY mcp-server/Cargo.toml mcp-server/Cargo.lock ./mcp-server/
COPY mcp-server/src ./mcp-server/src

WORKDIR /app/mcp-server
RUN cargo build --release

FROM debian:trixie-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/mcp-server/target/release/abalone_mcp_server /usr/local/bin/abalone-skill-book
COPY skills ./skills

ENV ABALONE_SKILLS_ROOT=/app/skills
ENV ABALONE_DATABASE_PATH=/data/abalone.sqlite3
ENV ABALONE_MODEL_DIR=/models/bge-m3
ENV ABALONE_HTTP_HOST=0.0.0.0
ENV ABALONE_HTTP_PORT=8732
ENV ABALONE_MCP_PATH=/mcp

EXPOSE 8732
VOLUME ["/data", "/models"]

CMD ["abalone-skill-book", "serve"]
