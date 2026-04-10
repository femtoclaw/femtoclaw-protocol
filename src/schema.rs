//! Protocol JSON Schema Definitions.

pub const MESSAGE_SCHEMA: &str = r#"{
    "type": "object",
    "properties": {
        "message": {
            "type": "object",
            "properties": {
                "content": { "type": "string" }
            },
            "required": ["content"]
        }
    },
    "required": ["message"]
}"#;

pub const TOOL_CALL_SCHEMA: &str = r#"{
    "type": "object",
    "properties": {
        "tool_call": {
            "type": "object",
            "properties": {
                "tool": { "type": "string" },
                "args": { "type": "object" }
            },
            "required": ["tool", "args"]
        }
    },
    "required": ["tool_call"]
}"#;
