use std::sync::{Arc, Mutex};
use crate::Mandate;
use crate::synapse::Signal;

pub struct AgentState {
    pub wasi: wasmtime_wasi::WasiCtx,
    pub mandate: Mandate,
    pub signal_bus: Arc<Mutex<std::collections::HashMap<String, Vec<Signal>>>>,
}
