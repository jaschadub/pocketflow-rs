use async_trait::async_trait;
use serde::Serialize;
use serde_json::Value;
use crate::error::FlowError;
use crate::node::Node;

/// A trait for type-safe tools that work with structured inputs and outputs
#[async_trait]
pub trait Tool: Send + Sync {
    /// The input type for this tool, must be deserializable
    type Input: serde::de::DeserializeOwned + Send + Sync;
    /// The output type for this tool, must be serializable
    type Output: Serialize + Send + Sync;

    /// Execute the tool with typed input and return typed output
    async fn run(&self, input: Self::Input) -> Result<Self::Output, FlowError>;
}

/// A wrapper that allows type-safe Tools to be used as Nodes in the Flow system
pub struct ToolNode<T: Tool> {
    tool: T,
}

impl<T: Tool> ToolNode<T> {
    /// Create a new ToolNode wrapping the given tool
    pub fn new(tool: T) -> Self {
        Self { tool }
    }
}

#[async_trait]
impl<T: Tool> Node for ToolNode<T> {
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