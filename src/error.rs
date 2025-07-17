//! Error types for RustyFlow operations.

use thiserror::Error;

/// Error types that can occur during flow execution.
///
/// This enum represents all possible errors that can happen when executing
/// nodes, flows, or other RustyFlow operations.
#[derive(Error, Debug)]
pub enum FlowError {
    /// A node failed to execute properly.
    ///
    /// This error occurs when a node's `call` method returns an error
    /// or when validation fails.
    #[error("Node execution failed: {0}")]
    NodeFailed(String),

    /// JSON serialization or deserialization failed.
    ///
    /// This error occurs when converting between JSON values and typed
    /// data structures fails.
    #[error("Data serialization/deserialization error: {0}")]
    SerdeError(#[from] serde_json::Error),

    /// An unknown error occurred.
    ///
    /// This is a catch-all for unexpected errors.
    #[error("An unknown error occurred")]
    Unknown,
}
