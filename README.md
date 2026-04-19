# 📜 FemtoClaw Protocol

[![Rust](https://img.shields.io/badge/rust-1.75%2B-blue.svg)](https://www.rust-lang.org)
[![Status](https://img.shields.io/badge/Status-Normative-green.svg)]()

The **FemtoClaw Protocol** is the normative interface between probabilistic inference (Brains) and deterministic execution (Runtime). It enforces strict JSON serialization, eliminating the parsing ambiguities inherent in natural language or markdown-based agent communication.

---

## 🛡️ Protocol Invariants (FC-03)

To ensure industrial-grade reliability, the protocol enforces the following invariants:

1.  **Strict JSON**: All Brain outputs MUST be valid JSON. Markdown wrappers (e.g., \````json\`) are strictly rejected by the validator to prevent injection attacks and parsing failures.
2.  **Mutual Exclusion**: A protocol message MUST contain exactly one of \`message\` or \`tool_call\`. This ensures that an agent is either communicating with the user or performing a system action, never both simultaneously in a single step.
3.  **Capability Binding**: Tool calls must bind to a registered and authorized capability identifier. Unknown or unauthorized tools are rejected at the protocol level.

---

## 🧱 Message Forms

### 1. Message (Communication)
Used for final user responses or intermediate internal reasoning steps.
```json
{
  "message": {
    "content": "The system audit is complete. No vulnerabilities were detected."
  }
}
```

### 2. Tool Call (Action)
Used to invoke a specific system capability (Claw).
```json
{
  "tool_call": {
    "tool": "shell",
    "args": {
      "bin": "ls",
      "argv": ["-la", "/var/log/audit"]
    }
  }
}
```

---

## 🧪 Validation & Enforcement

The \`Validator\` provided by this crate is the primary security gatekeeper. It ensures that every inference step conforms to the engineering specification before it reaches the policy engine.

```rust
use femtoclaw_protocol::Validator;

let validator = Validator::new();
let result = validator.validate_str(raw_brain_output)?;
```

---

## 📄 Related Specifications
- **[FC-03: Protocol Specification](../femtoclaw-spec/03-FemtoClaw_Protocol_Specification.md)**
- **[FC-05: Capability Authorization](../femtoclaw-spec/05-FemtoClaw_Capability_Authorization_and_Policy_Specification.md)**

Copyright © 2026 FemtoClaw Project.
