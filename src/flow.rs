use crate::node::Node;
use crate::error::FlowError;
use serde_json::Value;
use futures::future::join_all;

pub struct Flow {
    nodes: Vec<Box<dyn Node>>,
}

impl Flow {
    pub fn new(nodes: Vec<Box<dyn Node>>) -> Self {
        Self { nodes }
    }

    pub async fn execute(&self, mut input: Value) -> Result<Value, FlowError> {
        for node in &self.nodes {
            input = node.call(input).await?;
        }
        Ok(input)
    }
}

pub struct ParallelFlow {
    nodes: Vec<Box<dyn Node>>,
}

impl ParallelFlow {
    pub fn new(nodes: Vec<Box<dyn Node>>) -> Self {
        Self { nodes }
    }

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