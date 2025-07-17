//! Core node abstraction for RustyFlow.
//!
//! This module defines the fundamental [`Node`] trait that all computation
//! units in RustyFlow must implement.

use crate::error::FlowError;
use async_trait::async_trait;
use serde_json::Value;

/// The fundamental building block for all computations in RustyFlow.
///
/// A `Node` represents a single computation step that takes a JSON value as input
/// and produces a JSON value as output. Nodes are composable and can be chained
/// together in flows for complex processing pipelines.
///
/// # Example
///
/// ```rust
/// use async_trait::async_trait;
/// use rustyflow::{Node, FlowError};
/// use serde_json::{json, Value};
///
/// struct MultiplyNode {
///     factor: f64,
/// }
///
/// #[async_trait]
/// impl Node for MultiplyNode {
///     async fn call(&self, input: Value) -> Result<Value, FlowError> {
///         let number = input["value"].as_f64()
///             .ok_or_else(|| FlowError::NodeFailed("Expected 'value' field".to_string()))?;
///         Ok(json!({"value": number * self.factor}))
///     }
/// }
/// ```
#[async_trait]
pub trait Node: Send + Sync {
    /// Execute the node with the given input.
    ///
    /// This method is called when the node is executed as part of a flow.
    /// The input is a JSON value that may contain any structured data.
    ///
    /// # Arguments
    ///
    /// * `input` - The JSON input value to process
    ///
    /// # Returns
    ///
    /// * `Ok(Value)` - The processed output as a JSON value
    /// * `Err(FlowError)` - An error if processing fails
    async fn call(&self, input: Value) -> Result<Value, FlowError>;
}
