//! A2G Protocol - Agent-to-Governance Protocol Implementation
//! 
//! This module defines the formal protocol types for communication between
//! AI agents and the AEON governance system.
//!
//! # Message Flow
//! ```text
//! Agent → Governance: A2G_INTENT → G2A_VERDICT → Execute → A2G_REPORT
//! ```

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

// =============================================================================
// A2G MESSAGE TYPES (Agent → Governance)
// =============================================================================

/// A2G_INTENT: Agent requests permission to perform an action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct A2gIntent {
    pub jsonrpc: String,
    pub method: String, // "a2g/intent"
    pub params: IntentParams,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentParams {
    pub agent_did: String,
    pub intent_id: String,
    pub tool: String,
    pub arguments: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<IntentContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentContext {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_intent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<String>,
}

impl A2gIntent {
    pub fn new(agent_did: &str, tool: &str, arguments: serde_json::Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            method: "a2g/intent".to_string(),
            params: IntentParams {
                agent_did: agent_did.to_string(),
                intent_id: Uuid::new_v4().to_string(),
                tool: tool.to_string(),
                arguments,
                context: None,
            },
            id: Uuid::new_v4().to_string(),
        }
    }

    pub fn with_context(mut self, reasoning: &str) -> Self {
        self.params.context = Some(IntentContext {
            session_id: None,
            parent_intent: None,
            reasoning: Some(reasoning.to_string()),
        });
        self
    }
}

/// A2G_REPORT: Agent reports execution outcome
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct A2gReport {
    pub jsonrpc: String,
    pub method: String, // "a2g/report"
    pub params: ReportParams,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportParams {
    pub agent_did: String,
    pub intent_id: String,
    pub status: ExecutionStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<ExecutionMetrics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExecutionStatus {
    Success,
    Failure,
    Timeout,
    Aborted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionMetrics {
    pub duration_ms: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_used_mb: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_percent: Option<f32>,
}

impl A2gReport {
    pub fn success(agent_did: &str, intent_id: &str, result: serde_json::Value, duration_ms: u64) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            method: "a2g/report".to_string(),
            params: ReportParams {
                agent_did: agent_did.to_string(),
                intent_id: intent_id.to_string(),
                status: ExecutionStatus::Success,
                result: Some(result),
                metrics: Some(ExecutionMetrics {
                    duration_ms,
                    memory_used_mb: None,
                    cpu_percent: None,
                }),
                error: None,
            },
            id: Uuid::new_v4().to_string(),
        }
    }

    pub fn failure(agent_did: &str, intent_id: &str, error: &str) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            method: "a2g/report".to_string(),
            params: ReportParams {
                agent_did: agent_did.to_string(),
                intent_id: intent_id.to_string(),
                status: ExecutionStatus::Failure,
                result: None,
                metrics: None,
                error: Some(error.to_string()),
            },
            id: Uuid::new_v4().to_string(),
        }
    }
}

/// A2G_REGISTER: Agent registers with governance on startup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct A2gRegister {
    pub jsonrpc: String,
    pub method: String, // "a2g/register"
    pub params: RegisterParams,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterParams {
    pub agent_did: String,
    pub public_key: String,
    pub capabilities_requested: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<AgentMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMetadata {
    pub name: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
}

// =============================================================================
// G2A MESSAGE TYPES (Governance → Agent)
// =============================================================================

/// G2A_VERDICT: Governance responds with approval or denial
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct G2aVerdict {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<VerdictResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<VerdictError>,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerdictResult {
    pub verdict: Verdict,
    pub intent_id: String,
    pub risk_assessment: RiskAssessment,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability_manifest: Option<CapabilityManifest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Verdict {
    Approved,
    Denied,
    Escalate,
    Conditional,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerdictError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub score: f32,
    pub level: RiskLevel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_score: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heuristic_score: Option<f32>,
    #[serde(default)]
    pub threats: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RiskLevel {
    Critical,
    High,
    Medium,
    Low,
}

impl RiskLevel {
    pub fn from_score(score: f32) -> Self {
        match score {
            s if s >= 0.9 => RiskLevel::Critical,
            s if s >= 0.7 => RiskLevel::High,
            s if s >= 0.4 => RiskLevel::Medium,
            _ => RiskLevel::Low,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityManifest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_memory_mb: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_cpu_percent: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_allowed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filesystem_scope: Option<Vec<String>>,
}

impl CapabilityManifest {
    pub fn for_tool(tool: &str) -> Self {
        match tool {
            "write_file" => Self {
                max_memory_mb: Some(50),
                max_cpu_percent: Some(10),
                timeout_seconds: Some(30),
                network_allowed: Some(false),
                filesystem_scope: Some(vec!["/workspace/**".to_string(), "/tmp/**".to_string()]),
            },
            "read_file" => Self {
                max_memory_mb: Some(100),
                max_cpu_percent: Some(10),
                timeout_seconds: Some(30),
                network_allowed: Some(false),
                filesystem_scope: None,
            },
            "execute_command" => Self {
                max_memory_mb: Some(256),
                max_cpu_percent: Some(50),
                timeout_seconds: Some(60),
                network_allowed: Some(true),
                filesystem_scope: None,
            },
            _ => Self {
                max_memory_mb: Some(50),
                max_cpu_percent: Some(25),
                timeout_seconds: Some(30),
                network_allowed: Some(false),
                filesystem_scope: None,
            },
        }
    }
}

/// G2A_POLICY: Governance sends current capabilities to agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct G2aPolicy {
    pub jsonrpc: String,
    pub method: String, // "g2a/policy"
    pub params: PolicyParams,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyParams {
    pub agent_did: String,
    pub version: String,
    pub capabilities: PolicyCapabilities,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constitution_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<std::collections::HashMap<String, ToolPolicy>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<NetworkPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<ResourceLimits>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolPolicy {
    pub allowed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPolicy {
    #[serde(default)]
    pub allowed_domains: Vec<String>,
    #[serde(default)]
    pub blocked_domains: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_requests_per_minute: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_memory_mb: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_cpu_percent: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_disk_mb: Option<u32>,
}

// =============================================================================
// TELEMETRY SIGNALS
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TelemetrySignal {
    IntentReceived(IntentReceivedData),
    RiskAssessment(RiskAssessmentData),
    IntentBlocked(IntentBlockedData),
    IntentAllowed(IntentAllowedData),
    ExecutionComplete(ExecutionCompleteData),
    Heartbeat(HeartbeatData),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentReceivedData {
    pub timestamp: DateTime<Utc>,
    pub intent_id: String,
    pub method: String,
    pub agent_did: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessmentData {
    pub timestamp: DateTime<Utc>,
    pub intent_id: String,
    pub method: String,
    pub model_score: f32,
    pub heuristic_score: f32,
    pub final_score: f32,
    pub risk_level: RiskLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentBlockedData {
    pub timestamp: DateTime<Utc>,
    pub intent_id: String,
    pub method: String,
    pub reason: String,
    pub risk_score: f32,
    pub blocked_at_phase: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentAllowedData {
    pub timestamp: DateTime<Utc>,
    pub intent_id: String,
    pub method: String,
    pub risk_score: f32,
    pub risk_level: RiskLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionCompleteData {
    pub timestamp: DateTime<Utc>,
    pub intent_id: String,
    pub method: String,
    pub duration_ms: u64,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartbeatData {
    pub timestamp: DateTime<Utc>,
    pub status: String,
    pub load: f32,
}

// =============================================================================
// ERROR CODES
// =============================================================================

pub mod error_codes {
    pub const PARSE_ERROR: i32 = -32700;
    pub const INVALID_REQUEST: i32 = -32600;
    pub const METHOD_NOT_FOUND: i32 = -32601;
    pub const INVALID_PARAMS: i32 = -32602;
    pub const POLICY_VIOLATION: i32 = -32000;
    pub const EXECUTION_ERROR: i32 = -32001;
    pub const REGISTRATION_FAILED: i32 = -32002;
    pub const CAPABILITY_EXHAUSTED: i32 = -32003;
    pub const SESSION_EXPIRED: i32 = -32004;
}

// =============================================================================
// HELPER FUNCTIONS
// =============================================================================

impl G2aVerdict {
    /// Create an approved verdict
    pub fn approved(
        intent_id: &str,
        request_id: &str,
        risk_score: f32,
        tool: &str,
    ) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result: Some(VerdictResult {
                verdict: Verdict::Approved,
                intent_id: intent_id.to_string(),
                risk_assessment: RiskAssessment {
                    score: risk_score,
                    level: RiskLevel::from_score(risk_score),
                    model_score: None,
                    heuristic_score: Some(risk_score),
                    threats: vec![],
                },
                capability_manifest: Some(CapabilityManifest::for_tool(tool)),
                conditions: None,
                expires_at: Some(Utc::now() + chrono::Duration::minutes(5)),
            }),
            error: None,
            id: request_id.to_string(),
        }
    }

    /// Create a denied verdict
    pub fn denied(intent_id: &str, request_id: &str, reason: &str, risk_score: f32) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result: Some(VerdictResult {
                verdict: Verdict::Denied,
                intent_id: intent_id.to_string(),
                risk_assessment: RiskAssessment {
                    score: risk_score,
                    level: RiskLevel::from_score(risk_score),
                    model_score: None,
                    heuristic_score: Some(risk_score),
                    threats: vec![reason.to_string()],
                },
                capability_manifest: None,
                conditions: None,
                expires_at: None,
            }),
            error: Some(VerdictError {
                code: error_codes::POLICY_VIOLATION,
                message: reason.to_string(),
                data: Some(serde_json::json!({
                    "risk_score": risk_score,
                    "blocked_by": "governance_loop"
                })),
            }),
            id: request_id.to_string(),
        }
    }

    /// Check if verdict is approved
    pub fn is_approved(&self) -> bool {
        self.result.as_ref().map_or(false, |r| r.verdict == Verdict::Approved)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intent_creation() {
        let intent = A2gIntent::new(
            "did:aeon:test:1.0:abc",
            "write_file",
            serde_json::json!({"path": "/tmp/test.txt", "content": "hello"}),
        );
        
        assert_eq!(intent.method, "a2g/intent");
        assert_eq!(intent.params.tool, "write_file");
    }

    #[test]
    fn test_verdict_approved() {
        let verdict = G2aVerdict::approved("intent-1", "req-1", 0.2, "write_file");
        assert!(verdict.is_approved());
        assert!(verdict.result.unwrap().capability_manifest.is_some());
    }

    #[test]
    fn test_verdict_denied() {
        let verdict = G2aVerdict::denied("intent-1", "req-1", "High risk", 0.9);
        assert!(!verdict.is_approved());
        assert!(verdict.error.is_some());
    }

    #[test]
    fn test_risk_level_from_score() {
        assert_eq!(RiskLevel::from_score(0.95), RiskLevel::Critical);
        assert_eq!(RiskLevel::from_score(0.75), RiskLevel::High);
        assert_eq!(RiskLevel::from_score(0.5), RiskLevel::Medium);
        assert_eq!(RiskLevel::from_score(0.2), RiskLevel::Low);
    }
}
