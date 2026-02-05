use wasmtime::{Caller, Linker};
use anyhow::{Result, Error};
use std::fs;
use std::path::Path;
use tracing::{info, warn, error};
use crate::state::AgentState;
use crate::Mandate;

// --- UTILS ---
// --- UTILS ---
// (Oracle removed, using Global Bridge)

pub fn test_oracle(prompt: &str) {
    println!("DEBUG: Inside test_oracle (Bridge Mode)");
    
    // First Call (Warmup / Server Spawn)
    println!("🧠 [NEURAL HANDSHAKE] Call 1 (Warmup)...");
    match crate::gemma::BRIDGE.infer(prompt) {
        Ok(response) => println!("🧠 [NEURAL HANDSHAKE] Response 1: '{}'", response.trim()),
        Err(e) => eprintln!("❌ [NEURAL HANDSHAKE] Failed 1: {}", e),
    }

    // Second Call (Latency Test)
    println!("🧠 [NEURAL HANDSHAKE] Call 2 (Latency Test)...");
    let start = std::time::Instant::now();
    match crate::gemma::BRIDGE.infer("Summarize 'The Matrix' in 5 words.") {
        Ok(response) => {
            let duration = start.elapsed();
            println!("🧠 [NEURAL HANDSHAKE] Response 2 ({:.2?}): '{}'", duration, response.trim());
        },
        Err(e) => eprintln!("❌ [NEURAL HANDSHAKE] Failed 2: {}", e),
    }
}

// Helper to calculate DID (duplicated to avoid circular dep for now, or import from main if pub)
fn compute_did(agent_id: &str, version: &str) -> String {
    use sha2::{Sha256, Digest};
    use crate::keyring::AeonKeyring;
    
    // Load sovereign key
    let pubkey = match AeonKeyring::init() {
        Ok(keyring) => keyring.public_key_hex(),
        Err(_) => {
            let input = format!("{}:{}",agent_id, version);
            let hash = Sha256::digest(input.as_bytes());
            format!("{:x}", hash)
        }
    };
    format!("did:aeon:{}:{}:{}", agent_id, version, pubkey)
}

pub fn register_functions(linker: &mut Linker<AgentState>) -> Result<()> {

    // --- AI GOVERNANCE (The Cortical Implant) ---
    linker.func_wrap("aeon", "aeon_governance_inference", |mut caller: Caller<'_, AgentState>, ptr: u32, len: u32, out_ptr: u32, out_max_len: u32| {
        let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
        let mut buffer = vec![0u8; len as usize];
        memory.read(&caller, ptr as usize, &mut buffer).map_err(|e| anyhow::anyhow!("Read failed: {}", e))?;
        
        let prompt = match std::str::from_utf8(&buffer) {
            Ok(s) => s,
            Err(_) => return Ok(0u32),
        };

        if !caller.data().mandate.permissions.contains(&"GOVERNANCE_ORACLE".to_string()) {
            println!("🛑 [GOVERNANCE] Blocked ORACLE access for cell: {}", caller.data().mandate.agent_id);
            return Ok(0u32); 
        }

        // Call Gemma via Bridge
        // println!("🧠 [GEMMA] Analyzing scenario: {:.50}...", prompt);
        let response = crate::gemma::BRIDGE.infer(prompt).unwrap_or_else(|e| format!("{{ \"error\": \"{}\" }}", e));
        
        let bytes = response.as_bytes();
        let copy_len = bytes.len().min(out_max_len as usize);
        memory.write(&mut caller, out_ptr as usize, &bytes[..copy_len]).unwrap();
        
        Ok(copy_len as u32)
    })?;

    // --- NETWORKING ---
    linker.func_wrap("aeon", "net_skill", |caller: Caller<'_, AgentState>, _url_ptr: u32| {
        if !caller.data().mandate.permissions.contains(&"NET".to_string()) {
            println!("🛑 [GOVERNANCE] Blocked NET access for cell: {}", caller.data().mandate.agent_id);
            return Ok(1u32); 
        }
        println!("✅ [GOVERNANCE] NET authorized for cell: {}", caller.data().mandate.agent_id);
        Ok(0u32)
    })?;

    // --- INTROSPECTION ---
    linker.func_wrap("aeon", "get_dna", |mut caller: Caller<'_, AgentState>, ptr: u32, len: u32| {
        let mandate_json = serde_json::to_string(&caller.data().mandate).unwrap();
        let bytes = mandate_json.as_bytes();
        let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
        if bytes.len() > len as usize { return Err(Error::msg("DNA Buffer Too Small")); }
        memory.write(&mut caller, ptr as usize, bytes).unwrap();
        Ok(bytes.len() as u32)
    })?;

    // --- MITOSIS (The Void Check Protected) ---
    linker.func_wrap("aeon", "spawn_cell", |mut caller: Caller<'_, AgentState>, ptr: u32, len: u32| {
        if !caller.data().mandate.permissions.contains(&"MITOSIS".to_string()) {
            warn!(cell = %caller.data().mandate.agent_id, "🛑 [GOVERNANCE] Mitosis blocked");
            return Ok(1u32);
        }
        let memory = caller.get_export("memory").ok_or(anyhow::anyhow!("Memory not found"))?.into_memory().ok_or(anyhow::anyhow!("Memory export not a memory"))?;
        let mut buffer = vec![0u8; len as usize];
        memory.read(&caller, ptr as usize, &mut buffer).map_err(|e| anyhow::anyhow!("Memory read failed: {}", e))?;
        
        // [SAFETY] Verify UTF-8 string
        let dna_json = match std::str::from_utf8(&buffer) {
            Ok(s) => s,
            Err(_) => {
                warn!("⚠️ [MITOSIS] Guest sent invalid UTF-8 DNA");
                return Ok(1u32);
            }
        };

        // [SAFETY] Safe Deserialization
        let mut child_mandate: Mandate = match serde_json::from_str(dna_json) {
            Ok(m) => m,
            Err(e) => {
                warn!(error = %e, "⚠️ [MITOSIS] Invalid DNA JSON");
                return Ok(1u32);
            }
        };
        
        // --- IDENTITY DERIVATION ---
        child_mandate.version = "1.0.0".to_string(); 
        child_mandate.did = compute_did(&child_mandate.agent_id, &child_mandate.version);
        
        // [SECURITY] Phase 1: The Law of Conservation (Permission Inheritance)
        let parent_permissions = &caller.data().mandate.permissions;
        for perm in &child_mandate.permissions {
            if !parent_permissions.contains(perm) {
                warn!(parent = %caller.data().mandate.agent_id, child = %child_mandate.agent_id, permission = %perm, "🛑 [SECURITY] Privilege Escalation Attempt: Child requested permission not held by Parent");
                return Ok(1u32);
            }
        }

        info!(parent = %caller.data().mandate.agent_id, child = %child_mandate.agent_id, did = %child_mandate.did, "🧬 [MITOSIS] Spawning child");
        let dna_path = format!("../mandates/{}.toml", child_mandate.agent_id);
        
        // [SECURITY] Phase 2: The Law of Identity (Anti-Overwrite)
        if Path::new(&dna_path).exists() {
             warn!(parent = %caller.data().mandate.agent_id, target = %child_mandate.agent_id, "🛑 [SECURITY] Identity Theft Attempt: Mandate already exists");
             return Ok(1u32);
        }

        let dna_toml = format!("agent_id = \"{}\"\nversion = \"{}\"\ndid = \"{}\"\npermissions = {:?}\nsubscriptions = {:?}\n", child_mandate.agent_id, child_mandate.version, child_mandate.did, child_mandate.permissions, child_mandate.subscriptions);
        
        if let Err(e) = fs::write(&dna_path, dna_toml) {
             error!(error = %e, "❌ [MITOSIS] Failed to write DNA file");
             return Ok(1u32);
        }
        Ok(0u32)
    })?;

    // --- FILESYSTEM ---
    linker.func_wrap("aeon", "read_range", |mut caller: Caller<'_, AgentState>, path_ptr: u32, path_len: u32, start: u32, end: u32, out_ptr: u32, out_max_len: u32| {
        let memory = caller.get_export("memory").ok_or(anyhow::anyhow!("Memory missing"))?.into_memory().ok_or(anyhow::anyhow!("Not memory"))?;
        let mut path_bytes = vec![0u8; path_len as usize];
        memory.read(&caller, path_ptr as usize, &mut path_bytes).map_err(|e| anyhow::anyhow!("Mem read failed: {}", e))?;
        
        let path = match std::str::from_utf8(&path_bytes) {
            Ok(s) => s,
            Err(_) => { println!("⚠️ [FS_READ] Invalid UTF-8 path"); return Ok(0u32); }
        };

        if !caller.data().mandate.permissions.contains(&"FS_READ".to_string()) {
            println!("🛑 [GOVERNANCE] Blocked READ_RANGE for cell: {}.", caller.data().mandate.agent_id);
            return Ok(0u32);
        }
        
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => return Ok(0u32),
        };

        let lines: Vec<&str> = content.lines().collect();
        let start_idx = (start.saturating_sub(1)) as usize;
        let end_idx = (end as usize).min(lines.len());
        if start_idx >= lines.len() || start_idx >= end_idx { return Ok(0u32); }
        let selected_lines = lines[start_idx..end_idx].join("\n");
        let bytes = selected_lines.as_bytes();
        let copy_len = bytes.len().min(out_max_len as usize);
        
        if let Err(e) = memory.write(&mut caller, out_ptr as usize, &bytes[..copy_len]) {
             println!("⚠️ [FS_READ] Failed to write to guest memory: {}", e);
             return Ok(0u32);
        }
        Ok(copy_len as u32)
    })?;

    linker.func_wrap("aeon", "replace_block", |mut caller: Caller<'_, AgentState>, path_ptr: u32, path_len: u32, start: u32, end: u32, content_ptr: u32, content_len: u32| {
        let memory = caller.get_export("memory").ok_or(anyhow::anyhow!("Memory missing"))?.into_memory().ok_or(anyhow::anyhow!("Not memory"))?;
        let mut path_bytes = vec![0u8; path_len as usize];
        memory.read(&caller, path_ptr as usize, &mut path_bytes).map_err(|e| anyhow::anyhow!("Mem read failed: {}", e))?;
        
        let path = match std::str::from_utf8(&path_bytes) {
             Ok(s) => s,
             Err(_) => return Ok(1u32),
        };

        // Territoriality check
        let territory = &caller.data().mandate.territory;
        let is_authorized = territory.is_empty() || territory.iter().any(|t| path.starts_with(t));
        
        if !is_authorized {
            println!("🛑 [TERRITORY] Cell {} attempted unauthorized access to: {}", caller.data().mandate.agent_id, path);
            return Ok(1u32);
        }

        if !caller.data().mandate.permissions.contains(&"FS_WRITE".to_string()) {
            println!("🛑 [GOVERNANCE] Blocked REPLACE_BLOCK for cell: {}.", caller.data().mandate.agent_id);
            return Ok(1u32);
        }
        let mut new_content_bytes = vec![0u8; content_len as usize];
        if memory.read(&caller, content_ptr as usize, &mut new_content_bytes).is_err() { return Ok(1u32); }
        
        let new_lines_str = match std::str::from_utf8(&new_content_bytes) {
            Ok(s) => s,
            Err(_) => return Ok(1u32),
        };

        let content = fs::read_to_string(path).unwrap_or_default();
        let mut lines: Vec<String> = if content.is_empty() { Vec::new() } else { content.lines().map(|s| s.to_string()).collect() };
        
        let start_idx = (start.saturating_sub(1)) as usize;
        let end_idx = (end as usize).min(lines.len());
        
        if start_idx <= end_idx {
             // Replace logic
             lines.drain(start_idx..end_idx);
             if !new_lines_str.is_empty() {
                 lines.insert(start_idx, new_lines_str.to_string());
             }
        }
        
        let new_content = lines.join("\n");
        if fs::write(path, new_content).is_err() { return Ok(1u32); }
        
        Ok(0u32)
    })?;

    linker.func_wrap("aeon", "write_file", |mut caller: Caller<'_, AgentState>, path_ptr: u32, path_len: u32, content_ptr: u32, content_len: u32| {
        let memory = caller.get_export("memory").ok_or(anyhow::anyhow!("Memory missing"))?.into_memory().ok_or(anyhow::anyhow!("Not memory"))?;
        let mut path_bytes = vec![0u8; path_len as usize];
        memory.read(&caller, path_ptr as usize, &mut path_bytes).map_err(|e| anyhow::anyhow!("Mem read failed: {}", e))?;
        
        let path = match std::str::from_utf8(&path_bytes) {
             Ok(s) => s,
             Err(_) => return Ok(1u32),
        };

        // Canonicalization Check (Anti-Symlink)
        let abs_path = match fs::canonicalize(Path::new(path)) {
            Ok(p) => p,
            Err(_) => {
                // If file doesn't exist, we check parent
                let p = Path::new(path);
                if let Some(parent) = p.parent() {
                    if let Ok(canon_parent) = fs::canonicalize(parent) {
                        canon_parent.join(p.file_name().unwrap())
                    } else {
                        Path::new(path).to_path_buf()
                    }
                } else {
                    Path::new(path).to_path_buf()
                }
            }
        };
        let abs_path_str = abs_path.to_string_lossy();
        let path_str = path; 

        // [SECURITY] Keyring Defense (Sprint 4.7)
        if abs_path_str.contains("/.aeon") || path_str.contains(".aeon") {
             error!(cell = %caller.data().mandate.agent_id, path = %path, "🛑 [SECURITY] BLOCKED: Attempted to write to System Internals (.aeon)");
             return Ok(1u32);
        }

        // Territoriality check
        let territory = &caller.data().mandate.territory;
        let is_authorized = territory.is_empty() || territory.iter().any(|t| abs_path_str.starts_with(t));

        if !is_authorized {
            println!("🛑 [TERRITORY] Cell {} attempted unauthorized access to: {}", caller.data().mandate.agent_id, path);
            return Ok(1u32);
        }

        // [SECURITY] Mandate Protection (Sprint 4.6 & 4.8)
        if abs_path_str.contains("/mandates/") {
             println!("🛑 [SECURITY] Cell {} attempted to overwrite Mandate: {}", caller.data().mandate.agent_id, path);
             return Ok(1u32);
        }

        if !caller.data().mandate.permissions.contains(&"FS_WRITE".to_string()) {
            println!("🛑 [GOVERNANCE] Blocked WRITE_FILE for cell: {}.", caller.data().mandate.agent_id);
            return Ok(1u32);
        }

        let mut content_bytes = vec![0u8; content_len as usize];
        if memory.read(&caller, content_ptr as usize, &mut content_bytes).is_err() { return Ok(1u32); }
        
        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent).unwrap_or(());
        }

        if fs::write(path, &content_bytes).is_err() { return Ok(1u32); }
        Ok(0u32)
    })?;

    Ok(())
}
