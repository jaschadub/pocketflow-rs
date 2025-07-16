use async_trait::async_trait;
use futures::future::join_all;
use serde_json::Value;
use crate::error::FlowError;
use crate::node::Node;

/// A wrapper node that applies another node to each element of a JSON array concurrently
pub struct Batch<T>
where
    T: Node,
{
    wrapped_node: T,
}

impl<T> Batch<T>
where
    T: Node,
{
    /// Creates a new Batch node that wraps the given node
    pub fn new(wrapped_node: T) -> Self {
        Self { wrapped_node }
    }
}

#[async_trait]
impl<T> Node for Batch<T>
where
    T: Node,
{
    async fn call(&self, input: Value) -> Result<Value, FlowError> {
        // Ensure input is an array
        let array = match input.as_array() {
            Some(arr) => arr,
            None => return Err(FlowError::NodeFailed("Input must be a JSON array".to_string())),
        };

        // Create futures for processing each element
        let futures: Vec<_> = array
            .iter()
            .map(|element| self.wrapped_node.call(element.clone()))
            .collect();

        // Execute all operations concurrently
        let results = join_all(futures).await;

        // Collect successful results or return first error
        let mut values = Vec::new();
        for result in results {
            values.push(result?);
        }

        // Return as JSON array
        Ok(Value::Array(values))
    }
}