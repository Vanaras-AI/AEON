use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State, Query},
    response::IntoResponse,
    routing::get,
    Router,
    http::StatusCode,
};
use tokio::sync::broadcast;
use tokio::sync::mpsc;
use futures::{sink::SinkExt, stream::StreamExt};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use crate::synapse::{Signal, Command};
use serde::Deserialize;

// Shared state for the server
struct AppState {
    tx: broadcast::Sender<Signal>,
    cmd_tx: mpsc::Sender<Command>,
    telemetry_secret: Option<String>,
}

#[derive(Deserialize)]
struct TelemetryQuery {
    token: Option<String>,
}

pub async fn start_server(tx: broadcast::Sender<Signal>, cmd_tx: mpsc::Sender<Command>) {
    // Read optional secret from environment
    let telemetry_secret = std::env::var("TELEMETRY_SECRET").ok();
    
    if telemetry_secret.is_some() {
        println!("🔐 [SERVER] Telemetry authentication ENABLED");
    } else {
        println!("⚠️  [SERVER] Telemetry authentication DISABLED (set TELEMETRY_SECRET to enable)");
    }

    let app_state = Arc::new(AppState { tx, cmd_tx, telemetry_secret });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/telemetry", get(telemetry_handler))
        .layer(cors)
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await
        .expect("Failed to bind to port 3000");
    
    println!("🚀 [SERVER] Telemetry listening on ws://0.0.0.0:3000/telemetry");

    if let Err(e) = axum::serve(listener, app).await {
        eprintln!("❌ [SERVER] Server crash: {}", e);
    }
}

async fn health_check() -> &'static str {
    "OK"
}

async fn telemetry_handler(
    ws: WebSocketUpgrade,
    Query(query): Query<TelemetryQuery>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, StatusCode> {
    // Check authentication if secret is configured
    if let Some(ref expected_secret) = state.telemetry_secret {
        match query.token {
            Some(ref provided) if provided == expected_secret => {
                // Auth passed
            },
            _ => {
                println!("🛑 [SERVER] Unauthorized telemetry connection attempt");
                return Err(StatusCode::UNAUTHORIZED);
            }
        }
    }
    
    Ok(ws.on_upgrade(|socket| handle_socket(socket, state)))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.tx.subscribe();

    println!("🔌 [SERVER] Dashboard Connected");

    // Task to forward broadcast -> ws
    let send_task = tokio::spawn(async move {
        while let Ok(signal) = rx.recv().await {
            if let Ok(json) = serde_json::to_string(&signal) {
                if sender.send(Message::Text(json.into())).await.is_err() {
                    break;
                }
            }
        }
    });

    // Receive commands from ws
    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(text) = msg {
            if let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) {
                 if let Some(type_str) = value.get("type").and_then(|s| s.as_str()) {
                     match type_str {
                         "HALT" => {
                             let _ = state.cmd_tx.send(Command::Halt).await;
                         },
                         "SIGNAL" => {
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
    println!("❌ [SERVER] Dashboard Disconnected");
}
