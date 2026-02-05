pub enum PolicyDecision {
    Allow,
    Deny(String),
}

pub struct PolicyEngine {
    // Placeholder for static rules / OPA
}

impl PolicyEngine {
    pub fn new() -> Self {
        Self {}
    }

    /// Phase 2: Deterministic Policy
    pub fn evaluate_static(&self, intent: &super::intent::Intent) -> PolicyDecision {
        // Example Rule: Block /etc access
        if let Some(path) = intent.parameters.get("path") {
            if path.as_str().unwrap_or("").starts_with("/etc") {
                return PolicyDecision::Deny("Access to /etc is forbidden".to_string());
            }
        }
        PolicyDecision::Allow
    }

    /// Phase 3: Model-Assisted Evaluation (Advisory)
    pub async fn evaluate_risk(&self, intent: &super::intent::Intent) -> u8 {
        // This will call gemma3.rs in the future
        // 0 = Low Risk, 100 = Critical
        0 
    }
}
