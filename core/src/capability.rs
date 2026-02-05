use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Capability {
    FileRead(String),
    FileWrite(String),
    FileReadPattern(String),  // e.g., "/tmp/*.txt"
    NetworkConnect(String, u16),
    NetworkDeny,
    ProcessSpawn(String),
    MemoryLimit(usize),  // bytes
    CpuLimit(f32),       // percentage
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityManifest {
    pub capabilities: Vec<Capability>,
    pub max_memory_bytes: usize,
    pub max_cpu_percent: f32,
    pub allow_network: bool,
}

impl CapabilityManifest {
    /// Build a capability manifest for a given intent
    pub fn build(method: &str, params: &serde_json::Value) -> Self {
        let mut caps = Vec::new();
        let mut max_memory = 100 * 1024 * 1024; // 100MB default
        let max_cpu = 50.0; // 50% default
        let mut allow_network = false;
        
        match method {
            "write_file" => {
                if let Some(path) = params.get("path").and_then(|p| p.as_str()) {
                    caps.push(Capability::FileWrite(path.to_string()));
                }
                caps.push(Capability::NetworkDeny);
                max_memory = 10 * 1024 * 1024; // 10MB for file writes
            },
            "read_file" => {
                if let Some(path) = params.get("path").and_then(|p| p.as_str()) {
                    caps.push(Capability::FileRead(path.to_string()));
                }
                caps.push(Capability::NetworkDeny);
                max_memory = 50 * 1024 * 1024; // 50MB for file reads
            },
            "execute_command" => {
                // More permissive for commands
                caps.push(Capability::FileReadPattern("/tmp/*".to_string()));
                caps.push(Capability::FileReadPattern("/home/*".to_string()));
                allow_network = true; // Commands may need network
                max_memory = 100 * 1024 * 1024; // 100MB for commands
            },
            "list_directory" => {
                if let Some(path) = params.get("path").and_then(|p| p.as_str()) {
                    caps.push(Capability::FileReadPattern(format!("{}/*", path)));
                }
                caps.push(Capability::NetworkDeny);
                max_memory = 20 * 1024 * 1024;
            },
            _ => {
                // Unknown methods get minimal permissions
                caps.push(Capability::NetworkDeny);
                max_memory = 10 * 1024 * 1024;
            }
        }
        
        Self {
            capabilities: caps,
            max_memory_bytes: max_memory,
            max_cpu_percent: max_cpu,
            allow_network,
        }
    }
    
    /// Check if a file write is allowed by this manifest
    pub fn allows_file_write(&self, path: &str) -> bool {
        self.capabilities.iter().any(|cap| {
            matches!(cap, Capability::FileWrite(p) if p == path)
        })
    }
    
    /// Check if a file read is allowed by this manifest
    pub fn allows_file_read(&self, path: &str) -> bool {
        self.capabilities.iter().any(|cap| match cap {
            Capability::FileRead(p) if p == path => true,
            Capability::FileReadPattern(pattern) => {
                // Simple glob matching (just * for now)
                if pattern.ends_with("/*") {
                    let prefix = &pattern[..pattern.len() - 2];
                    path.starts_with(prefix)
                } else {
                    pattern == path
                }
            },
            _ => false,
        })
    }
    
    /// Check if network access is allowed
    pub fn allows_network(&self) -> bool {
        self.allow_network && !self.capabilities.iter().any(|cap| {
            matches!(cap, Capability::NetworkDeny)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_write_file_manifest() {
        let params = json!({"path": "/tmp/test.txt"});
        let manifest = CapabilityManifest::build("write_file", &params);
        
        assert!(manifest.allows_file_write("/tmp/test.txt"));
        assert!(!manifest.allows_file_write("/tmp/other.txt"));
        assert!(!manifest.allows_network());
    }

    #[test]
    fn test_execute_command_manifest() {
        let params = json!({"command": "ls /tmp"});
        let manifest = CapabilityManifest::build("execute_command", &params);
        
        assert!(manifest.allows_file_read("/tmp/file.txt"));
        assert!(manifest.allows_network());
    }
}
