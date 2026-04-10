//! Protocol Validation.
//!
//! Implements strict validation according to FC-03 specification.
//! Validator enforces:
//! - Valid JSON structure
//! - Correct protocol form (message OR tool_call, not both)
//! - Valid capability identifier
//! - Valid argument structure

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("invalid JSON: {0}")]
    InvalidJson(#[from] serde_json::Error),
    #[error("protocol violation: {0}")]
    ProtocolViolation(String),
    #[error("missing required field: {0}")]
    MissingField(String),
    #[error("invalid field type: {0}")]
    InvalidFieldType(String),
    #[error("unknown field: {0}")]
    UnknownField(String),
}

pub struct Validator {
    known_fields: Vec<&'static str>,
}

impl Validator {
    pub fn new() -> Self {
        Self {
            known_fields: vec!["message", "tool_call"],
        }
    }

    pub fn validate(
        &self,
        value: &serde_json::Value,
    ) -> Result<crate::message::ProtocolOutput, ValidationError> {
        let obj = value.as_object().ok_or_else(|| {
            ValidationError::ProtocolViolation("must be a JSON object".to_string())
        })?;

        let has_message = obj.contains_key("message");
        let has_tool_call = obj.contains_key("tool_call");

        if has_message && has_tool_call {
            return Err(ValidationError::ProtocolViolation(
                "protocol messages must contain exactly one of 'message' or 'tool_call', not both"
                    .to_string(),
            ));
        }

        if !has_message && !has_tool_call {
            return Err(ValidationError::ProtocolViolation(
                "protocol messages must contain either 'message' or 'tool_call'".to_string(),
            ));
        }

        for key in obj.keys() {
            if !self.known_fields.contains(&key.as_str()) {
                return Err(ValidationError::UnknownField(format!(
                    "unknown field '{}' - protocol messages must only contain 'message' or 'tool_call'", key
                )));
            }
        }

        if has_message {
            self.validate_message_form(obj)?;
            let output = crate::message::ProtocolOutput::Message(crate::message::MessageForm {
                message: crate::message::MessageContent {
                    content: obj["message"]["content"]
                        .as_str()
                        .ok_or_else(|| {
                            ValidationError::InvalidFieldType(
                                "message.content must be a string".to_string(),
                            )
                        })?
                        .to_string(),
                },
            });
            return Ok(output);
        }

        self.validate_tool_call_form(obj)?;
        let output = crate::message::ProtocolOutput::ToolCall(crate::message::ToolCallWrapper {
            tool_call: crate::message::ToolCallForm {
                tool: obj["tool_call"]["tool"]
                    .as_str()
                    .ok_or_else(|| {
                        ValidationError::InvalidFieldType(
                            "tool_call.tool must be a string".to_string(),
                        )
                    })?
                    .to_string(),
                args: obj["tool_call"]["args"].clone(),
            },
        });
        Ok(output)
    }

    fn validate_message_form(
        &self,
        obj: &serde_json::Map<String, serde_json::Value>,
    ) -> Result<(), ValidationError> {
        let message = obj
            .get("message")
            .ok_or_else(|| ValidationError::MissingField("message".to_string()))?;

        let message_obj = message.as_object().ok_or_else(|| {
            ValidationError::ProtocolViolation("message must be an object".to_string())
        })?;

        if !message_obj.contains_key("content") {
            return Err(ValidationError::MissingField("message.content".to_string()));
        }

        let content = message_obj["content"].as_str().ok_or_else(|| {
            ValidationError::InvalidFieldType("message.content must be a string".to_string())
        })?;

        if content.is_empty() {
            return Err(ValidationError::ProtocolViolation(
                "message.content must not be empty".to_string(),
            ));
        }

        Ok(())
    }

    fn validate_tool_call_form(
        &self,
        obj: &serde_json::Map<String, serde_json::Value>,
    ) -> Result<(), ValidationError> {
        let tool_call = obj
            .get("tool_call")
            .ok_or_else(|| ValidationError::MissingField("tool_call".to_string()))?;

        let tc_obj = tool_call.as_object().ok_or_else(|| {
            ValidationError::ProtocolViolation("tool_call must be an object".to_string())
        })?;

        if !tc_obj.contains_key("tool") {
            return Err(ValidationError::MissingField("tool_call.tool".to_string()));
        }

        let tool = tc_obj["tool"].as_str().ok_or_else(|| {
            ValidationError::InvalidFieldType("tool_call.tool must be a string".to_string())
        })?;

        if tool.is_empty() {
            return Err(ValidationError::ProtocolViolation(
                "tool_call.tool must not be empty".to_string(),
            ));
        }

        if !tc_obj.contains_key("args") {
            return Err(ValidationError::MissingField("tool_call.args".to_string()));
        }

        if !tc_obj["args"].is_object() {
            return Err(ValidationError::InvalidFieldType(
                "tool_call.args must be an object".to_string(),
            ));
        }

        Ok(())
    }

    pub fn validate_str(
        &self,
        input: &str,
    ) -> Result<crate::message::ProtocolOutput, ValidationError> {
        let value: serde_json::Value = serde_json::from_str(input)?;
        self.validate(&value)
    }
}

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_valid_message() {
        let validator = Validator::new();
        let input = serde_json::json!({"message": {"content": "Hello"}});
        let result = validator.validate(&input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_valid_tool_call() {
        let validator = Validator::new();
        let input = serde_json::json!({"tool_call": {"tool": "fs.read", "args": {}}});
        let result = validator.validate(&input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_reject_both_fields() {
        let validator = Validator::new();
        let input = serde_json::json!({"message": {"content": "x"}, "tool_call": {"tool": "x", "args": {}}});
        let result = validator.validate(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_reject_unknown_field() {
        let validator = Validator::new();
        let input = serde_json::json!({"message": {"content": "x"}, "unknown": "value"});
        let result = validator.validate(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_reject_missing_content() {
        let validator = Validator::new();
        let input = serde_json::json!({"message": {}});
        let result = validator.validate(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_reject_empty_tool() {
        let validator = Validator::new();
        let input = serde_json::json!({"tool_call": {"tool": "", "args": {}}});
        let result = validator.validate(&input);
        assert!(result.is_err());
    }
}
