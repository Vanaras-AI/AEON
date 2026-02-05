use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State},
    response::IntoResponse,
    routing::get,
    Router,
};
use tokio::sync::broadcast;
use tokio::sync::mpsc; // [NEW] For Command Channel
use futures::{sink::SinkExt, stream::StreamExt};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use crate::synapse::{Signal, Command};

// Shared state for the server
struct AppState {
    tx: broadcast::Sender<Signal>,
    cmd_tx: mpsc::Sender<Command>, // [NEW] Command Channel
}

pub async fn start_server(tx: broadcast::Sender<Signal>, cmd_tx: mpsc::Sender<Command>) {
    let app_state = Arc::new(AppState { tx, cmd_tx });

    let cors = CorsLayer::new()
        .allow_origin(Any) // [DEV] Allow all origins for local dev
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/telemetry", get(telemetry_handler))
        .layer(cors)
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await
        .map_err(|e| {
            eprintln!("❌ [SERVER] Failed to bind to {}: {}", addr, e);
            e
        }).unwrap();
    
    println!("🚀 [SERVER] Cortex Telemetry listening on ws://{}", addr);

    if let Err(e) = axum::serve(listener, app).await {
        eprintln!("❌ [SERVER] Server crash: {}", e);
    }
}

async fn health_check() -> &'static str {
    "OK"
}

async fn telemetry_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.tx.subscribe();

    println!("🔌 [SERVER] New Dashboard Client Connected");

    // Spawn a task to forward broadcast -> ws
    let mut send_task = tokio::spawn(async move {
        while let Ok(signal) = rx.recv().await {
            if let Ok(json) = serde_json::to_string(&signal) {
                if sender.send(Message::Text(json.into())).await.is_err() {
                    break;
                }
            }
        }
    });

    // Main loop: Receive messages from ws
    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(text) = msg {
            // Parse incoming JSON command
            if let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) {
                 if let Some(type_str) = value.get("type").and_then(|s| s.as_str()) {
                     match type_str {
                         "HALT" => {
                             println!("🛑 [SERVER] RECV: HALT COMMAND");
                             let _ = state.cmd_tx.send(Command::Halt).await;
                         },
                         "SIGNAL" => {
                             println!("⚡ [SERVER] RECV: EXTERNAL SIGNAL");
                             let signal = Signal::new(
                                 "USER_INJECTION", 
                                 value.get("payload").cloned().unwrap_or(serde_json::json!({})), 
                                 "DASHBOARD"
                             );
                             let _ = state.cmd_tx.send(Command::Inject(signal)).await;
                         },
                         _ => {}
                     }
                 }
            }
        }
    }

    send_task.abort();
    println!("❌ [SERVER] Dashboard Client Disconnected");
}
