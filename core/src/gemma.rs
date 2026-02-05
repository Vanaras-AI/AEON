use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

// Wrapper for the Bridge Server Process
pub struct GemmaBridge {
    process: Mutex<Option<Child>>,
    url: String,
}

#[derive(Serialize)]
struct InferenceRequest {
    prompt: String,
}

#[derive(Deserialize, Debug)]
pub struct InferenceResponse {
    pub response: String,
}

// Governance Decision Structure (JSON Output from SLM)
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GovernanceDecision {
    pub decision: String,   // "COMPLIANT" | "NON_COMPLIANT"
    pub risk_level: String, // "LOW" | "MEDIUM" | "HIGH" | "CRITICAL"
    pub required_actions: Vec<String>,
}

impl GemmaBridge {
    pub fn new() -> Self {
        Self {
            process: Mutex::new(None),
            url: "http://127.0.0.1:8000/infer".to_string(),
        }
    }

    pub fn start_server(&self) -> Result<()> {
        let mut guard = self.process.lock().unwrap();
        if guard.is_some() {
            return Ok(()); // Already running
        }

        println!("🧠 [GEMMA] Spawning Bridge Server (governance_model/bridge_server.py)...");
        
        let child = Command::new("python3")
            .arg("../governance_model/bridge_server.py")
            .stdout(Stdio::inherit()) // Pipe stdout to parent
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|e| anyhow!("Failed to spawn bridge server: {}", e))?;

        *guard = Some(child);
        
        // Wait for server to be ready (naive sleep, better to poll health)
        println!("⏳ [GEMMA] Waiting for Bridge Server to warm up...");
        std::thread::sleep(Duration::from_secs(5));
        
        Ok(())
    }

    pub fn kill_server(&self) {
        let mut guard = self.process.lock().unwrap();
        if let Some(mut child) = guard.take() {
            println!("💀 [GEMMA] Killing Bridge Server...");
            let _ = child.kill();
        }
    }

    pub fn infer(&self, prompt: &str) -> Result<String> {
        let req = InferenceRequest { prompt: prompt.to_string() };
        
        let response: InferenceResponse = ureq::post(&self.url)
            .send_json(serde_json::to_value(&req)?)?
            .into_json()?;
            
        Ok(response.response)
    }

    pub fn assess_governance(&self, scenario: &str) -> Result<GovernanceDecision> {
        let raw_json = self.infer(scenario)?;
        
        // Attempt to parse the JSON string returned by the generic inference
        let decision: GovernanceDecision = serde_json::from_str(&raw_json)
             .map_err(|e| anyhow!("Failed to parse Governance JSON: {} \nRaw: {}", e, raw_json))?;
             
        Ok(decision)
    }
}

// Global Singleton for the Bridge
use lazy_static::lazy_static;
lazy_static! {
    pub static ref BRIDGE: GemmaBridge = GemmaBridge::new();
}
