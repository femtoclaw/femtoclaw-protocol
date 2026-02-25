//! Protocol Validation.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("invalid JSON: {0}")]
    InvalidJson(#[from] serde_json::Error),
    #[error("schema validation failed: {0}")]
    SchemaFailed(String),
    #[error("protocol violation: {0}")]
    ProtocolViolation(String),
}

pub struct Validator;

impl Validator {
    pub fn new() -> Self {
        Self
    }

    pub fn validate(&self, input: &str) -> Result<serde_json::Value, ValidationError> {
        let value: serde_json::Value = serde_json::from_str(input)?;
        Ok(value)
    }
}

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}
