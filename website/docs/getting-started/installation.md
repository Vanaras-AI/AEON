---
sidebar_position: 1
---

# Installation

Install AEON Engine and its dependencies.

## Prerequisites

- **Rust** 1.70+ (for building the engine)
- **Python** 3.9+ (for Gemma risk model)
- **Node.js** 18+ (for dashboard)

## Quick Install

### 1. Clone Repository

```bash
git clone https://github.com/Vanaras-AI/AEON.git
cd AEON
```

### 2. Build AEON Engine

```bash
cd aeon-engine
cargo build --release
```

### 3. Install Python Dependencies (Optional)

For hybrid AI+heuristics risk scoring:

```bash
cd governance_model
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
```

### 4. Install Dashboard (Optional)

```bash
cd dashboard
npm install
```

## Model Weights

The Gemma 3 270M adapter is tracked via Git LFS:

```bash
git lfs install
git lfs pull
```

## Configuration

Copy the example environment file:

```bash
cp .env.example .env
```

Edit `.env` to configure:

- `GEMMA_RISK_SERVER_URL` - Gemma endpoint (default: `http://127.0.0.1:8001/score_risk`)
- `TODOIST_API_TOKEN` - Optional MCP integration
- `AEON_WASM_PATH` - Custom WASM module path

## Verify Installation

```bash
# Check AEON builds
cd aeon-engine && cargo check

# Run tests
cargo test
```

## Next Steps

- [Quickstart](/docs/getting-started/quickstart) - Run your first governed agent
- [Configuration](/docs/getting-started/configuration) - Detailed configuration options
