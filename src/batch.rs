//! Batch processing for concurrent array operations.
//!
//! This module provides the [`Batch`] wrapper that applies a node to each
//! element of a JSON array concurrently.

use async_trait::async_trait;
use futures::future::join_all;
use serde_json::Value;
use crate::error::FlowError;
use crate::node::Node;

/// A wrapper node that applies another node to each element of a JSON array concurrently.
///
/// `Batch` takes any node and applies it to each element of a JSON array in parallel,
/// collecting the results back into an array. This is useful for processing large
/// datasets efficiently.
///
/// # Example
///
/// ```rust
/// use rustyflow::{Batch, Node, FlowError};
/// use serde_json::{json, Value};
/// use async_trait::async_trait;
///
/// struct UppercaseNode;
///
/// #[async_trait]
/// impl Node for UppercaseNode {
///     async fn call(&self, input: Value) -> Result<Value, FlowError> {
///         if let Some(text) = input.as_str() {
///             Ok(Value::String(text.to_uppercase()))
///         } else {
///             Err(FlowError::NodeFailed("Expected string input".to_string()))
///         }
///     }
/// }
///
/// # async fn example() -> Result<(), FlowError> {
/// let batch_node = Batch::new(UppercaseNode);
/// let input = json!(["hello", "world", "rust"]);
/// let result = batch_node.call(input).await?;
/// // Result: ["HELLO", "WORLD", "RUST"]
/// # Ok(())
/// # }
/// ```
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
    /// Creates a new Batch node that wraps the given node.
    ///
    /// # Arguments
    ///
    /// * `wrapped_node` - The node to apply to each array element
    ///
    /// # Returns
    ///
    /// A new `Batch` instance that will process arrays concurrently
    pub fn new(wrapped_node: T) -> Self {
        Self { wrapped_node }
    }
}

#[async_trait]
impl<T> Node for Batch<T>
where
    T: Node,
{
    /// Apply the wrapped node to each element of the input array concurrently.
    ///
    /// # Arguments
    ///
    /// * `input` - Must be a JSON array; each element will be processed
    ///
    /// # Returns
    ///
    /// * `Ok(Value::Array)` - Array of processed results in the same order
    /// * `Err(FlowError)` - Error if input is not an array or any element fails
    ///
    /// # Errors
    ///
    /// Returns `FlowError::NodeFailed` if the input is not a JSON array,
    /// or propagates any error from the wrapped node.
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