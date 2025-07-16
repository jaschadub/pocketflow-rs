use async_trait::async_trait;
use rustyflow::flow::Flow;
use rustyflow::node::Node;
use rustyflow::tool::{Tool, ToolNode};
use rustyflow::error::FlowError;
use serde::{Deserialize, Serialize};
use serde_json::json;

// Define a structured input for our tool
#[derive(Deserialize)]
struct AddRequest {
    a: i32,
    b: i32,
}

// Define a structured output for our tool
#[derive(Serialize)]
struct AddResponse {
    result: i32,
}

// The tool itself, implementing the type-safe Tool trait
struct AddTool;

#[async_trait]
impl Tool for AddTool {
    type Input = AddRequest;
    type Output = AddResponse;

    async fn run(&self, input: Self::Input) -> Result<Self::Output, FlowError> {
        let result = input.a + input.b;
        println!("AddTool executed: {} + {} = {}", input.a, input.b, result);
        Ok(AddResponse { result })
    }
}

#[tokio::main]
async fn main() {
    // Create an instance of our type-safe tool
    let add_tool = AddTool;

    // Wrap it in ToolNode to make it compatible with the Flow system
    let tool_node: Box<dyn Node> = Box::new(ToolNode::new(add_tool));

    // The ToolNode can now be used in a regular Flow
    let flow = Flow::new(vec![tool_node]);

    // The input is a dynamic JSON value, but the tool receives a typed struct
    let initial_input = json!({ "a": 10, "b": 5 });
    println!("Starting tool node flow...");
    println!("Initial Input: {}", initial_input);

    match flow.execute(initial_input).await {
        Ok(output) => {
            println!("\nTool flow executed successfully!");
            println!("Output:\n{}", serde_json::to_string_pretty(&output).unwrap());
        }
        Err(e) => {
            eprintln!("Error executing tool flow: {}", e);
        }
    }
}