---
sidebar_position: 1
---

# A2G Protocol Overview

The **A2G (Agent-to-Governance) Protocol** is a standardized communication interface between AI agents and governance systems. AEON is the reference implementation.

## What is A2G?

A2G defines how AI agents request permission to perform actions, how governance evaluates those requests, and how outcomes are reported for audit.

```
Agent                              Governance
  |                                    |
  |-------- A2G_INTENT --------------->|
  |        (request permission)        |
  |                                    |
  |<------- G2A_VERDICT ---------------|
  |        (approve/deny)              |
  |                                    |
  |         [Execute in WASM]          |
  |                                    |
  |-------- A2G_REPORT --------------->|
  |        (report outcome)            |
```

## Protocol Stack

| Layer | Description |
|-------|-------------|
| **Application** | Agent logic and tool execution |
| **A2G Messages** | Intent, Report, Register, Verdict, Policy |
| **Transport** | WebSocket or HTTP |
| **Wire** | JSON-RPC 2.0 |

## Message Types

### Agent → Governance (A2G)

| Message | Purpose |
|---------|---------|
| `A2G_INTENT` | Request permission to execute a tool |
| `A2G_REPORT` | Report execution outcome |
| `A2G_REGISTER` | Register agent on startup |
| `A2G_HEARTBEAT` | Keep connection alive |

### Governance → Agent (G2A)

| Message | Purpose |
|---------|---------|
| `G2A_VERDICT` | Approve/deny with risk assessment |
| `G2A_POLICY` | Send capability manifest |
| `G2A_DIRECTIVE` | Request immediate action |
| `G2A_REVOKE` | Revoke agent capabilities |

## Why A2G?

### 🔒 Security
- Every agent action is validated before execution
- Capability manifests enforce resource limits
- Full audit trail for compliance

### 🔌 Interoperability
- Any agent framework can implement A2G
- Standardized message format (JSON-RPC 2.0)
- Language-agnostic protocol

### 📊 Observability
- Real-time telemetry via WebSocket
- Risk scoring with explanations
- Execution metrics

## Quick Example

```python
from a2g_sdk import A2gIntent

# Create an intent
intent = A2gIntent(
    agent_did="did:aeon:myagent:1.0:abc",
    tool="write_file",
    arguments={"path": "/tmp/test.txt", "content": "Hello"}
)

# Send to governance
response = await client.send(intent.to_jsonrpc())
verdict = G2aVerdict.from_jsonrpc(response)

if verdict.is_approved:
    # Execute with constraints from capability manifest
    manifest = verdict.capability_manifest
    # max_memory_mb, timeout_seconds, etc.
```

## Next Steps

- [Message Types](/docs/a2g-protocol/message-types) - Detailed message schemas
- [Governance Flow](/docs/a2g-protocol/governance-flow) - 6-phase pipeline
- [Risk Scoring](/docs/a2g-protocol/risk-scoring) - Hybrid AI scoring
