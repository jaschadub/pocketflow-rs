use async_trait::async_trait;
use serde_json::Value;
use crate::error::FlowError;

#[async_trait]
pub trait Node: Send + Sync {
    async fn call(&self, input: Value) -> Result<Value, FlowError>;
}