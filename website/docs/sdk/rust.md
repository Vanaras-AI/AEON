---
sidebar_position: 1
---

# Rust SDK

Native Rust implementation of the A2G Protocol in the AEON Engine.

## Overview

The Rust A2G implementation is in `aeon-engine/src/a2g.rs`. It provides type-safe message types and helper functions for building governance-aware applications.

## Installation

The A2G module is part of the `aeon-engine` crate:

```toml
[dependencies]
aeon-engine = { path = "../aeon-engine" }
```

## Message Types

### A2G_INTENT

```rust
use aeon_engine::a2g::{A2gIntent, IntentContext};

let intent = A2gIntent::new(
    "did:aeon:myagent:1.0:abc",
    "write_file",
    serde_json::json!({
        "path": "/tmp/test.txt",
        "content": "Hello, World!"
    }),
);

// Optional: add context
let intent = intent.with_context("Creating test file for demonstration");

// Convert to JSON-RPC
let jsonrpc = serde_json::to_string(&intent)?;
```

### G2A_VERDICT

```rust
use aeon_engine::a2g::{G2aVerdict, Verdict, RiskLevel};

// Approved verdict
let verdict = G2aVerdict::approved(
    "intent-123",
    "req-001",
    0.15,
    "write_file",
);

assert!(verdict.is_approved());

// Denied verdict
let verdict = G2aVerdict::denied(
    "intent-123",
    "req-001", 
    "High risk operation",
    0.92,
);

assert!(!verdict.is_approved());
```

### A2G_REPORT

```rust
use aeon_engine::a2g::{A2gReport, ExecutionStatus};

// Success report
let report = A2gReport::success(
    "did:aeon:myagent:1.0:abc",
    "intent-123",
    serde_json::json!({"bytes_written": 13}),
    45, // duration_ms
);

// Failure report
let report = A2gReport::failure(
    "did:aeon:myagent:1.0:abc",
    "intent-123",
    "File permission denied",
);
```

## Risk Assessment

```rust
use aeon_engine::a2g::{RiskLevel, RiskAssessment};

// Determine risk level from score
let level = RiskLevel::from_score(0.85);
assert_eq!(level, RiskLevel::High);

// Create risk assessment
let assessment = RiskAssessment {
    score: 0.15,
    level: RiskLevel::Low,
    model_score: Some(0.12),
    heuristic_score: Some(0.15),
    threats: vec![],
};
```

## Capability Manifest

```rust
use aeon_engine::a2g::CapabilityManifest;

// Get default manifest for a tool
let manifest = CapabilityManifest::for_tool("write_file");

// manifest.max_memory_mb = Some(50)
// manifest.timeout_seconds = Some(30)
// manifest.network_allowed = Some(false)
```

## Error Codes

```rust
use aeon_engine::a2g::error_codes;

match error_code {
    error_codes::POLICY_VIOLATION => "Blocked by policy",
    error_codes::EXECUTION_ERROR => "Execution failed",
    error_codes::CAPABILITY_EXHAUSTED => "Resource limit exceeded",
    _ => "Unknown error",
}
```

## Telemetry Signals

```rust
use aeon_engine::a2g::{TelemetrySignal, RiskAssessmentData};

let signal = TelemetrySignal::RiskAssessment(RiskAssessmentData {
    timestamp: Utc::now(),
    intent_id: "intent-123".to_string(),
    method: "write_file".to_string(),
    model_score: 0.12,
    heuristic_score: 0.15,
    final_score: 0.15,
    risk_level: RiskLevel::Low,
});
```

## Full Example

```rust
use aeon_engine::a2g::*;
use serde_json::json;

fn evaluate_intent(tool: &str, args: serde_json::Value) -> G2aVerdict {
    // Create intent
    let intent = A2gIntent::new("did:aeon:test:1.0", tool, args);
    
    // Score risk (simplified)
    let risk_score = score_tool_risk(tool);
    
    if risk_score >= 0.8 {
        G2aVerdict::denied(
            &intent.params.intent_id,
            &intent.id,
            "High risk operation blocked",
            risk_score,
        )
    } else {
        G2aVerdict::approved(
            &intent.params.intent_id,
            &intent.id,
            risk_score,
            tool,
        )
    }
}
```

## Source Code

Full implementation: [`aeon-engine/src/a2g.rs`](https://github.com/Vanaras-AI/AEON/blob/main/aeon-engine/src/a2g.rs)
