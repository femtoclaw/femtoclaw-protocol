//! FemtoClaw Protocol Message Types.
//!
//! Implements protocol messages according to FC-03 specification.
//! Two mutually exclusive forms: Message and ToolCall.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageContent {
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageForm {
    pub message: MessageContent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallArgs {
    pub args: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallForm {
    pub tool: String,
    pub args: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallWrapper {
    pub tool_call: ToolCallForm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProtocolOutput {
    Message(MessageForm),
    ToolCall(ToolCallWrapper),
}

impl ProtocolOutput {
    pub fn parse(input: &str) -> Result<Self, crate::validation::ValidationError> {
        let value: serde_json::Value = serde_json::from_str(input)?;
        crate::validation::Validator::new().validate(&value)
    }

    pub fn is_message(&self) -> bool {
        matches!(self, ProtocolOutput::Message(_))
    }

    pub fn is_tool_call(&self) -> bool {
        matches!(self, ProtocolOutput::ToolCall(_))
    }

    pub fn into_message(self) -> Option<String> {
        match self {
            ProtocolOutput::Message(m) => Some(m.message.content),
            ProtocolOutput::ToolCall(_) => None,
        }
    }

    pub fn into_tool_call(self) -> Option<(String, serde_json::Value)> {
        match self {
            ProtocolOutput::Message(_) => None,
            ProtocolOutput::ToolCall(tc) => Some((tc.tool_call.tool, tc.tool_call.args)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_message_form() {
        let input = r#"{"message":{"content":"Hello, world"}}"#;
        let output = ProtocolOutput::parse(input).unwrap();
        assert!(output.is_message());
        assert_eq!(output.into_message().unwrap(), "Hello, world");
    }

    #[test]
    fn test_parse_tool_call_form() {
        let input = r#"{"tool_call":{"tool":"web.get","args":{"url":"https://example.com"}}}"#;
        let output = ProtocolOutput::parse(input).unwrap();
        assert!(output.is_tool_call());
        let (tool, args) = output.into_tool_call().unwrap();
        assert_eq!(tool, "web.get");
        assert_eq!(args["url"], "https://example.com");
    }

    #[test]
    fn test_reject_both_forms() {
        let input = r#"{"message":{"content":"test"},"tool_call":{"tool":"test","args":{}}}"#;
        let result = ProtocolOutput::parse(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_reject_invalid_json() {
        let input = "not valid json";
        let result = ProtocolOutput::parse(input);
        assert!(result.is_err());
    }
}
