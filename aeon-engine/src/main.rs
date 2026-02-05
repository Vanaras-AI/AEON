use std::env;
use std::process::{Command, Stdio};
use std::fs::{self, File};
use std::io::{BufReader, BufRead, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::os::unix::io::{IntoRawFd, FromRawFd};
// Remove unused import if not used yet, but keeping for compatibility
// use std::time::Duration; // Removed unused

// WASMTIME for V5 Subagent (v16.0 stable)
use wasmtime::*;
use wasmtime_wasi::sync::{WasiCtxBuilder, add_to_linker};
use wasmtime_wasi::WasiCtx;
use os_pipe::{pipe, PipeReader, PipeWriter};
use serde_json::Value; // Added serde_json for policy check

mod synapse;
mod server;
mod risk_scorer;

use crate::synapse::{Signal, Command as AeonCommand};
use crate::risk_scorer::RiskScorer;
use tokio::sync::{broadcast, mpsc};

// Hardening Constants
const MAX_BODY_SIZE: usize = 10 * 1024 * 1024; // 10MB
const RECYCLE_LIMIT: usize = 50; // Restart after 50 calls

struct WasmState {
    ctx: WasiCtx,
}

// =============================================================================
// HARDENED RESIDENT SUBAGENT (PHASE 11 + 12 Lite)
// =============================================================================

struct InnerAgent {
    input_writer: PipeWriter,
    output_reader: BufReader<PipeReader>,
}

#[derive(Clone)]
struct ResidentSubagent {
    wasm_path: PathBuf,
    sandbox_path: PathBuf,
    inner: Arc<Mutex<InnerAgent>>,
    call_count: Arc<Mutex<usize>>,
    telemetry_tx: broadcast::Sender<Signal>, // [NEW] Telemetry Link
}

impl ResidentSubagent {
    /// Spawns the initial subagent and returns the handle
    fn spawn(wasm_path: &Path, sandbox_path: &Path, telemetry_tx: broadcast::Sender<Signal>) -> Result<Self, Box<dyn std::error::Error>> {
        let inner = Self::spawn_process(wasm_path, sandbox_path)?;
        
        Ok(ResidentSubagent {
            wasm_path: wasm_path.to_path_buf(),
            sandbox_path: sandbox_path.to_path_buf(),
            inner: Arc::new(Mutex::new(inner)),
            call_count: Arc::new(Mutex::new(0)),
            telemetry_tx,
        })
    }

    /// Internal helper to actually spawn the thread and pipes
    fn spawn_process(wasm_path: &Path, sandbox_path: &Path) -> Result<InnerAgent, Box<dyn std::error::Error>> {
        // Create Pipes
        let (input_reader, input_writer) = pipe()?;
        let (output_reader, output_writer) = pipe()?;

        let wasm_path_clone = wasm_path.to_path_buf();
        let sandbox_path_clone = sandbox_path.to_path_buf();

        // Spawn WASM Thread
        std::thread::spawn(move || {
            let engine = Engine::default();
            // Load Module
            let module = match Module::from_file(&engine, &wasm_path_clone) {
                Ok(m) => m,
                Err(e) => { eprintln!("[WASM THREAD] Failed to load module: {}", e); return; }
            };

            let mut linker = Linker::new(&engine);
            if let Err(e) = add_to_linker(&mut linker, |s: &mut WasmState| &mut s.ctx) {
                eprintln!("[WASM THREAD] Linker error: {}", e); return;
            }

            // unsafe conversion for OS pipes -> CapFile -> WasiFile
            let stdin_fd = input_reader.into_raw_fd();
            let stdin_std = unsafe { File::from_raw_fd(stdin_fd) };
            let stdin_cap = cap_std::fs::File::from_std(stdin_std);
            let stdin_wasi = wasmtime_wasi::sync::file::File::from_cap_std(stdin_cap);

            let stdout_fd = output_writer.into_raw_fd();
            let stdout_std = unsafe { File::from_raw_fd(stdout_fd) };
            let stdout_cap = cap_std::fs::File::from_std(stdout_std);
            let stdout_wasi = wasmtime_wasi::sync::file::File::from_cap_std(stdout_cap);

            let mut wasi_builder = WasiCtxBuilder::new();
            wasi_builder.stdin(Box::new(stdin_wasi));
            wasi_builder.stdout(Box::new(stdout_wasi));

            if !sandbox_path_clone.exists() { let _ = fs::create_dir_all(&sandbox_path_clone); }
            let dir = cap_std::fs::Dir::open_ambient_dir(&sandbox_path_clone, cap_std::ambient_authority()).unwrap();
            let _ = wasi_builder.preopened_dir(dir, "/");

            let mut store = Store::new(&engine, WasmState {
                ctx: wasi_builder.build(),
            });

            let instance = match linker.instantiate(&mut store, &module) {
                Ok(i) => i,
                Err(e) => { eprintln!("[WASM THREAD] Instantiate Error: {}", e); return; }
            };
            
            let func = match instance.get_typed_func::<(), ()>(&mut store, "_start") {
                 Ok(f) => f,
                 Err(e) => { eprintln!("[WASM THREAD] _start missing: {}", e); return; }
            };

            // Run Forever (until pipe closes)
            if let Err(e) = func.call(&mut store, ()) {
                // eprintln!("[WASM THREAD] Execution Finished/Error: {}", e); 
                let _ = e; // suppress unused warning
            }
        });

        Ok(InnerAgent {
            input_writer,
            output_reader: BufReader::new(output_reader),
        })
    }

    /// Checks if we need to recycle the process, and does so if needed
    fn check_recycle(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut count = self.call_count.lock().unwrap();
        if *count >= RECYCLE_LIMIT {
            println!("♻️  Warden: Recycling Subagent (Limit {} reached)...", *count);
            
            // 2. Spawn new
            let new_inner = Self::spawn_process(&self.wasm_path, &self.sandbox_path)?;
            
            // 3. Replace
            let mut inner_guard = self.inner.lock().unwrap();
            *inner_guard = new_inner;
            
            // 4. Reset count
            *count = 0;
            println!("♻️  Warden: Subagent Respawned.");
        }
        Ok(())
    }

    fn handle_request(&self, req_text: &str) -> String {
        let req_val: Value = match serde_json::from_str(req_text) {
            Ok(v) => v,
            Err(e) => return format!("{{\"error\": \"Invalid JSON: {}\"}}", e),
        };

        let method = req_val["method"].as_str().unwrap_or("");
        let params = &req_val["params"];
        let id_val = &req_val["id"];

        // Extract tool name for MCP protocol
        let empty_args = serde_json::json!({});
        let tool_name = if method == "tools/call" {
            params.get("name").and_then(|n| n.as_str()).unwrap_or("unknown")
        } else {
            method
        };
        let tool_args = if method == "tools/call" {
            params.get("arguments").unwrap_or(&empty_args)
        } else {
            params
        };

        // Broadcast Attempt Signal
        let _ = self.telemetry_tx.send(Signal::new(
            "AUDIT_INTENT", 
            serde_json::json!({
                "method": tool_name,
                "params": tool_args,
                "status": "CHECKING"
            }), 
            "WARDEN"
        ));

        // PHASE 2: Static Policy Check
        if let Err(e) = Self::check_intent(req_text) {
            let _ = self.telemetry_tx.send(Signal::new(
                "INTENT_BLOCKED", 
                serde_json::json!({
                    "method": tool_name,
                    "reason": e,
                    "phase": "POLICY"
                }), 
                "WARDEN"
            ));

            return serde_json::to_string(&serde_json::json!({
                "jsonrpc": "2.0",
                "id": id_val,
                "error": { "code": -32603, "message": e }
            })).unwrap();
        }

        // PHASE 3: Advisory Model (Risk Scoring)
        let risk_score = RiskScorer::score_intent(tool_name, tool_args);
        let risk_level = RiskScorer::risk_level(risk_score);
        
        println!("🎯 Risk Score: {:.2} ({})", risk_score, risk_level);
        
        if risk_score >= 0.8 {
            let reason = format!(
                "High risk operation detected (score: {:.2}, level: {})", 
                risk_score, risk_level
            );
            
            let _ = self.telemetry_tx.send(Signal::new(
                "INTENT_BLOCKED", 
                serde_json::json!({
                    "method": tool_name,
                    "reason": reason,
                    "risk_score": risk_score,
                    "phase": "RISK_ASSESSMENT"
                }), 
                "WARDEN"
            ));

            return serde_json::to_string(&serde_json::json!({
                "jsonrpc": "2.0",
                "id": id_val,
                "error": { "code": -32603, "message": reason }
            })).unwrap();
        }
        
        if risk_score >= 0.5 {
            println!("⚠️  Governor: Medium risk operation (score: {:.2})", risk_score);
        }

        // PHASE 4: Build Capability Manifest
        // Note: Using core::capability requires adding it to aeon-engine/Cargo.toml
        // For now, just log what the manifest would be
        println!("📋 Capability Manifest: method={}, max_memory={}MB", 
            tool_name,
            match tool_name {
                "write_file" => 10,
                "read_file" => 50,
                "execute_command" => 100,
                _ => 10
            }
        );

        // Broadcast Allowed Signal
        let _ = self.telemetry_tx.send(Signal::new(
            "INTENT_ALLOWED", 
            serde_json::json!({
                "method": tool_name,
                "risk_score": risk_score,
                "risk_level": risk_level
            }), 
            "WARDEN"
        ));

        // PHASE 5: Execute in WASM (already implemented)
        let res = self.call_wasm(req_text);
        res
    }

    // Phase 12 Lite: Intent Checker (Fixed for MCP Protocol)
    fn check_intent(json_rpc: &str) -> Result<(), String> {
        println!("🔍 RAW JSON: {}", json_rpc); // DEBUG: Dump Raw JSON
        
        let v: Value = serde_json::from_str(json_rpc).map_err(|e| e.to_string())?;
        
        if let Some(json_method) = v.get("method").and_then(|m| m.as_str()) {
             // MCP requests use "tools/call" method, with actual tool name in params
             if json_method == "tools/call" {
                 if let Some(params) = v.get("params") {
                     if let Some(tool_name) = params.get("name").and_then(|n| n.as_str()) {
                         println!("🔍 Governor: Checking Tool='{}'", tool_name); // DEBUG
                         
                         let tool_args = params.get("arguments");
                         
                         // RULE 1: Filesystem Policy
                         if tool_name == "write_file" {
                             if let Some(args) = tool_args {
                                 if let Some(path) = args.get("path").and_then(|p| p.as_str()) {
                                     println!("🔍 Governor: Checking Path='{}'", path); // DEBUG
                                     if path.starts_with("/etc") {
                                         println!("🛑 Governor: BLOCKING /etc access"); // DEBUG
                                         return Err(format!("Governor Denied: Access to {} is forbidden (Sensitive Directory)", path));
                                     }
                                     if path.contains("..") {
                                         return Err(format!("Governor Denied: Access to {} is forbidden (Path Traversal)", path));
                                     }
                                 }
                             }
                         }

                        // RULE 2: Execution Policy
                        if tool_name == "execute_command" {
                             if let Some(args) = tool_args {
                                if let Some(cmd) = args.get("command").and_then(|c| c.as_str()) {
                                    if cmd.trim().starts_with("rm") {
                                        return Err(format!("Governor Denied: Command '{}' is forbidden (Destructive)", cmd));
                                    }
                                }
                             }
                        }
                     }
                 }
             }
        }
        Ok(())
    }

    fn call_wasm(&self, json_rpc: &str) -> String {
        let msg = format!("Content-Length: {}\r\n\r\n{}", json_rpc.len(), json_rpc);
        
        // Scope for the lock
        {
            let mut inner = self.inner.lock().unwrap();
            if let Err(e) = inner.input_writer.write_all(msg.as_bytes()) {
                return serde_json::json!({"error": format!("Failed to write to WASM stdin: {}", e)}).to_string();
            }
            if let Err(e) = inner.input_writer.flush() {
                return serde_json::json!({"error": format!("Failed to flush WASM stdin: {}", e)}).to_string();
            }
            
            // Read Response
            match read_framed_message(&mut inner.output_reader) {
                Ok(response) => response,
                Err(e) => serde_json::json!({"error": format!("Failed to read from WASM stdout: {}", e)}).to_string(),
            }
        }
    }

    fn call_mcp(&self, json_rpc: String) -> Result<String, Box<dyn std::error::Error>> {
        let response = self.handle_request(&json_rpc);
        
        // Hygiene Check
        self.check_recycle()?;
        
        // Increment Count
        let mut count = self.call_count.lock().unwrap();
        *count += 1;
        
        Ok(response)
    }
}

// LSP Reader Helper with MAX_BODY_SIZE Check
fn read_framed_message<R: BufRead>(reader: &mut R) -> Result<String, Box<dyn std::error::Error>> {
    let mut content_length: Option<usize> = None;
    
    // Read Headers
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 {
             return Err("Peer disconnected".into());
        }
        let line = line.trim_end();
        if line.is_empty() { break; } // End of headers
        if line.starts_with("Content-Length: ") {
            if let Ok(len) = line[16..].parse::<usize>() {
                if len > MAX_BODY_SIZE {
                    return Err(format!("Content-Length {} exceeds limit {}", len, MAX_BODY_SIZE).into());
                }
                content_length = Some(len);
            }
        }
    }

    if let Some(len) = content_length {
        let mut buf = vec![0; len];
        reader.read_exact(&mut buf)?;
        let s = String::from_utf8(buf)?;
        Ok(s)
    } else {
        Err("Missing Content-Length".into())
    }
}


// =============================================================================
// MAIN ENTRY
// =============================================================================

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize Telemetry Broadcast Channel
    let (tx_telemetry, _rx_telemetry) = broadcast::channel::<Signal>(100);

    // 2. Initialize Command Channel (Server -> Engine)
    let (cmd_tx, _cmd_rx) = mpsc::channel::<AeonCommand>(100);

    // 3. Spawn WebSocket Server
    let tx_for_server = tx_telemetry.clone();
    tokio::spawn(async move {
        server::start_server(tx_for_server, cmd_tx).await;
    });

    // 4. Start Heartbeat Loop
    let tx_heartbeat = tx_telemetry.clone();
    tokio::spawn(async move {
        loop {
            let _ = tx_heartbeat.send(Signal::new(
                "HEARTBEAT", 
                serde_json::json!({ "status": "NOMINAL", "load": 0.05 }), 
                "WARDEN"
            ));
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        }
    });

    println!("✅ Warden: Governor Gateway Active (Phase 12 Lite)");

    let args: Vec<String> = env::args().collect();

    if args.len() > 2 && args[1] == "run" {
        let input_path = &args[2];
        let script_path = input_path.to_string();

        let home = std::env::var("HOME").unwrap_or_default();
        let wasm_env = std::env::var("AEON_WASM_PATH").unwrap_or_else(|_| "../aeon-toolroom/target/wasm32-wasip1/release/aeon-toolroom.wasm".to_string());
        let wasm_path = Path::new(&wasm_env);
        let sandbox_path = PathBuf::from(&home).join(".aeon").join("sandbox");
        
        // Spawn Resident Subagent ONCE
        let subagent = Arc::new(ResidentSubagent::spawn(wasm_path, &sandbox_path, tx_telemetry.clone())?);
        
        println!("✅ Warden: Governor Gateway Active (Phase 12 Lite)");

        let mut cmd = Command::new("python3");
        cmd.arg(&script_path);
        cmd.stdin(Stdio::inherit()).stdout(Stdio::inherit()).stderr(Stdio::piped());
        let mut child = cmd.spawn().expect("Launch Failed");
        
        if let Some(stderr) = child.stderr.take() {
            let subagent_clone = subagent.clone();
            tokio::task::spawn_blocking(move || {
                let reader = BufReader::new(stderr);
                for line in reader.lines() {
                    if let Ok(l) = line {
                         if l.starts_with("[AEON_MCP]") {
                            let json_rpc = l[10..].to_string();
                            // Call Persistent Subagent (Gateway handles Policy)
                            match subagent_clone.call_mcp(json_rpc) {
                                Ok(response) => println!("🌐 WASM RESIDENT RESPONSE: {}", response),
                                Err(e) => eprintln!("❌ WASM ERROR: {}", e),
                            }
                        } else {
                            eprintln!("{}", l);
                        }
                    }
                }
            });
        }

        let _ = child.wait().expect("Wait Failed");
        return Ok(());
    }
    println!("usage: aeon <run> <path>");
    Ok(())
}
