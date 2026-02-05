---
sidebar_position: 3
---

# Configuration

Environment variables and policy configuration for AEON.

## Environment Variables

AEON is configured via environment variables. See `.env.example` for all options.

| Variable | Default | Description |
|----------|---------|-------------|
| `GEMMA_RISK_SERVER_URL` | `http://127.0.0.1:8001/score_risk` | Gemma risk scoring endpoint |
| `TODOIST_API_TOKEN` | - | Optional Todoist MCP integration |
| `AEON_WASM_PATH` | `../aeon-toolroom/target/...` | Path to WASM toolroom module |
| `TELEMETRY_SECRET` | - | Authentication token for telemetry WebSocket |

### Example `.env` File

```bash
# Gemma Risk Model
GEMMA_RISK_SERVER_URL=http://127.0.0.1:8001/score_risk

# Optional Integrations
TODOIST_API_TOKEN=your_token_here

# Development
RUST_LOG=info
```

## Constitution (Policy-as-Code)

AEON policies are defined in TOML format at `mandates/constitution.toml`:

```toml
[network]
allow = [
    "api.github.com",
    "pypi.org",
    "registry.npmjs.org"
]

block = [
    "*.onion",
    "pastebin.com"
]

[filesystem]
write_allow = [
    "/tmp/**",
    "/workspace/**",
    "~/.aeon/**"
]
```

### Network Policy

- `allow` - Domains that agents can access
- `block` - Domains that are always blocked

### Filesystem Policy

- `write_allow` - Paths where agents can write files
- Glob patterns supported (`**`, `*`)

## Risk Thresholds

Default risk scoring thresholds:

| Score | Level | Action |
|-------|-------|--------|
| ≥ 0.8 | CRITICAL/HIGH | **Block** |
| ≥ 0.5 | MEDIUM | Allow with warning |
| < 0.5 | LOW | Allow silently |

These are hardcoded in `aeon-engine/src/main.rs` but can be made configurable.

## Dashboard Configuration

The dashboard connects to AEON via WebSocket:

```javascript
// dashboard/src/App.jsx
const WS_URL = 'ws://localhost:3000/telemetry';
```

## Resource Limits

Capability manifests define per-tool resource limits:

| Tool | Memory | CPU | Timeout | Network |
|------|--------|-----|---------|---------|
| `write_file` | 10 MB | 10% | 30s | ❌ |
| `read_file` | 50 MB | 10% | 30s | ❌ |
| `execute_command` | 100 MB | 50% | 60s | ✅ |

These are defined in `aeon-engine/src/a2g.rs` in `CapabilityManifest::for_tool()`.
