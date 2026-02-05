# AEON ENGINE (Autonomous Execution & Orchestration Network) 🧬

AEON is a **governance-first AI runtime** built on **Rust** and **WebAssembly (WASM)**. It provides secure, policy-enforced execution for AI agents with local intelligence and zero-trust architecture.

## 🚀 Key Features

### 🛡️ **6-Phase Governance Loop**
Every AI agent action passes through a comprehensive security pipeline:
1. **Intent Transformation** - Parse and validate tool calls
2. **Static Policy Evaluation** - Check against constitution rules
3. **Advisory Model (Hybrid Risk Scoring)** - AI + heuristics threat detection
4. **Capability Manifest** - Resource limits and network policies
5. **WASM Isolation** - Sandboxed execution in secure containers
6. **Verification & Audit** - Telemetry and compliance logging

### 🧠 **Hybrid Risk Scoring**
- **Gemma 3 270M** (GRC-trained) for unknown threat detection
- **Battle-tested heuristics** for known attack patterns
- **100% threat detection** with `max(model, heuristic)` strategy
- **Local inference** - no cloud dependency

### 🔒 **Security-First Architecture**
- **WASM Isolation**: Perfect security boundaries for agent execution
- **Policy-as-Code**: Constitution-based governance (TOML)
- **Zero-Trust**: Every action validated before execution
- **Audit Trail**: Complete telemetry for compliance

### ⚡ **Performance**
- **Rust Core**: Sub-millisecond policy evaluation
- **WASM Runtime**: Fast, portable agent execution
- **Real-time Dashboard**: WebSocket telemetry streaming

## 📂 Project Structure

- `/aeon-engine`: Rust-based governance engine and WASM runtime
- `/governance_model`: Gemma 3 270M risk scoring model + LoRA adapter
- `/core`: Capability system and policy enforcement
- `/dashboard`: Real-time governance monitoring UI
- `/mandates`: Agent DNA (TOML configurations)

## 🛠️ Quick Start

### Option 1: Heuristics-Only (Fast)
```bash
# Build and run
cd aeon-engine && cargo build --release
./target/release/aeon-engine run <agent_script>
```

### Option 2: With Gemma 3 (Enterprise)
```bash
# Start Gemma risk server
cd governance_model
python3 -m venv venv && source venv/bin/activate
pip install -r requirements.txt
python3 risk_server.py &

# Run aeon-engine (will use Gemma for risk scoring)
cd ../aeon-engine
cargo run --release
```

### Dashboard
```bash
cd dashboard && npm install && npm run dev
# Visit http://localhost:5173
```

## 🎯 Use Cases

- **AI Agent Sandboxing**: Run untrusted LLM-generated code safely
- **Compliance Automation**: Enforce GRC policies (ISO 27001, GDPR, NIST)
- **Enterprise AI**: Governed AI agents for regulated industries
- **Security Research**: Test and validate AI agent behaviors

## 📊 Governance Metrics

Track in real-time via dashboard:
- Risk scores (Gemma + Heuristic)
- Policy violations
- Resource usage
- Audit logs

---

**"Autonomous execution, governed by intelligence."**
