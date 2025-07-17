//! Type-safe tools with structured input and output.
//!
//! This module provides the [`Tool`] trait for type-safe operations and
//! [`ToolNode`] for integrating tools into flows.

use crate::error::FlowError;
use crate::node::Node;
use async_trait::async_trait;
use serde::Serialize;
use serde_json::Value;

/// A trait for type-safe tools that work with structured inputs and outputs.
///
/// `Tool` provides compile-time type safety by defining specific input and output
/// types, unlike the generic JSON-based [`Node`] trait. This enables better
/// validation and developer experience.
///
/// # Example
///
/// ```rust
/// use async_trait::async_trait;
/// use serde::{Deserialize, Serialize};
/// use rustyflow::{Tool, ToolNode, FlowError};
///
/// #[derive(Deserialize)]
/// struct MathInput {
///     a: f64,
///     b: f64,
///     operation: String,
/// }
///
/// #[derive(Serialize)]
/// struct MathOutput {
///     result: f64,
/// }
///
/// struct Calculator;
///
/// #[async_trait]
/// impl Tool for Calculator {
///     type Input = MathInput;
///     type Output = MathOutput;
///
///     async fn run(&self, input: Self::Input) -> Result<Self::Output, FlowError> {
///         let result = match input.operation.as_str() {
///             "add" => input.a + input.b,
///             "multiply" => input.a * input.b,
///             _ => return Err(FlowError::NodeFailed("Unknown operation".to_string())),
///         };
///         Ok(MathOutput { result })
///     }
/// }
///
/// // Use as a node in a flow
/// let tool_node = ToolNode::new(Calculator);
/// ```
#[async_trait]
pub trait Tool: Send + Sync {
    /// The input type for this tool, must be deserializable from JSON.
    type Input: serde::de::DeserializeOwned + Send + Sync;

    /// The output type for this tool, must be serializable to JSON.
    type Output: Serialize + Send + Sync;

    /// Execute the tool with typed input and return typed output.
    ///
    /// # Arguments
    ///
    /// * `input` - The typed input data for the tool
    ///
    /// # Returns
    ///
    /// * `Ok(Self::Output)` - The successful result of the tool execution
    /// * `Err(FlowError)` - An error if the tool execution fails
    async fn run(&self, input: Self::Input) -> Result<Self::Output, FlowError>;
}

/// A wrapper that allows type-safe Tools to be used as Nodes in the Flow system.
///
/// `ToolNode` bridges the gap between the type-safe [`Tool`] trait and the
/// JSON-based [`Node`] trait, providing automatic serialization and deserialization.
///
/// # Example
///
/// ```rust
/// use serde::{Deserialize, Serialize};
/// use rustyflow::{Flow, ToolNode};
/// # use rustyflow::{Tool, FlowError};
/// # use async_trait::async_trait;
/// # #[derive(Deserialize)] struct Input { value: i32 }
/// # #[derive(Serialize)] struct Output { doubled: i32 }
/// # struct Doubler;
/// # #[async_trait]
/// # impl Tool for Doubler {
/// #     type Input = Input;
/// #     type Output = Output;
/// #     async fn run(&self, input: Self::Input) -> Result<Self::Output, FlowError> {
/// #         Ok(Output { doubled: input.value * 2 })
/// #     }
/// # }
///
/// let flow = Flow::new(vec![
///     Box::new(ToolNode::new(Doubler)),
/// ]);
/// ```
pub struct ToolNode<T: Tool> {
    tool: T,
}

impl<T: Tool> ToolNode<T> {
    /// Create a new ToolNode wrapping the given tool.
    ///
    /// # Arguments
    ///
    /// * `tool` - The tool instance to wrap
    ///
    /// # Returns
    ///
    /// A new `ToolNode` that can be used in flows
    pub fn new(tool: T) -> Self {
        Self { tool }
    }
}

#[async_trait]
impl<T: Tool> Node for ToolNode<T> {
    /// Execute the wrapped tool with automatic JSON conversion.
    ///
    /// This method handles the conversion between JSON values and the tool's
    /// typed input/output, providing a seamless bridge between type-safe tools
    /// and the flow system.
    ///
    /// # Arguments
    ///
    /// * `input` - JSON input that will be deserialized to the tool's input type
    ///
    /// # Returns
    ///
    /// * `Ok(Value)` - The tool's output serialized to JSON
    /// * `Err(FlowError)` - Deserialization, tool execution, or serialization error
    async fn call(&self, input: Value) -> Result<Value, FlowError> {
        // Deserialize the JSON value into the tool's input type
        let typed_input: T::Input = serde_json::from_value(input)?;

        // Execute the tool with the typed input
        let typed_output = self.tool.run(typed_input).await?;

        // Serialize the typed output back to a JSON value
        let output_value = serde_json::to_value(typed_output)?;

        Ok(output_value)
    }
}
