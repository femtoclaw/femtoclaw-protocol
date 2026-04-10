//! FemtoClaw Protocol Validation Library.
//!
//! Provides strict JSON schema validation for runtime messages according to
//! FemtoClaw Protocol Specification (FC-03).

pub mod message;
pub mod schema;
pub mod validation;

pub use message::{MessageContent, MessageForm, ProtocolOutput, ToolCallForm, ToolCallWrapper};
pub use validation::{ValidationError, Validator};
