# Contributing to AEON

We welcome contributions! Here's how to get started:

## Development Setup

1. Clone the repo
2. Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
3. Install Node.js 18+
4. Install Python 3.8+

## Building

```bash
# Engine
cd aeon-engine && cargo build --release

# Dashboard
cd dashboard && npm install && npm run dev

# Governance Model (optional)
cd governance_model
python3 -m venv venv && source venv/bin/activate
pip install -r requirements.txt
python3 risk_server.py
```

## Testing

```bash
# Risk scoring tests
cd governance_model && python3 attack_test_suite.py
```

## Pull Requests

1. Fork the repo
2. Create a feature branch
3. Make your changes
4. Run tests
5. Submit PR

## Code Style

- Rust: `cargo fmt`
- TypeScript: ESLint
- Python: Black + isort

## Issues

Report bugs and feature requests via GitHub Issues.
