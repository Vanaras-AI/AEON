---
sidebar_position: 11
---

# Security

Security practices and vulnerability reporting for AEON.

## Reporting Vulnerabilities

**Email:** founder@vanaras.ai

We take security seriously. If you discover a vulnerability:

1. **Do not** open a public GitHub issue
2. Email us with details of the vulnerability
3. Include steps to reproduce if possible
4. We will acknowledge within 48 hours
5. We aim to patch critical issues within 7 days

## Security Architecture

### 6-Phase Governance Loop

Every agent action passes through:

1. **Intent Transformation** - Parse and validate
2. **Static Policy Evaluation** - Constitution check
3. **Hybrid Risk Scoring** - AI + heuristics
4. **Capability Manifest** - Resource limits
5. **WASM Isolation** - Sandboxed execution
6. **Audit & Telemetry** - Complete logging

### WASM Sandboxing

- Memory isolation between agents
- CPU time limits enforced
- Filesystem access restricted
- Network access controlled
- No direct system calls

### Cryptographic Identity

- **Ed25519** keypairs for agent identity
- Private keys stored with `0600` permissions
- DIDs for decentralized identification

### Zero-Trust Architecture

- Every action validated before execution
- No implicit trust between components
- Continuous verification of agent state

## Best Practices

### Environment Variables

```bash
# Never commit .env files
echo ".env" >> .gitignore
echo ".env.*" >> .gitignore

# Use the template
cp .env.example .env
```

### Secret Management

- Use environment variables for secrets
- Never hardcode credentials
- Rotate API keys regularly
- Use secret management tools in production

### Network Security

- Run behind TLS in production
- Restrict telemetry WebSocket access
- Use allow-lists for network policies

### Filesystem Security

- Restrict write paths in constitution
- Use isolated directories for agents
- Regular audit of file operations

## Security Checklist

### Deployment

- [ ] TLS enabled for all endpoints
- [ ] Telemetry requires authentication
- [ ] .env file not committed
- [ ] Non-root user in containers
- [ ] Resource limits configured

### Configuration

- [ ] Network allow-list configured
- [ ] Filesystem write paths restricted
- [ ] Risk threshold appropriate
- [ ] Gemma model validated

### Monitoring

- [ ] Telemetry logging enabled
- [ ] Audit logs retained
- [ ] Alerts for blocked intents
- [ ] Dashboard accessible

## Dependency Security

### Rust (Cargo)

```bash
# Install cargo-audit
cargo install cargo-audit

# Run audit
cargo audit
```

### Python (pip)

```bash
# Install pip-audit
pip install pip-audit

# Run audit
pip-audit
```

### Node.js (npm)

```bash
# Built-in audit
npm audit
```

## Threat Model

### In Scope

- Malicious agent actions
- Prompt injection attacks
- Data exfiltration attempts
- Privilege escalation
- Denial of service

### Mitigations

| Threat | Mitigation |
|--------|------------|
| Malicious commands | Risk scoring + policy |
| File system abuse | Path restrictions |
| Network exfil | Domain allow-lists |
| Resource abuse | Capability limits |
| Credential theft | No credential access |

## Compliance

AEON supports compliance with:

- **ISO 27001** - Information security
- **GDPR** - Data protection (audit logs)
- **NIST** - Cybersecurity framework
- **SOC 2** - Security controls

The audit ledger provides complete traceability for all agent actions.
