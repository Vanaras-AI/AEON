# Security Policy

## Reporting Vulnerabilities

If you discover a security vulnerability in AEON, please report it responsibly:

**📧 Email**: founder@vanaras.ai

**Please DO NOT** open public GitHub issues for security vulnerabilities.

### What to Include

When reporting a vulnerability, please include:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if available)

We will acknowledge receipt within 48 hours and provide a timeline for a fix.

---

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

---

## Security Features

AEON is built with security-first principles:

### 🛡️ **6-Phase Governance Loop**
Every AI agent action passes through comprehensive security validation:
1. **Intent Transformation** - Parse and validate tool calls
2. **Static Policy Evaluation** - Constitution-based rules
3. **Advisory Model (Hybrid Risk Scoring)** - Gemma 3 270M + heuristics
4. **Capability Manifest** - Resource limits and network policies
5. **WASM Isolation** - Sandboxed execution
6. **Verification & Audit** - Complete telemetry logging

### 🔒 **Isolation & Sandboxing**
- **WASM Runtime**: Perfect security boundaries using Wasmtime
- **Docker Isolation**: Non-root user execution
- **Capability-based Security**: Explicit resource permissions

### 🧠 **Hybrid Risk Scoring**
- **Gemma 3 270M** (GRC-trained) for unknown threat detection
- **Battle-tested heuristics** for known attack patterns
- **100% threat detection** with `max(model, heuristic)` strategy
- **Local inference** - no cloud dependency

### 🔐 **Cryptographic Identity**
- **Ed25519 keypairs** for agent signing
- **DID-based identity** (`did:aeon:agent:version:pubkey`)
- **Private keys** stored with `0o600` permissions (Unix)

### 📊 **Audit Trail**
- Complete telemetry via WebSocket
- Immutable ledger for compliance
- Real-time governance monitoring

---

## Best Practices

### For Contributors

1. **Never commit credentials**:
   - API keys, tokens, passwords
   - Private keys (`.key`, `.pem`, `.crt`)
   - `.env` files (use `.env.example` as template)

2. **Use environment variables** for all secrets:
   ```bash
   # Good
   export TODOIST_API_TOKEN="your_token"
   
   # Bad - hardcoding in source
   const token = "abc123";
   ```

3. **Run security scans** before submitting PRs:
   ```bash
   # Rust
   cargo audit
   
   # Python
   pip install pip-audit
   pip-audit
   
   # JavaScript
   cd dashboard && npm audit
   ```

4. **Review constitution policies** when adding new capabilities

### For Deployment

1. **Rotate keys** generated during development:
   ```bash
   rm -rf .aeon/keyring
   # AEON will generate new keypair on next run
   ```

2. **Use separate credentials** for dev/staging/prod environments

3. **Enable audit logging** in production:
   - Monitor WebSocket telemetry on port 9001
   - Store ledger events for compliance

4. **Review and customize** constitution policies:
   - Network allow/block lists
   - Filesystem write permissions
   - Resource limits

5. **Keep dependencies updated**:
   ```bash
   cargo update
   pip install --upgrade -r requirements.txt
   npm update
   ```

---

## Known Security Considerations

### WASM Sandbox Limitations
- WASM provides strong isolation but is not a complete security boundary
- Always run AEON in a containerized environment for production
- Review capability manifests carefully

### Model Inference
- Gemma risk scoring runs locally (no data leaves your system)
- Model can be bypassed if risk server is unavailable (falls back to heuristics)
- Consider running Gemma server with authentication in production

### Network Policies
- Constitution network policies are enforced at application level
- For defense-in-depth, use firewall rules and network segmentation
- Review allowed domains regularly

---

## Security Roadmap

Future security enhancements planned:
- [ ] Encrypted telemetry streams
- [ ] Multi-signature governance approvals
- [ ] Hardware security module (HSM) integration
- [ ] Formal verification of WASM isolation
- [ ] Security audit by third-party firm

---

## Acknowledgments

We appreciate responsible disclosure and will acknowledge security researchers who help improve AEON's security.

---

**Last Updated**: 2026-02-05
