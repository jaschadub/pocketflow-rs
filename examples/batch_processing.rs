use async_trait::async_trait;
use pocketflow_rs::batch::Batch;
use pocketflow_rs::flow::Flow;
use pocketflow_rs::node::Node;
use pocketflow_rs::error::FlowError;
use serde_json::{json, Value};

// A simple node that appends a suffix to a string
#[derive(Clone)]
struct StringAppenderNode {
    suffix: String,
}

#[async_trait]
impl Node for StringAppenderNode {
    async fn call(&self, input: Value) -> Result<Value, FlowError> {
        if let Some(s) = input.as_str() {
            Ok(json!(format!("{}{}", s, self.suffix)))
        } else {
            Err(FlowError::NodeFailed("Input must be a string".to_string()))
        }
    }
}

#[tokio::main]
async fn main() {
    // The node that will process each item in the batch
    let appender_node = StringAppenderNode {
        suffix: "_processed".to_string(),
    };

    // Wrap the node in a Batch processor
    let batch_node = Batch::new(appender_node);

    // Create a flow with the batch node
    let flow = Flow::new(vec![Box::new(batch_node)]);

    // The input is an array of strings
    let initial_input = json!(["item1", "item2", "item3"]);
    println!("Starting batch processing flow...");
    println!("Initial Input: {}", initial_input);

    match flow.execute(initial_input).await {
        Ok(output) => {
            println!("\nBatch flow executed successfully!");
            println!("Output:\n{}", serde_json::to_string_pretty(&output).unwrap());
        }
        Err(e) => {
            eprintln!("Error executing batch flow: {}", e);
        }
    }
}