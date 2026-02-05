---
sidebar_position: 2
---

# Message Types

Detailed JSON-RPC 2.0 schemas for all A2G Protocol messages.

## A2G_INTENT

Request permission to execute a tool.

```json
{
  "jsonrpc": "2.0",
  "method": "a2g/intent",
  "params": {
    "agent_did": "did:aeon:myagent:1.0:abc123",
    "intent_id": "550e8400-e29b-41d4-a716-446655440000",
    "tool": "write_file",
    "arguments": {
      "path": "/tmp/test.txt",
      "content": "Hello, World!"
    },
    "context": {
      "session_id": "session-123",
      "parent_intent": null,
      "reasoning": "Creating test file for demonstration"
    }
  },
  "id": "req-001"
}
```

### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `agent_did` | string | ✅ | Agent DID identifier |
| `intent_id` | string | ✅ | Unique intent UUID |
| `tool` | string | ✅ | Tool name to execute |
| `arguments` | object | ✅ | Tool arguments |
| `context` | object | ❌ | Optional execution context |

## G2A_VERDICT

Governance response to an intent.

```json
{
  "jsonrpc": "2.0",
  "result": {
    "verdict": "APPROVED",
    "intent_id": "550e8400-e29b-41d4-a716-446655440000",
    "risk_assessment": {
      "score": 0.15,
      "level": "LOW",
      "model_score": 0.12,
      "heuristic_score": 0.15,
      "threats": []
    },
    "capability_manifest": {
      "max_memory_mb": 50,
      "max_cpu_percent": 10,
      "timeout_seconds": 30,
      "network_allowed": false,
      "filesystem_scope": ["/tmp/**", "/workspace/**"]
    },
    "conditions": [],
    "expires_at": "2024-01-15T10:05:00Z"
  },
  "id": "req-001"
}
```

### Verdict Values

| Verdict | Description |
|---------|-------------|
| `APPROVED` | Intent is approved for execution |
| `DENIED` | Intent is blocked by governance |
| `ESCALATE` | Requires human approval |
| `CONDITIONAL` | Approved with conditions |

### Risk Assessment

| Field | Type | Description |
|-------|------|-------------|
| `score` | float | Combined risk score (0.0-1.0) |
| `level` | enum | `CRITICAL`, `HIGH`, `MEDIUM`, `LOW` |
| `model_score` | float | Gemma model score |
| `heuristic_score` | float | Heuristic rules score |
| `threats` | array | Detected threat descriptions |

### Capability Manifest

| Field | Type | Description |
|-------|------|-------------|
| `max_memory_mb` | int | Memory limit in MB |
| `max_cpu_percent` | int | CPU usage limit |
| `timeout_seconds` | int | Execution timeout |
| `network_allowed` | bool | Network access permission |
| `filesystem_scope` | array | Allowed filesystem paths |

## A2G_REPORT

Report execution outcome.

```json
{
  "jsonrpc": "2.0",
  "method": "a2g/report",
  "params": {
    "agent_did": "did:aeon:myagent:1.0:abc123",
    "intent_id": "550e8400-e29b-41d4-a716-446655440000",
    "status": "SUCCESS",
    "result": {
      "bytes_written": 13,
      "path": "/tmp/test.txt"
    },
    "metrics": {
      "duration_ms": 45,
      "memory_used_mb": 2,
      "cpu_percent": 1.5
    }
  },
  "id": "report-001"
}
```

### Status Values

| Status | Description |
|--------|-------------|
| `SUCCESS` | Tool executed successfully |
| `FAILURE` | Tool execution failed |
| `TIMEOUT` | Execution exceeded timeout |
| `ABORTED` | Execution was cancelled |

## A2G_REGISTER

Register agent with governance on startup.

```json
{
  "jsonrpc": "2.0",
  "method": "a2g/register",
  "params": {
    "agent_did": "did:aeon:myagent:1.0:abc123",
    "public_key": "ed25519:abc123...",
    "capabilities_requested": [
      "write_file",
      "read_file",
      "execute_command"
    ],
    "metadata": {
      "name": "My Agent",
      "version": "1.0.0",
      "runtime": "python"
    }
  },
  "id": "register-001"
}
```

## G2A_POLICY

Governance sends current capabilities to agent.

```json
{
  "jsonrpc": "2.0",
  "method": "g2a/policy",
  "params": {
    "agent_did": "did:aeon:myagent:1.0:abc123",
    "version": "2024-01-15",
    "capabilities": {
      "tools": {
        "write_file": {"allowed": true, "constraints": {}},
        "execute_command": {"allowed": true, "constraints": {"blocked_patterns": ["rm -rf"]}}
      },
      "network": {
        "allowed_domains": ["api.github.com"],
        "blocked_domains": ["*.onion"]
      },
      "resources": {
        "max_memory_mb": 256,
        "max_cpu_percent": 50
      }
    },
    "constitution_hash": "sha256:abc123..."
  }
}
```

## Error Response

When an intent is denied:

```json
{
  "jsonrpc": "2.0",
  "error": {
    "code": -32000,
    "message": "Policy violation: Network access to blocked domain",
    "data": {
      "intent_id": "550e8400-e29b-41d4-a716-446655440000",
      "risk_score": 0.92,
      "blocked_by": "static_policy"
    }
  },
  "id": "req-001"
}
```

### Error Codes

| Code | Name | Description |
|------|------|-------------|
| -32700 | Parse Error | Invalid JSON |
| -32600 | Invalid Request | Invalid JSON-RPC |
| -32601 | Method Not Found | Unknown method |
| -32602 | Invalid Params | Invalid parameters |
| -32000 | Policy Violation | Blocked by policy |
| -32001 | Execution Error | Tool execution failed |
| -32002 | Registration Failed | Agent registration failed |
| -32003 | Capability Exhausted | Resource limits exceeded |
| -32004 | Session Expired | Session timeout |
