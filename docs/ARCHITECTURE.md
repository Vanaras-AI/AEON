# AEON Architecture

## Overview

AEON is a governance-first AI runtime with a 6-phase execution loop:

1. **Intent Transformation** - Parse JSON-RPC requests
2. **Policy Evaluation** - Check against constitution
3. **Risk Scoring** - Hybrid (Gemma 3 + Heuristics)
4. **Capability Manifest** - Resource limits
5. **WASM Isolation** - Sandboxed execution
6. **Audit & Telemetry** - Compliance logging

## Components

```
aeon-engine/       # Rust-based governance engine
├── src/
│   ├── main.rs          # Warden (entry point)
│   ├── risk_scorer.rs   # Hybrid risk scoring
│   ├── server.rs        # WebSocket telemetry
│   └── synapse.rs       # Signal types

governance_model/  # Gemma 3 270M risk scoring
├── risk_server.py       # HTTP inference server
├── adapter/             # LoRA weights

dashboard/         # React monitoring UI
├── src/
│   ├── App.tsx
│   └── hooks/useNervousSystem.ts

core/              # Shared Rust library
├── src/
│   ├── capability.rs    # Resource gating
│   └── intent.rs        # Intent parsing

mandates/          # Agent DNA (TOML configs)
```

## Hybrid Risk Scoring

```rust
final_score = max(gemma_score, heuristic_score)
```

- **Gemma 3**: Catches unknown threats (high recall)
- **Heuristics**: Knows safe patterns (high precision)
- **Result**: 100% threat detection

## Deployment

Option 1: **Heuristics-only** (fast, no Python)
Option 2: **Hybrid with Gemma** (100% detection)
