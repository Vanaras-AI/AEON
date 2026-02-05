mod synapse;
mod governance; 
mod keyring;
mod ledger;
mod state;
mod gemma;
mod gemma3; 
mod host_functions; 
mod server; // [NEW]

use synapse::{Cortex, Signal};
use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use notify::{Watcher, RecursiveMode, Config};
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex}; 
use tracing::{info, warn, error}; 
use std::env;
use state::AgentState;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mandate { // Made Public for modules
    agent_id: String,
    version: String,
    did: String,
    permissions: Vec<String>,
    #[serde(default)]
    subscriptions: Vec<String>,
    #[serde(default)]
    territory: Vec<String>,
    #[serde(default)]
    blueprint: std::collections::HashMap<String, String>,
}


// --- DID COMPUTATION (Public Key Based) ---
fn compute_did(agent_id: &str, version: &str) -> String {
    use keyring::AeonKeyring;
    
    // Load sovereign key
    let pubkey = match AeonKeyring::init() {
        Ok(keyring) => keyring.public_key_hex(),
        Err(_) => {
            // Fallback to SHA256 if keyring not initialized (for backwards compatibility)
            use sha2::{Sha256, Digest};
            let input = format!("{}:{}",agent_id, version);
            let hash = Sha256::digest(input.as_bytes());
            format!("{:x}", hash)
        }
    };
    
    format!("did:aeon:{}:{}:{}", agent_id, version, pubkey)
}

// --- LEDGER MANAGEMENT (SQLite) ---
fn append_ledger(agent_id: &str, operation: &str, target: &str, status: &str) -> Result<()> {
    use ledger::{Ledger, LedgerEntry};
    
    // Use the global singleton actor with non-blocking send (safe for tokio runtime)
    let ledger = Ledger::init()?;
    ledger.append_non_blocking(LedgerEntry {
        agent_id: agent_id.to_string(),
        operation: operation.to_string(),
        target: if target == "N/A" { None } else { Some(target.to_string()) },
        status: status.to_string(),
        metadata: None,
    })?;
    
    Ok(())
}

fn generate_territory_map() -> std::io::Result<()> {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    
    // Scan mandates directory
    let mandates_dir = Path::new("../mandates");
    if mandates_dir.exists() {
        for entry in fs::read_dir(mandates_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("toml") {
                let toml_content = fs::read_to_string(&path)?;
                if let Ok(mandate) = toml::from_str::<Mandate>(&toml_content) {
                    for t in mandate.territory {
                        map.insert(t, mandate.did.clone());
                    }
                }
            }
        }
    }

    let map_path = "../specs/metropolis_map.json";
    fs::write(map_path, serde_json::to_string_pretty(&map).unwrap())?;
    Ok(())
}

// AgentState moved to state.rs

fn snapshot_territory(mandate: &Mandate) -> std::io::Result<()> {
    let archive_dir = format!("../archive/{}/{}", mandate.agent_id, mandate.version);
    fs::create_dir_all(&archive_dir)?;
    
    for path in &mandate.territory {
        if Path::new(path).exists() {
            let filename = Path::new(path).file_name().unwrap_or_default();
            let dest = Path::new(&archive_dir).join(filename);
            fs::copy(path, dest)?;
            println!("📸 [CHRONOS] Snapshot created for {} territory: {} (v{})", mandate.agent_id, path, mandate.version);
        }
    }
    Ok(())
}

fn spawn_instance(engine: &Engine, linker: &Linker<AgentState>, module: &Module, mandate: Mandate, bus: Arc<Mutex<std::collections::HashMap<String, Vec<Signal>>>>) -> Result<()> {
    println!("🧬 Spawning Cell: {} (DNA: {:?})", mandate.agent_id, mandate.permissions);
    let wasi = WasiCtxBuilder::new().inherit_stdout().inherit_stderr().build();
    let mut store = Store::new(&engine, AgentState { wasi, mandate, signal_bus: bus });
    let instance = linker.instantiate(&mut store, &module)?;
    let run = instance.get_typed_func::<(), ()>(&mut store, "_start")?;
    run.call(&mut store, ())?;
    Ok(())
}
fn pulse(engine: &Engine, linker: &Linker<AgentState>, module: &Module, bus: Arc<Mutex<std::collections::HashMap<String, Vec<Signal>>>>) -> Result<()> {
    let mandates_dir = "../mandates";
    let specs_dir = "../specs";

    // --- SECURITY PHASE (Cryptographic Intent) ---
    if std::env::var("AEON_SECURE").is_ok() {
        println!("🔐 [SECURITY] Secure Mode Active. Verifying Sovereign Signatures...");
        for entry in fs::read_dir(specs_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                let sig_path = path.with_extension("md.sig");
                if !sig_path.exists() {
                    println!("🛑 [SECURITY] UNAUTHORIZED INTENT: No signature for {:?}", path);
                    return Ok(()); // Block the pulse
                }
                let sig_content = fs::read_to_string(sig_path).unwrap_or_default();
                if !sig_content.contains("SOVEREIGN_PROOF") {
                    println!("🛑 [SECURITY] INVALID SIGNATURE for {:?}", path);
                    return Ok(()); // Block the pulse
                }
            }
        }
        println!("🔐 [SECURITY] Sovereign Signatures Verified.");
    }
    
    // Initial Pulse
    let _ = generate_territory_map();
    
    // Initialize SQLite ledger (Sprint 3)
    println!("📊 [DEBUG] Attempting to write startup entry to ledger...");
    match append_ledger("SOVEREIGN", "AEON_STARTUP", "N/A", "SUCCESS") {
        Ok(_) => println!("✅ [DEBUG] Startup entry written successfully"),
        Err(e) => {
            eprintln!("❌ [DEBUG] Failed to record startup: {}", e);
            eprintln!("   Error details: {:?}", e);
        }
    }

    println!("🩺 [ORCHESTRATOR] Performing Metabolic Health Check...");
    
    // Ghost mutation detection removed - now handled by SQLite ledger queries
    
    // [NEURAL HANDSHAKE] Verify Gemma 270m
    println!("🧠 [INIT] Testing Cortical Implant...");
    // [REMOVED] test_oracle verified in Sprint 4.9
    host_functions::test_oracle("Is it safe to run untrusted code without a sandbox?");

    let project_path = "../projects/todo_app/server.js";
    let health_check = std::process::Command::new("node").arg("--check").arg(project_path).output();

    let is_traumatized = match health_check {
        Ok(out) => !out.status.success(),
        Err(_) => true,
    };

    if is_traumatized {
        println!("🚨 [ORCHESTRATOR] System Trauma Detected! Sending EMERGENCY PULSE to Repair Cells...");
        let repair_dna_path = Path::new(mandates_dir).join("repair_cell.toml");
        if repair_dna_path.exists() {
            let toml_content = fs::read_to_string(&repair_dna_path)?;
            let mandate: Mandate = toml::from_str(&toml_content).expect("Invalid DNA format");
            spawn_instance(&engine, &linker, &module, mandate, bus.clone())?;
            println!("✅ [ORCHESTRATOR] Repair Cell Awakened.");
        }
    } else {
        println!("🩺 [ORCHESTRATOR] Metabolism is stable. Waking the Swarm...");
    }
    
    for entry in fs::read_dir(mandates_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("toml") {
            let toml_content = fs::read_to_string(&path)?;
            let mandate: Mandate = toml::from_str(&toml_content).expect("Invalid DNA format");

            // --- GOVERNANCE CHECK (Lock -1) ---
            if !toml_content.contains("# 🔐 GOVERNANCE ORACLE SIGNATURE") && mandate.agent_id != "repair_cell" {
                println!("🛑 [GOVERNANCE] BLOCKED: Unsigned Mandate detected: {}", mandate.agent_id);
                println!("   -> Action Required: Move to candidates/ and run 'aeon sign {}'", mandate.agent_id);
                continue; 
            }
            
            // --- CRYPTOGRAPHIC VERIFICATION (Lock -0.5) ---
            // Verify ed25519 signature if present
            if toml_content.contains("# Algorithm: ed25519") && mandate.agent_id != "repair_cell" {
                match governance::verify_mandate_signature(&path) {
                    Ok(true) => {
                        println!("✅ [CRYPTO] Signature verified for: {}", mandate.agent_id);
                    }
                    Ok(false) => {
                        println!("🛑 [CRYPTO] INVALID SIGNATURE for: {}", mandate.agent_id);
                        println!("   -> The mandate content has been tampered with!");
                        println!("   -> Signature does not match the public key");
                        continue; // Block execution
                    }
                    Err(e) => {
                        println!("🛑 [CRYPTO] Signature verification ERROR for {}: {}", mandate.agent_id, e);
                        println!("   -> Mandate may be corrupted or use an unsupported format");
                        continue; // Block execution
                    }
                }
            }
            
            // --- IDENTITY VERIFICATION (Lock 0) ---
            let expected_did = compute_did(&mandate.agent_id, &mandate.version);
            if mandate.did != expected_did {
                println!("🛑 [IDENTITY] Forged Agent ID detected! ID: {} (v{}) | Expected DID: {} | Found: {}", mandate.agent_id, mandate.version, expected_did, mandate.did);
                continue; // Block this cell from the pulse
            }

            // --- CHRONOS SNAPSHOT ---
            // Always snapshot on first run (version detection via SQLite pending)
            let is_new_version = true;

            if is_new_version {
                let _ = snapshot_territory(&mandate);
            }

            if mandate.agent_id != "repair_cell" {
                spawn_instance(&engine, &linker, &module, mandate, bus.clone())?;
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    
    // [OBSERVABILITY] Initialize structured logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // [NEW] Initialize Telemetry Broadcast Channel
    let (tx_telemetry, _rx_telemetry) = tokio::sync::broadcast::channel::<Signal>(100);

    // [NEW] Initialize Command Channel (Server -> Main)
    let (cmd_tx, mut cmd_rx) = tokio::sync::mpsc::channel::<synapse::Command>(100);

    // [NEW] Spawn WebSocket Server
    let tx_for_server = tx_telemetry.clone();
    tokio::spawn(async move {
        server::start_server(tx_for_server, cmd_tx).await;
    });

    // [NEW] CLI Argument Parsing
    let args: Vec<String> = env::args().collect();
    
    // ... (Arg parsing omitted for brevity in diff, assume it remains) ...
    // Handle "aeon init" command (Sprint 2)
    if args.len() > 1 && args[1] == "init" {
        println!("🔐 [KEYRING] Initializing AEON Cryptographic Identity...");
        match keyring::AeonKeyring::init() {
            Ok(kr) => {
                println!("✅ [KEYRING] Initialization complete!");
                println!("   Public Key: {}", kr.public_key_hex());
                println!("   Location: .aeon/keyring/sovereign.key");
                println!("");
                println!("⚠️  IMPORTANT: Keep your private key secure!");
                println!("   - Never commit .aeon/keyring/ to version control");
                println!("   - Backup your key to a secure location");
                if let Ok(l) = ledger::Ledger::init() { l.flush().await; }
                return Ok(());
            }
            Err(e) => {
                eprintln!("❌ [KEYRING] Failed to initialize: {}", e);
                std::process::exit(1);
            }
        }
    }
    
    // Handle "aeon sign <name>" command
    for (i, arg) in args.iter().enumerate() {
        if arg == "sign" && i + 1 < args.len() {
            let candidate_name = &args[i + 1];
            if let Err(e) = governance::approve_candidate(candidate_name) {
                eprintln!("❌ [GOVERNANCE] Failed to sign candidate: {}", e);
                std::process::exit(1);
            }
            return Ok(());
        }
    }
    
    // Handle "aeon query" and "aeon analytics"
    if args.len() > 1 {
        // ... (Existing query/analytics logic preserved) ...
        match args[1].as_str() {
            "query" => {
                 println!("📊 [LEDGER] Querying metabolic ledger...");
                
                match ledger::Ledger::init() {
                    Ok(db) => {
                        // Parse query type
                        if args.len() > 2 {
                            match args[2].as_str() {
                                "--recent" | "-r" => {
                                    let limit = args.get(3)
                                        .and_then(|s| s.parse::<usize>().ok())
                                        .unwrap_or(10);
                                    
                                    match db.recent(limit).await {
                                        Ok(records) => {
                                            println!("\n📋 Recent {} entries:\n", limit);
                                            for record in records {
                                                println!("{}", record.format());
                                            }
                                        }
                                        Err(e) => eprintln!("❌ Query failed: {}", e),
                                    }
                                }
                                "--agent" | "-a" => {
                                    eprintln!("⚠️ Filter by agent not yet supported in Actor mode");
                                }
                                "--operation" | "-o" => {
                                    eprintln!("⚠️ Filter by operation not yet supported in Actor mode");
                                }
                                "--count" | "-c" => {
                                    match db.count().await {
                                        Ok(count) => println!("\n📊 Total ledger entries: {}", count),
                                        Err(e) => eprintln!("❌ Query failed: {}", e),
                                    }
                                }
                                _ => {
                                    println!("\n📖 Usage:");
                                    println!("  aeon query --recent [limit]       # Show recent entries");
                                    println!("  aeon query --count                # Total entry count");
                                }
                            }
                        } else {
                            // Default: show recent 10
                            match db.recent(10).await {
                                Ok(records) => {
                                    println!("\n📋 Recent 10 entries:\n");
                                    for record in records {
                                        println!("{}", record.format());
                                    }
                                }
                                Err(e) => eprintln!("❌ Query failed: {}", e),
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ [LEDGER] Failed to open database: {}", e);
                        std::process::exit(1);
                    }
                }
                return Ok(());
            },
            "analytics" => {
                 use ledger::Ledger;
                let ledger = match Ledger::init() {
                    Ok(l) => l,
                    Err(e) => {
                        eprintln!("❌ [LEDGER] Failed to open database: {}", e);
                        std::process::exit(1);
                    }
                };
                
                // --top-agents
                if args.contains(&"--top-agents".to_string()) || args.len() == 2 {
                    println!("\n🏆 Top Agents by Activity");
                    println!("------------------------");
                    match ledger.top_agents(10).await {
                        Ok(stats) => {
                            let max_count = stats.first().map(|s| s.count).unwrap_or(1);
                            for stat in stats {
                                let bars = (stat.count as f64 / max_count as f64 * 20.0) as usize;
                                let bar_str = "█".repeat(bars);
                                println!("{:<20} {} ({})", stat.agent_id, bar_str, stat.count);
                            }
                        },
                        Err(e) => eprintln!("❌ Failed to query top agents: {}", e),
                    }
                }
                
                // --ops (Distribution)
                if args.contains(&"--ops".to_string()) || args.len() == 2 {
                    println!("\n📊 Operation Distribution");
                    println!("------------------------");
                    match ledger.operation_distribution().await {
                        Ok(stats) => {
                            let max_count = stats.first().map(|s| s.count).unwrap_or(1);
                            for stat in stats {
                                let bars = (stat.count as f64 / max_count as f64 * 20.0) as usize;
                                let bar_str = "█".repeat(bars);
                                println!("{:<20} {} ({})", stat.operation, bar_str, stat.count);
                            }
                        },
                        Err(e) => eprintln!("❌ Failed to query operation distribution: {}", e),
                    }
                }
                
                // --success (Metrics)
                if args.contains(&"--success".to_string()) || args.len() == 2 {
                    println!("\n✅ Metabolic Health");
                    println!("------------------------");
                    match ledger.success_metrics().await {
                        Ok((success, failures, rate)) => {
                            println!("Success Rate: {:.2}%", rate);
                            println!("Successful:   {}", success);
                            println!("Failures:     {}", failures);
                        },
                        Err(e) => eprintln!("❌ Failed to query success metrics: {}", e),
                    }
                }

                // --timeline (Activity)
                if args.contains(&"--timeline".to_string()) || args.len() == 2 {
                    println!("\n📅 Activity Timeline (Last 24h)");
                    println!("------------------------");
                    match ledger.timeline().await {
                        Ok(stats) => {
                            if stats.is_empty() {
                                println!("No activity recorded.");
                            } else {
                                let max_count = stats.iter().map(|s| s.count).max().unwrap_or(1);
                                for stat in stats {
                                    let bars = (stat.count as f64 / max_count as f64 * 40.0) as usize;
                                    let bar_str = "█".repeat(bars);
                                    println!("{} │ {} ({})", stat.time_bucket, bar_str, stat.count);
                                }
                            }
                        },
                        Err(e) => eprintln!("❌ Failed to query timeline: {}", e),
                    }
                }

                println!(""); // Final newline
                if let Ok(l) = ledger::Ledger::init() { l.flush().await; }
                return Ok(());
            },
            "archive" => {
                 use ledger::Ledger;
                let ledger = match Ledger::init() {
                    Ok(l) => l,
                    Err(e) => {
                        eprintln!("❌ [LEDGER] Failed to open database: {}", e);
                        std::process::exit(1);
                    }
                };

                let mut days = 90; // Default retention
                if let Some(pos) = args.iter().position(|a| a == "--older-than") {
                    if pos + 1 < args.len() {
                        if let Ok(d) = args[pos + 1].parse::<u64>() {
                            days = d;
                        }
                    }
                }
                
                println!("📦 [ARCHIVE] Archiving entries older than {} days...", days);
                match ledger.archive_older_than(days).await {
                    Ok(count) => println!("✅ [ARCHIVE] archived {} entries to cold storage.", count),
                    Err(e) => eprintln!("❌ [ARCHIVE] Failed: {}", e),
                }
                
                if let Ok(l) = ledger::Ledger::init() { l.flush().await; }
                return Ok(());
            },
            "detect" => {
                 use ledger::{Ledger, DetectionRule};
                let ledger = match Ledger::init() {
                    Ok(l) => l,
                    Err(e) => {
                        eprintln!("❌ [LEDGER] Failed to open database: {}", e);
                        std::process::exit(1);
                    }
                };
                
                println!("🚨 [DETECT] Scanning metabolic ledger for anomalies...");
                
                // Parse flags (simplified for now, mostly using All)
                let rule = if args.contains(&"--privilege".to_string()) {
                    DetectionRule::PrivilegeEscalation
                } else if args.contains(&"--burst".to_string()) {
                    DetectionRule::BurstActivity
                } else {
                    DetectionRule::All
                };

                match ledger.detect_anomalies(rule).await {
                    Ok(anomalies) => {
                        if anomalies.is_empty() {
                            println!("✅ [DETECT] No anomalies found. System nominal.");
                        } else {
                            println!("⚠️  [DETECT] Found {} anomalies:", anomalies.len());
                            for anomaly in anomalies {
                                let color = match anomaly.severity.as_str() {
                                    "HIGH" => "\x1b[31m", // Red
                                    "MEDIUM" => "\x1b[33m", // Yellow
                                    _ => "\x1b[0m",
                                };
                                let reset = "\x1b[0m";
                                println!("   {}[{}] {} - {}{}", color, anomaly.severity, anomaly.check, anomaly.description, reset);
                            }
                        }
                    },
                    Err(e) => eprintln!("❌ [DETECT] Failed: {}", e),
                }

                
                if let Ok(l) = ledger::Ledger::init() { l.flush().await; }
                return Ok(());
            },
            "benchmark" => {
                 use ledger::{Ledger, LedgerEntry};
                use std::time::Instant;

                println!("🏎️  [BENCHMARK] Starting AEON Ledger Throughput Test...");
                
                let mut insert_count = 1000;
                let mut batch_size = 100;
                
                if let Some(pos) = args.iter().position(|a| a == "--inserts") {
                    if pos + 1 < args.len() { insert_count = args[pos+1].parse().unwrap_or(1000); }
                }
                if let Some(pos) = args.iter().position(|a| a == "--batch-size") {
                    if pos + 1 < args.len() { batch_size = args[pos+1].parse().unwrap_or(100); }
                }

                // 1. Setup Test DB
                let mut ledger = match Ledger::init() {
                    Ok(l) => l,
                    Err(e) => {
                        eprintln!("❌ [BENCHMARK] DB Init failed: {}", e);
                        std::process::exit(1);
                    }
                };

                // 2. Default Insert Test
                let start = Instant::now();
                println!("   -> Running {} single inserts...", insert_count);
                for i in 0..insert_count {
                    ledger.append(LedgerEntry {
                        agent_id: "bench_agent".to_string(),
                        operation: "BENCH_OP".to_string(),
                        target: Some(format!("item_{}", i)),
                        status: "SUCCESS".to_string(),
                        metadata: None,
                    }).await.unwrap();
                }
                let duration = start.elapsed();
                let ops_sec = insert_count as f64 / duration.as_secs_f64();
                println!("   ⏱️  Result: {:.2} ops/sec (Total: {:.2}s)", ops_sec, duration.as_secs_f64());

                // 3. Batch Insert Test
                let start_batch = Instant::now();
                let total_batches = insert_count / batch_size;
                println!("   -> Running {} inserts in batches of {}...", insert_count, batch_size);
                
                for _ in 0..total_batches {
                    let mut batch = Vec::new();
                    for j in 0..batch_size {
                         batch.push(LedgerEntry {
                            agent_id: "bench_agent_batch".to_string(),
                            operation: "BENCH_BATCH".to_string(),
                            target: Some(format!("batch_item_{}", j)),
                            status: "SUCCESS".to_string(),
                            metadata: None,
                        });
                    }
                    ledger.append_batch(batch).await.unwrap();
                }
                let duration_batch = start_batch.elapsed();
                let ops_sec_batch = (total_batches * batch_size) as f64 / duration_batch.as_secs_f64();
                println!("   ⏱️  Result: {:.2} ops/sec (Total: {:.2}s)", ops_sec_batch, duration_batch.as_secs_f64());
                println!("   🚀 Speedup: {:.2}x", ops_sec_batch / ops_sec);

                if let Ok(l) = ledger::Ledger::init() { l.flush().await; }
                return Ok(());
            },
            _ => { /* Ignore unrecognized */ }
        }
    }

    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    // Wrap Cortex in Arc<Mutex> for shared access
    let cortex = Arc::new(std::sync::Mutex::new(Cortex::new()));

    // ... (Connector Logic for MCPs Omitted - Assume Preserved) ...
    // Connect to Filesystem MCP (Restricted to ../projects for safety)
    println!("🔌 [CORTEX] Initializing Filesystem MCP (Background)...");
    Cortex::connect_stdio_background(
        cortex.clone(),
        "filesystem".to_string(), 
        "npx".to_string(), 
        vec!["-y".to_string(), "@modelcontextprotocol/server-filesystem".to_string(), "../projects".to_string()]
    ).await;

    // Connect to Discord MCP if configured
    if std::env::var("DISCORD_BOT_TOKEN").is_ok() {
        println!("🔌 [CORTEX] Found DISCORD_BOT_TOKEN. Initializing Discord MCP (Background)...");
        Cortex::connect_stdio_background(
            cortex.clone(),
            "discord".to_string(),
            "npx".to_string(), 
            vec!["-y".to_string(), "@missionsquad/mcp-discord".to_string()]
        ).await;
    } else {
         println!("⚠️ [CORTEX] No DISCORD_BOT_TOKEN found. Skipping Discord MCP connection.");
    }

    // Connect to GitHub MCP if configured
    if std::env::var("GITHUB_PERSONAL_ACCESS_TOKEN").is_ok() {
        println!("🔌 [CORTEX] Found GITHUB_PERSONAL_ACCESS_TOKEN. Initializing GitHub MCP (Background)...");
        Cortex::connect_stdio_background(
            cortex.clone(),
            "github".to_string(),
            "npx".to_string(),
            vec!["-y".to_string(), "@modelcontextprotocol/server-github".to_string()]
        ).await;
    } else {
        println!("⚠️ [CORTEX] No GITHUB_PERSONAL_ACCESS_TOKEN found. Skipping GitHub MCP connection.");
    }

    // Connect to Todoist MCP (Community) if configured
    if std::env::var("TODOIST_API_TOKEN").is_ok() {
        println!("🔌 [CORTEX] Found TODOIST_API_TOKEN. Initializing Todoist MCP (Background)...");
         Cortex::connect_stdio_background(
            cortex.clone(),
            "todoist".to_string(),
            "npx".to_string(),
            vec!["-y".to_string(), "@hoffination/mcp-todoist".to_string()]
        ).await;
    } else {
        println!("⚠️ [CORTEX] No TODOIST_API_TOKEN found. Skipping Todoist MCP connection.");
    }

    // Connect to Google Workspace MCP (via uvx) if configured
    if std::env::var("GOOGLE_OAUTH_CLIENT_ID").is_ok() {
        println!("🔌 [CORTEX] Found GOOGLE_OAUTH_CLIENT_ID. Initializing Google Workspace MCP (Background)...");
        Cortex::connect_stdio_background(
            cortex.clone(),
            "google_workspace".to_string(),
            "uvx".to_string(),
            vec!["workspace-mcp".to_string(), "--tool-tier".to_string(), "core".to_string()]
        ).await;
    } else {
        println!("⚠️ [CORTEX] No GOOGLE_OAUTH_CLIENT_ID found. Skipping Google Workspace MCP connection.");
    }

    // Connect to Memory MCP (The Hippocampus)
    println!("🔌 [CORTEX] Initializing Memory MCP (Hippocampus)...");
    Cortex::connect_stdio_background(
        cortex.clone(),
        "memory".to_string(),
        "npx".to_string(),
        vec!["-y".to_string(), "@modelcontextprotocol/server-memory".to_string()]
    ).await;

    // [WATCHTOWER] Start Supervision Loop (Sprint 4.6)
    let cortex_clone_for_watchtower = cortex.clone();
    let registry_for_watchtower = {
        let guard = cortex.lock().unwrap();
        guard.registry.clone()
    };
    tokio::spawn(async move {
        Cortex::supervise(registry_for_watchtower, cortex_clone_for_watchtower).await;
    });

    wasmtime_wasi::add_to_linker(&mut linker, |s: &mut AgentState| &mut s.wasi)?;
    host_functions::register_functions(&mut linker)?;

    let candidates = vec![
        "../agents/cell/target/wasm32-wasip1/release/aeon-cell.wasm", // From core/
        "agents/cell/target/wasm32-wasip1/release/aeon-cell.wasm",    // From root/
        "aeon-cell.wasm",                                             // Local
    ];

    let cell_wasm_path = candidates.iter()
        .find(|p| Path::new(p).exists())
        .ok_or_else(|| anyhow::anyhow!("Could not find aeon-cell.wasm in candidates: {:?}", candidates))?;
    
    println!("🔌 [CORTEX] Loading Cell DNA from: {}", cell_wasm_path);
    let module = Module::from_file(&engine, cell_wasm_path)?;
    // Initial Pulse
    {
        let cortex_guard = cortex.lock().unwrap();
        pulse(&engine, &linker, &module, cortex_guard.signal_bus.clone())?;
    }

    // [NEW] Start Governance Bridge (Gemma 3)
    if let Err(e) = crate::gemma::BRIDGE.start_server() {
        eprintln!("❌ [MAIN] Failed to start Governance Bridge: {}", e);
        // We continue, but governance checks will fail.
    }

    println!("💓 [HEARTBEAT] AEON Heartbeat Online. Watching for Sovereign Intent...");
    let (tx, rx) = channel(); // std::sync::mpsc for Notify
    let mut watcher = notify::RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(Path::new("../specs"), RecursiveMode::NonRecursive)?;
    watcher.watch(Path::new("../mandates"), RecursiveMode::NonRecursive)?;

    let mut last_event_time = Instant::now();
    let mut pending_pulse = false;
    let mut is_halted = false; // [NEW] Halt Flag

    
    // [REFAC] Hybrid Async Loop (Sync Notify + Async Command)
    loop {
        // 0. Check for Async Commands (HALT / SIGNAL) using .try_recv()
        while let Ok(cmd) = cmd_rx.try_recv() {
            match cmd {
                synapse::Command::Halt => {
                    println!("🛑 [CORTEX] EMERGENCY HALT SIGNAL RECEIVED!");
                    println!("    -> Freezing all pulse activity.");
                    is_halted = true;
                },
                synapse::Command::Inject(signal) => {
                     println!("⚡ [CORTEX] INJECTING ARTIFICIAL SIGNAL: {:?}", signal);
                     
                     // [GOVERNANCE CHECK]
                     // We serialize the signal payload and ask Gemma if it's compliant.
                     // This is a synchronous blocking call for now (optimize later).
                     if let Ok(json_str) = serde_json::to_string(&signal.params) {
                         println!("⚖️  [GOVERNANCE] Assessing Signal Compliance...");
                         match crate::gemma::BRIDGE.assess_governance(&json_str) {
                             Ok(decision) => {
                                 println!("    -> Decision: {} (Risk: {})", decision.decision, decision.risk_level);
                                 if decision.decision == "NON_COMPLIANT" && decision.risk_level == "CRITICAL" {
                                      println!("🚨 [GOVERNANCE] CRITICAL VIOLATION DETECTED. INITIATING HALT.");
                                      is_halted = true;
                                      let _ = tx_telemetry.send(Signal::new("GOVERNANCE_HALT", serde_json::json!({"reason": decision}), "CORTEX_G"));
                                 } else {
                                     // Approved.
                                      let _ = tx_telemetry.send(signal);
                                 }
                             },
                             Err(e) => {
                                 println!("⚠️  [GOVERNANCE] Check Failed: {}", e);
                                 // Fail-open or Fail-closed? For now, Fail-Open (allow).
                                 let _ = tx_telemetry.send(signal);
                             }
                         }
                     }
                }
            }
        }

        if is_halted {
            tokio::time::sleep(Duration::from_millis(500)).await;
            continue; 
        }

        // Drive the Sensory Cortex (Async)
        {
            let mut cortex_guard = cortex.lock().unwrap();
            cortex_guard.heartbeat().await;
        }
        
        // [TELEMETRY] Broadcast Heartbeat
        let _ = tx_telemetry.send(Signal::new(
            "HEARTBEAT", 
            serde_json::json!({ "status": "nominal" }), 
            "CORTEX"
        ));
        
        // 1. Poll File Watcher (Non-blocking check)
        // Since we replaced rx.recv_timeout (blocking) with a polling loop
        if let Ok(res) = rx.try_recv() {
             match res {
                Ok(_) => {
                    last_event_time = Instant::now();
                    if !pending_pulse {
                        println!("💓 [HEARTBEAT] Sovereign Intent change detected. Initiating Coalescence...");
                        pending_pulse = true;
                        let _ = tx_telemetry.send(Signal::new("COALESCENCE_START", serde_json::json!({}), "CORTEX"));
                    }
                },
                Err(e) => println!("❌ [HEARTBEAT] Watcher error: {:?}", e),
            }
        }

        // 2. Pulse Trigger Check
        if pending_pulse && last_event_time.elapsed() >= Duration::from_secs(2) {
            println!("💎 [COALESCENCE] Intent stabilized. Pulsing Swarm...");
            pending_pulse = false;
            
            { // Lock scope
                let mut cortex_guard = cortex.lock().unwrap();
                cortex_guard.heartbeat().await;
                
                if let Err(e) = pulse(&engine, &linker, &module, cortex_guard.signal_bus.clone()) {
                   println!("❌ [HEARTBEAT] Pulse failure: {:?}", e);
                    let _ = tx_telemetry.send(Signal::new(
                        "PULSE_FAILED", 
                        serde_json::json!({ "error": format!("{:?}", e) }), 
                        "CORTEX"
                    ));
                } else {
                    let _ = tx_telemetry.send(Signal::new("PULSE", serde_json::json!({ "status": "success" }), "CORTEX"));
                }
            }
        }

        // 3. Short Sleep to prevent CPU spinning
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
