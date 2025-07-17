//! Flow orchestration for sequential and parallel execution.
//!
//! This module provides the core flow types for organizing nodes into
//! execution pipelines.

use crate::node::Node;
use crate::error::FlowError;
use serde_json::Value;
use futures::future::join_all;

/// A sequential execution pipeline for nodes.
///
/// `Flow` executes nodes one after another, passing the output of each node
/// as the input to the next node in the sequence.
///
/// # Example
///
/// ```rust
/// use rustyflow::{Flow, Node, FlowError};
/// use serde_json::{json, Value};
/// use async_trait::async_trait;
///
/// struct AddNode(i32);
///
/// #[async_trait]
/// impl Node for AddNode {
///     async fn call(&self, input: Value) -> Result<Value, FlowError> {
///         let num = input["value"].as_i64().unwrap_or(0) as i32;
///         Ok(json!({"value": num + self.0}))
///     }
/// }
///
/// # async fn example() -> Result<(), FlowError> {
/// let flow = Flow::new(vec![
///     Box::new(AddNode(5)),
///     Box::new(AddNode(10)),
/// ]);
///
/// let result = flow.execute(json!({"value": 0})).await?;
/// assert_eq!(result["value"], 15);
/// # Ok(())
/// # }
/// ```
pub struct Flow {
    nodes: Vec<Box<dyn Node>>,
}

impl Flow {
    /// Create a new sequential flow with the given nodes.
    ///
    /// # Arguments
    ///
    /// * `nodes` - Vector of boxed nodes to execute in sequence
    pub fn new(nodes: Vec<Box<dyn Node>>) -> Self {
        Self { nodes }
    }

    /// Execute the flow with the given input.
    ///
    /// Nodes are executed sequentially, with each node's output becoming
    /// the input for the next node.
    ///
    /// # Arguments
    ///
    /// * `input` - The initial input value for the flow
    ///
    /// # Returns
    ///
    /// The final output after all nodes have been executed, or the first
    /// error encountered.
    pub async fn execute(&self, mut input: Value) -> Result<Value, FlowError> {
        for node in &self.nodes {
            input = node.call(input).await?;
        }
        Ok(input)
    }
}

/// A parallel execution pipeline for nodes.
///
/// `ParallelFlow` executes all nodes concurrently with the same input,
/// collecting their outputs into a JSON array.
///
/// # Example
///
/// ```rust
/// use rustyflow::{ParallelFlow, Node, FlowError};
/// use serde_json::{json, Value};
/// use async_trait::async_trait;
///
/// struct ProcessorNode {
///     name: String,
/// }
///
/// #[async_trait]
/// impl Node for ProcessorNode {
///     async fn call(&self, input: Value) -> Result<Value, FlowError> {
///         Ok(json!({"processor": self.name, "data": input}))
///     }
/// }
///
/// # async fn example() -> Result<(), FlowError> {
/// let parallel_flow = ParallelFlow::new(vec![
///     Box::new(ProcessorNode { name: "A".to_string() }),
///     Box::new(ProcessorNode { name: "B".to_string() }),
/// ]);
///
/// let result = parallel_flow.execute(json!({"value": 42})).await?;
/// // Result is an array with outputs from both processors
/// # Ok(())
/// # }
/// ```
pub struct ParallelFlow {
    nodes: Vec<Box<dyn Node>>,
}

impl ParallelFlow {
    /// Create a new parallel flow with the given nodes.
    ///
    /// # Arguments
    ///
    /// * `nodes` - Vector of boxed nodes to execute in parallel
    pub fn new(nodes: Vec<Box<dyn Node>>) -> Self {
        Self { nodes }
    }

    /// Execute all nodes in parallel with the same input.
    ///
    /// Each node receives a clone of the input and executes concurrently.
    /// Results are collected into a JSON array in the same order as the nodes.
    ///
    /// # Arguments
    ///
    /// * `input` - The input value to pass to all nodes
    ///
    /// # Returns
    ///
    /// A JSON array containing the outputs from all nodes, or the first
    /// error encountered.
    pub async fn execute(&self, input: Value) -> Result<Value, FlowError> {
        // Create futures for all nodes, each receiving a clone of the input
        let futures: Vec<_> = self.nodes
            .iter()
            .map(|node| node.call(input.clone()))
            .collect();

        // Execute all nodes concurrently
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