use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid;
use chrono;
use rust_mcp_sdk; // Test import

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

// Output Interface: Motors react to Signals
// pub trait Motor: Send + Sync { ... } 
// (Removed for MCP migration)

#[derive(Clone, Debug)]
pub enum Command {
    Halt,
    Inject(Signal),
}

// MCP Tool Representation
#[derive(Debug, Clone)]
pub struct McpTool {
    pub name: String,
    pub description: String,
}

pub struct ClientConfig {
    pub alias: String,
    pub command: String,
    pub args: Vec<String>,
}

pub struct SupervisedClient {
    pub client: Arc<dyn McpClient>,
    pub handle: tokio::task::JoinHandle<()>,
    pub config: ClientConfig,
    pub restarts: u32,
    pub last_restart: chrono::DateTime<chrono::Utc>,
}

// The Peripheral Nervous System
pub struct Cortex {
    pub signal_bus: Arc<Mutex<HashMap<String, Vec<Signal>>>>,
    pub tools: Vec<McpTool>,
    pub clients: HashMap<String, Arc<dyn McpClient>>, // For quick access
    pub registry: Arc<Mutex<HashMap<String, SupervisedClient>>>, // For supervision
}

// ... imports remain ...
use rust_mcp_sdk::{
    mcp_client::{client_runtime, McpClientOptions},
    StdioTransport, TransportOptions,
    schema::{ClientCapabilities, Implementation, InitializeRequestParams, LATEST_PROTOCOL_VERSION},
};
use rust_mcp_sdk::error::SdkResult;
use rust_mcp_sdk::McpClient;
use rust_mcp_sdk::McpClientHandler;

#[derive(Clone)]
struct CortexHandler;

use rust_mcp_sdk::schema::{
    ServerJsonrpcRequest, ResultFromClient, NotificationFromServer
};
use rust_mcp_sdk::schema::RpcError;
use async_trait::async_trait;

#[async_trait]
impl McpClientHandler for CortexHandler {
    async fn handle_request(&self, _req: ServerJsonrpcRequest, _c: &dyn McpClient) -> Result<ResultFromClient, RpcError> {
        Err(RpcError { code: -32601, message: "Method not found".to_string(), data: None })
    }
    async fn handle_notification(&self, _n: NotificationFromServer, _c: &dyn McpClient) -> SdkResult<()> { Ok(()) }
    async fn handle_error(&self, _e: &RpcError, _c: &dyn McpClient) -> SdkResult<()> { Ok(()) }
    async fn handle_process_error(&self, _e: String, _c: &dyn McpClient) -> SdkResult<()> { Ok(()) }
}

impl Cortex {
    pub fn new() -> Self {
        Self {
            signal_bus: Arc::new(Mutex::new(HashMap::new())),
            tools: Vec::new(),
            clients: HashMap::new(),
            registry: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn add_client(&mut self, name: String, client: Arc<dyn McpClient>) {
        self.clients.insert(name, client);
    }
    
    // The Watchtower Loop
    pub async fn supervise(registry: Arc<Mutex<HashMap<String, SupervisedClient>>>, cortex_arc: Arc<Mutex<Cortex>>) {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            
            let mut dead_clients = Vec::new();
            
            // 1. Check for dead handles
            if let Ok(reg) = registry.lock() {
                for (name, supervised) in reg.iter() {
                    if supervised.handle.is_finished() {
                         dead_clients.push((name.clone(), supervised.config.alias.clone(), supervised.config.command.clone(), supervised.config.args.clone(), supervised.restarts));
                    }
                }
            }
            
            // 2. Restart Phase
            for (name, alias, cmd, args, restarts) in dead_clients {
                println!("🚑 [WATCHTOWER] Client '{}' died. Attempting restart #{}...", name, restarts + 1);
                
                // Remove dead entry
                if let Ok(mut reg) = registry.lock() {
                    reg.remove(&name);
                }
                
                // Remove from active clients map
                if let Ok(mut c) = cortex_arc.lock() {
                    c.clients.remove(&name);
                }

                // Respawn
                let _ = Self::connect_stdio_background(cortex_arc.clone(), alias, cmd, args).await;
            }
        }
    }

    pub async fn connect_stdio_background(
        cortex_arc: Arc<Mutex<Cortex>>,
        alias: String, 
        command: String, 
        args: Vec<String>
    ) -> tokio::task::JoinHandle<()> {
        let _alias_clone = alias.clone();
        let _cmd_clone = command.clone();
        let _args_clone = args.clone();
        
        tokio::spawn(async move {
            println!("🔌 [CORTEX] Connecting to MCP Server ({}): {} (Background)...", alias, command);
            
            let transport_result = StdioTransport::create_with_server_launch(
                &command,
                args.clone(),
                None,
                TransportOptions::default(),
            );

            match transport_result {
                Ok(transport) => {
                    let client_details = InitializeRequestParams {
                        capabilities: ClientCapabilities::default(),
                        client_info: Implementation {
                            name: "aeon-cortex".into(),
                            version: "0.1.0".into(),
                            description: Some("AEON Cortex MCP Client".into()),
                            title: Some("AEON Cortex".into()),
                            icons: vec![],
                            website_url: None,
                        },
                        protocol_version: LATEST_PROTOCOL_VERSION.into(),
                        meta: None,
                    };

                    let client = client_runtime::create_client(McpClientOptions {
                        client_details,
                        transport,
                        handler: Box::new(CortexHandler),
                        task_store: None,
                        server_task_store: None,
                    });

                    let client_arc = client.clone();
                    
                    // The client logic loop
                    let handle = tokio::spawn(async move {
                        let _ = client.start().await;
                    });
                    
                    println!("✅ [CORTEX] Connected to MCP Server ({}): {}", alias, command);
                    
                    if let Ok(mut locked_cortex) = cortex_arc.lock() {
                        locked_cortex.add_client(alias.clone(), client_arc.clone());
                        if let Ok(mut reg) = locked_cortex.registry.lock() {
                            reg.insert(alias.clone(), SupervisedClient {
                                client: client_arc,
                                handle,
                                config: ClientConfig { alias: alias.clone(), command: command.clone(), args: args.clone() },
                                restarts: 0,
                                last_restart: chrono::Utc::now(),
                            });
                        }
                    }
                },
                Err(e) => {
                    println!("❌ [CORTEX] Failed to connect to MCP Server ({}) {}: {}", alias, command, e);
                }
            }
        })
    }

    pub async fn heartbeat(&mut self) {
        // Heartbeat is now handled by supervise() task
    }

    pub async fn call_tool(&self, _name: &str, _args: serde_json::Value) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        // TODO: implementations
        println!("🔧 [CORTEX] Tool Call Requested: {}", _name);
        Ok(serde_json::Value::Null)
    }
}

use rust_mcp_sdk::schema::CallToolRequestParams;

impl Cortex {
    // --- HIPPOCAMPUS (Memory) API ---

    pub async fn remember_fact(&self, entity_name: &str, entity_type: &str, observation: &str) -> Result<(), String> {
        let client = self.clients.get("memory").cloned().ok_or("Memory client not available")?;
        
        let entity_name = entity_name.to_string();
        let entity_type = entity_type.to_string();
        let observation = observation.to_string();
        
        let args = serde_json::json!({
            "entities": [{
                "name": entity_name,
                "entityType": entity_type,
                "observations": [observation]
            }]
        });

        let params = CallToolRequestParams {
            name: "create_entities".to_string(),
            arguments: args.as_object().cloned(),
            meta: None,
            task: None,
        };

        // [RELIABILITY] Zombie Prevention: Add 30s timeout (Sprint 4.7)
        let tool_call = client.request_tool_call(params);
        match tokio::time::timeout(std::time::Duration::from_secs(30), tool_call).await {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(e)) => Err(format!("Memory Tool Error: {}", e)),
            Err(_) => {
                // Timeout occurred
                Err("Memory Tool Timeout (>30s)".to_string())
            }
        }
    }

    pub async fn establish_relation(&self, from: &str, relation: &str, to: &str) -> Result<(), String> {
         let client = self.clients.get("memory").cloned().ok_or("Memory client not available")?;

         let args = serde_json::json!({
            "relations": [{
                "from": from,
                "relationType": relation,
                "to": to
            }]
        });
        
        let params = CallToolRequestParams {
            name: "create_relations".to_string(),
            arguments: args.as_object().cloned(),
            meta: None,
            task: None,
        };

        // [RELIABILITY] Zombie Prevention: Add 30s timeout (Sprint 4.7)
        let tool_call = client.request_tool_call(params);
        match tokio::time::timeout(std::time::Duration::from_secs(30), tool_call).await {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(e)) => Err(format!("Memory Tool Error: {}", e)),
            Err(_) => {
                 Err("Memory Tool Timeout (>30s)".to_string())
            }
        }
    }
}
