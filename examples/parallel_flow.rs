use async_trait::async_trait;
use pocketflow_rs::flow::ParallelFlow;
use pocketflow_rs::node::Node;
use pocketflow_rs::error::FlowError;
use serde_json::{json, Value};
use std::time::Duration;
use tokio::time::sleep;

// A simple node that waits and then returns a value
struct WaitAndReturnNode {
    id: u32,
    wait_ms: u64,
}

#[async_trait]
impl Node for WaitAndReturnNode {
    async fn call(&self, input: Value) -> Result<Value, FlowError> {
        println!("Node {} received input: {}", self.id, input);
        sleep(Duration::from_millis(self.wait_ms)).await;
        let result = json!({
            "id": self.id,
            "status": "done"
        });
        println!("Node {} finished.", self.id);
        Ok(result)
    }
}

#[tokio::main]
async fn main() {
    let nodes: Vec<Box<dyn Node>> = vec![
        Box::new(WaitAndReturnNode { id: 1, wait_ms: 1000 }),
        Box::new(WaitAndReturnNode { id: 2, wait_ms: 500 }),
        Box::new(WaitAndReturnNode { id: 3, wait_ms: 750 }),
    ];

    let parallel_flow = ParallelFlow::new(nodes);

    let initial_input = json!({"start": true});
    println!("Starting parallel flow...");

    match parallel_flow.execute(initial_input).await {
        Ok(output) => {
            println!("\nParallel flow executed successfully!");
            println!("Output:\n{}", serde_json::to_string_pretty(&output).unwrap());
        }
        Err(e) => {
            eprintln!("Error executing parallel flow: {}", e);
        }
    }
}