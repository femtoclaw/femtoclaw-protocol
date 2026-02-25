# FemtoClaw Protocol

[![Rust](https://img.shields.io/badge/rust-1.75%2B-blue.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)
[![Status](https://img.shields.io/badge/Status-Normative-green.svg)]()

FemtoClaw Protocol Validation Library — strict JSON schema validation for runtime messages.

## Overview

`femtoclaw-protocol` provides the core protocol validation layer for the FemtoClaw Industrial Agent Runtime. It implements strict JSON schema validation according to the [FemtoClaw Protocol Specification (FC-03)](../femtoclaw-spec/03-FemtoClaw_Protocol_Specification.md).

This library ensures that all communication between probabilistic inference systems and the deterministic FemtoClaw runtime authority conforms to the protocol requirements, eliminating natural language execution ambiguity.

## Features

- **Strict JSON Validation**: Validate all protocol messages against defined schemas
- **Message Parsing**: Parse and validate `Message` and `ToolCall` protocol outputs
- **Schema Definitions**: JSON schema definitions for protocol compliance
- **Error Reporting**: Detailed validation error messages for debugging

## Protocol

The FemtoClaw Protocol defines two mutually exclusive output forms:

1. **Message Form**: `{ "message": { "content": "..." } }`
2. **Tool Call Form**: `{ "tool_call": { "tool": "...", "args": {...} } }`

Only one form may be present per protocol message.

## Installation

```toml
[dependencies]
femtoclaw-protocol = "1.0"
```

## Usage

```rust
use femtoclaw_protocol::{Validator, Message, ToolCall};

let validator = Validator::new();

// Validate incoming protocol message
let input = r#"{"message":{"content":"Hello, world"}}"#;
let value = validator.validate(input)?;

println!("Validated: {:?}", value);
```

## Architecture

```
┌─────────────────────────────────────────────┐
│         Inference System (LLM)              │
└─────────────────┬───────────────────────────┘
                  │ JSON Protocol Message
                  ▼
┌─────────────────────────────────────────────┐
│         femtoclaw-protocol                  │
│  ┌─────────────────────────────────────┐   │
│  │  Schema Validation                  │   │
│  │  - Message Schema                   │   │
│  │  - ToolCall Schema                  │   │
│  └─────────────────────────────────────┘   │
└─────────────────┬───────────────────────────┘
                  │ Validated Message
                  ▼
┌─────────────────────────────────────────────┐
│         FemtoClaw Agent Core                │
└─────────────────────────────────────────────┘
```

## Modules

- `message` — Protocol message types (Message, ToolCall, ProtocolOutput)
- `schema` — JSON schema definitions for validation
- `validation` — Validator implementation and error types

## Requirements

- Rust 1.75 or later
- serde 1.x
- serde_json 1.x
- thiserror 1.x

## Related Specifications

- [FC-03: Protocol Specification](../femtoclaw-spec/03-FemtoClaw_Protocol_Specification.md)
- [FC-05: Capability Authorization](../femtoclaw-spec/05-FemtoClaw_Capability_Authorization_and_Policy_Specification.md)
- [FC-08: Observability and Telemetry](../femtoclaw-spec/08-FemtoClaw_Observability_and_Telemetry_Specification.md)

## License

Copyright 2026 FemtoClaw

Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.
