---
sidebar_position: 1
---

# Introduction to AEON

**AEON** (Autonomous Execution & Orchestration Network) is a **governance-first AI runtime** built on Rust and WebAssembly. It provides secure, policy-enforced execution for AI agents with local intelligence and zero-trust architecture.

## Why AEON?

AI agents are increasingly being used to automate complex tasks, but they come with significant risks:

- **Security** - Agents can execute arbitrary code, access files, and make network requests
- **Compliance** - Regulated industries need audit trails and policy enforcement
- **Trust** - Users need confidence that agents won't act maliciously

AEON solves these problems by implementing a **6-Phase Governance Loop** that evaluates every agent action before execution.

## Key Features

### 🛡️ 6-Phase Governance Loop

Every AI agent action passes through a comprehensive security pipeline:

1. **Intent Transformation** - Parse and validate tool calls
2. **Static Policy Evaluation** - Check against constitution rules
3. **Advisory Model (Hybrid Risk Scoring)** - AI + heuristics threat detection
4. **Capability Manifest** - Resource limits and network policies
5. **WASM Isolation** - Sandboxed execution in secure containers
6. **Verification & Audit** - Telemetry and compliance logging

### 🧠 Hybrid Risk Scoring

- **Gemma 3 270M** (GRC-trained) for unknown threat detection
- **Battle-tested heuristics** for known attack patterns
- **100% threat detection** with `max(model, heuristic)` strategy
- **Local inference** - no cloud dependency

### 🔒 Security-First Architecture

- **WASM Isolation**: Perfect security boundaries for agent execution
- **Policy-as-Code**: Constitution-based governance (TOML)
- **Zero-Trust**: Every action validated before execution
- **Audit Trail**: Complete telemetry for compliance

### 🔗 A2G Protocol

AEON implements the **A2G (Agent-to-Governance) Protocol** - a standardized interface for AI agent governance that can be adopted by other frameworks.

## Quick Example

```python
from a2g_sdk import A2gClient, ClientConfig

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
```

## Next Steps

- [Installation](/docs/getting-started/installation) - Get AEON running
- [Quickstart](/docs/getting-started/quickstart) - Run your first governed agent
- [A2G Protocol](/docs/a2g-protocol/overview) - Understand the governance protocol
