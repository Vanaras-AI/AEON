use serde::{Deserialize, Serialize};
use uuid;
use chrono;

// A2A Standard Signal (JSON-RPC 2.0)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Signal {
    pub jsonrpc: String,
    pub method: String,
    pub params: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<serde_json::Value>,
    #[serde(default)]
    pub source_did: String,
    #[serde(default)]
    pub timestamp: String,
}

impl Signal {
    pub fn new(method: &str, params: serde_json::Value, source_did: &str) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            params,
            id: Some(serde_json::json!(uuid::Uuid::new_v4().to_string())),
            source_did: source_did.to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Command {
    Halt,
    Inject(Signal),
}
