use async_trait::async_trait;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::post,
    Router,
};
use rustyflow::{
    error::FlowError,
    flow::Flow,
    node::Node,
    tool::{Tool, ToolNode},
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// --- Tool Definition (could be in its own module) ---

#[derive(Deserialize)]
struct AddRequest {
    a: i32,
    b: i32,
}

#[derive(Serialize)]
struct AddResponse {
    result: i32,
}

struct AddTool;

#[async_trait]
impl Tool for AddTool {
    type Input = AddRequest;
    type Output = AddResponse;

    async fn run(&self, input: Self::Input) -> Result<Self::Output, FlowError> {
        let result = input.a + input.b;
        Ok(AddResponse { result })
    }
}

// --- Axum Handler ---

async fn execute_flow(
    State(flow): State<Arc<Flow>>,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    tracing::info!("Received request with payload: {:?}", payload);
    match flow.execute(payload).await {
        Ok(result) => {
            tracing::info!("Flow executed successfully with result: {:?}", result);
            (StatusCode::OK, Json(result))
        }
        Err(e) => {
            tracing::error!("Flow execution failed: {}", e);
            let error_response = json!({ "error": e.to_string() });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        }
    }
}

// --- Main Server Setup ---

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rustyflow=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create a reusable flow instance
    let add_tool = AddTool;
    let tool_node: Box<dyn Node> = Box::new(ToolNode::new(add_tool));
    let flow = Arc::new(Flow::new(vec![tool_node]));

    // Build our application with a route
    let app = Router::new()
        .route("/execute", post(execute_flow))
        .with_state(flow);

    // Run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
