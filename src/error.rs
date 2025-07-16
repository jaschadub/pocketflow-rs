use thiserror::Error;

#[derive(Error, Debug)]
pub enum FlowError {
    #[error("Node execution failed: {0}")]
    NodeFailed(String),

    #[error("Data serialization/deserialization error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("An unknown error occurred")]
    Unknown,
}