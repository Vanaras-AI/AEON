# ==========================================
# STAGE 1: The Builder (Rust)
# ==========================================
FROM rust:alpine AS builder
WORKDIR /app

# Install build dependencies
RUN apk add --no-cache musl-dev git protobuf-dev perl make

# Add WASM target
RUN rustup target add wasm32-wasip1

# Copy Source
COPY aeon-engine ./aeon-engine
COPY aeon-toolroom ./aeon-toolroom

# Build aeon-toolroom (WASM)
WORKDIR /app/aeon-toolroom
RUN cargo build --release --target wasm32-wasip1

# Build aeon-engine (Native)
WORKDIR /app/aeon-engine
RUN cargo build --release

# ==========================================
# STAGE 2: The Runtime (The Muscle)
# ==========================================
FROM python:3.11-slim-bullseye

# 1. Inject the Governor (The Conscience)
COPY --from=builder /app/aeon-engine/target/release/aeon-engine /usr/local/bin/aeon
COPY --from=builder /app/aeon-toolroom/target/wasm32-wasip1/release/aeon-toolroom.wasm /usr/local/lib/aeon-toolroom.wasm

# 2. Inject the Python SDK (The Bridge)
# We copy the 'aeon' folder directly into site-packages for immediate availability.
COPY aeon-sdk/python/aeon /usr/local/lib/python3.11/site-packages/aeon

# 3. Setup User (The Prisoner)
RUN useradd -m -u 1000 agent
WORKDIR /home/agent

# 4. Install Agent Dependencies
RUN pip install numpy

# 5. Copy Agent Code
COPY wasm_bridge_test/main.py /home/agent/agent.py

# 6. Environment & Permissions
# The binary now uses this ENV to find the WASM module
ENV AEON_WASM_PATH=/usr/local/lib/aeon-toolroom.wasm
RUN chown -R agent:agent /home/agent

# 7. The Supervisor Pattern (PID 1)
# Aeon runs as Root. It spawns the Agent.
ENTRYPOINT ["/usr/local/bin/aeon", "run", "/home/agent/agent.py"]
