# A2G SDK - Agent-to-Governance Protocol for Python

Python SDK for the A2G (Agent-to-Governance) Protocol, enabling AI agents to securely communicate with AEON governance systems.

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
            result = execute_tool(verdict.capability_manifest)
            await client.report_success(verdict.intent_id, result, duration_ms=45)
        else:
            print(f"Denied: {verdict.error_message}")

asyncio.run(main())
```

## Message Types

### A2G (Agent → Governance)

| Message | Description |
|---------|-------------|
| `A2gIntent` | Request permission for tool execution |
| `A2gReport` | Report execution outcome |
| `A2gRegister` | Register agent with governance |

### G2A (Governance → Agent)

| Message | Description |
|---------|-------------|
| `G2aVerdict` | Approval/denial with risk assessment |
| `G2aPolicy` | Capability manifest and constraints |

## API Reference

### A2gClient

```python
client = A2gClient(ClientConfig(...))

# Async methods
await client.connect()
await client.disconnect()
verdict = await client.request_intent(tool, arguments, reasoning=None)
await client.report_success(intent_id, result, duration_ms)
await client.report_failure(intent_id, error)

# Properties
client.is_connected  # bool
client.policy        # G2aPolicy or None
```

### G2aVerdict

```python
verdict.is_approved      # bool
verdict.is_denied        # bool
verdict.intent_id        # str
verdict.risk_assessment  # RiskAssessment
verdict.capability_manifest  # CapabilityManifest or None
verdict.error_message    # str or None
```

## Synchronous Usage

For non-async contexts:

```python
from a2g_sdk.client import A2gClientSync, ClientConfig

config = ClientConfig(agent_did="did:aeon:myagent:1.0:abc")
client = A2gClientSync(config)

client.connect()
verdict = client.request_intent("write_file", {"path": "/tmp/x"})
client.disconnect()
```

## License

MIT License - see [LICENSE](../LICENSE)

## Documentation

- [A2G Protocol Specification](../docs/A2G_PROTOCOL.md)
- [AEON Security](../SECURITY.md)
