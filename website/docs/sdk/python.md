---
sidebar_position: 2
---

# Python SDK

Python SDK for the A2G Protocol, enabling AI agents to integrate with AEON governance.

## Installation

```bash
pip install a2g-sdk
```

Or install from source:

```bash
cd a2g_sdk
pip install -e .
```

## Quick Start

```python
import asyncio
from a2g_sdk import A2gClient, ClientConfig

async def main():
    config = ClientConfig(
        agent_did="did:aeon:myagent:1.0:abc123",
        governance_url="ws://localhost:3000",
    )
    
    async with A2gClient(config) as client:
        # Request permission to execute a tool
        verdict = await client.request_intent(
            tool="write_file",
            arguments={"path": "/tmp/test.txt", "content": "Hello"}
        )
        
        if verdict.is_approved:
            # Execute with capability constraints
            print(f"Approved! Risk: {verdict.risk_assessment.score}")
            await client.report_success(
                intent_id=verdict.intent_id,
                result={"bytes_written": 5},
                duration_ms=45
            )
        else:
            print(f"Denied: {verdict.error_message}")

asyncio.run(main())
```

## Message Types

### A2gIntent

```python
from a2g_sdk import A2gIntent, IntentContext

# Create an intent
intent = A2gIntent(
    agent_did="did:aeon:myagent:1.0:abc",
    tool="write_file",
    arguments={"path": "/tmp/test.txt", "content": "Hello"},
    context=IntentContext(reasoning="Creating test file"),
)

# Convert to JSON-RPC
jsonrpc = intent.to_jsonrpc()
```

### G2aVerdict

```python
from a2g_sdk import G2aVerdict, Verdict

# Parse from JSON-RPC response
verdict = G2aVerdict.from_jsonrpc(response)

# Check result
if verdict.is_approved:
    print(f"Risk: {verdict.risk_assessment.score}")
    print(f"Timeout: {verdict.capability_manifest.timeout_seconds}s")
elif verdict.is_denied:
    print(f"Denied: {verdict.error_message}")
```

### A2gReport

```python
from a2g_sdk import A2gReport, ExecutionStatus

# Success report
report = A2gReport.success(
    agent_did="did:aeon:myagent:1.0:abc",
    intent_id="intent-123",
    result={"bytes_written": 13},
    duration_ms=45,
)

# Failure report
report = A2gReport.failure(
    agent_did="did:aeon:myagent:1.0:abc",
    intent_id="intent-123",
    error="Permission denied",
)
```

## Client API

### A2gClient

```python
from a2g_sdk import A2gClient, ClientConfig

config = ClientConfig(
    agent_did="did:aeon:myagent:1.0:abc",
    governance_url="ws://localhost:3000",
    timeout_seconds=30.0,
    auto_reconnect=True,
)

# Async context manager
async with A2gClient(config) as client:
    verdict = await client.request_intent(tool, arguments)

# Manual connection
client = A2gClient(config)
await client.connect()
# ... use client
await client.disconnect()
```

### Methods

| Method | Description |
|--------|-------------|
| `connect()` | Connect to governance server |
| `disconnect()` | Disconnect from server |
| `request_intent(tool, arguments, reasoning)` | Request permission |
| `report_success(intent_id, result, duration_ms)` | Report success |
| `report_failure(intent_id, error)` | Report failure |

### Properties

| Property | Type | Description |
|----------|------|-------------|
| `is_connected` | bool | Connection status |
| `policy` | G2aPolicy | Current policy from governance |

## Risk Assessment

```python
from a2g_sdk import RiskLevel

# Get level from score
level = RiskLevel.from_score(0.85)  # RiskLevel.HIGH

# Use in conditions
if verdict.risk_assessment.level == RiskLevel.CRITICAL:
    print("Critical risk detected!")
```

## Synchronous Wrapper

For non-async contexts:

```python
from a2g_sdk.client import A2gClientSync, ClientConfig

config = ClientConfig(agent_did="did:aeon:myagent:1.0:abc")
client = A2gClientSync(config)

client.connect()
verdict = client.request_intent("write_file", {"path": "/tmp/x"})
client.disconnect()
```

## Error Handling

```python
from a2g_sdk import A2gErrorCodes

try:
    verdict = await client.request_intent(tool, args)
except TimeoutError:
    print("Request timed out")

if verdict.error_code == A2gErrorCodes.POLICY_VIOLATION:
    print(f"Policy violation: {verdict.error_message}")
```

## Policy Callbacks

```python
async def on_policy_update(policy):
    print(f"New policy version: {policy.version}")
    for tool, tp in policy.tools.items():
        print(f"  {tool}: {'allowed' if tp.allowed else 'blocked'}")

client.on_policy_update(on_policy_update)
```

## Source Code

- [`a2g_sdk/__init__.py`](https://github.com/Vanaras-AI/AEON/blob/main/a2g_sdk/__init__.py)
- [`a2g_sdk/messages.py`](https://github.com/Vanaras-AI/AEON/blob/main/a2g_sdk/messages.py)
- [`a2g_sdk/client.py`](https://github.com/Vanaras-AI/AEON/blob/main/a2g_sdk/client.py)
