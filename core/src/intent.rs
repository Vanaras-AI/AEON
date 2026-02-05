use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    pub agent_id: String,
    pub action: String,
    pub parameters: Value,
}

pub enum IntentError {
    InvalidSchema(String),
    UnregisteredAction(String),
}

pub struct IntentGateway {
    // Registry of allowed MCP actions
    registry: Vec<String>,
}

impl IntentGateway {
    pub fn new(registry: Vec<String>) -> Self {
        Self { registry }
    }

    pub fn validate(&self, raw: &str) -> Result<Intent, IntentError> {
        let intent: Intent = serde_json::from_str(raw)
            .map_err(|e| IntentError::InvalidSchema(e.to_string()))?;
        
        if !self.registry.contains(&intent.action) {
            return Err(IntentError::UnregisteredAction(intent.action));
        }

        Ok(intent)
    }
}
