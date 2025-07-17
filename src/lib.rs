//! # RustyFlow
//!
//! A lightweight, high-performance agent framework for Rust, providing elegant abstractions
//! for building complex AI workflows with type safety and async concurrency.
//!
//! RustyFlow is designed to model AI workflows as graphs with async execution, offering
//! sequential, parallel, and batch processing patterns for building robust agent systems.
//!
//! ## Quick Start
//!
//! ```rust
//! use async_trait::async_trait;
//! use rustyflow::{flow::Flow, node::Node, error::FlowError};
//! use serde_json::{json, Value};
//!
//! struct GreetingNode;
//!
//! #[async_trait]
//! impl Node for GreetingNode {
//!     async fn call(&self, input: Value) -> Result<Value, FlowError> {
//!         let name = input["name"].as_str().unwrap_or("World");
//!         Ok(json!({ "message": format!("Hello, {}!", name) }))
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), FlowError> {
//!     let flow = Flow::new(vec![Box::new(GreetingNode)]);
//!     let result = flow.execute(json!({"name": "Rust"})).await?;
//!     println!("{}", result); // {"message": "Hello, Rust!"}
//!     Ok(())
//! }
//! ```
//!
//! ## Core Components
//!
//! - [`Node`]: Basic computation unit with async execution
//! - [`Flow`]: Sequential orchestration of nodes
//! - [`ParallelFlow`]: Concurrent execution of multiple nodes
//! - [`Tool`]: Type-safe, structured computation with validation
//! - [`Batch`]: Concurrent processing of arrays
//!
//! ## Features
//!
//! - **Type Safety**: Compile-time guarantees for data flow
//! - **Async/Concurrent**: Full async/await support with parallel processing
//! - **Zero-Cost Abstractions**: High-level APIs with low-level performance
//! - **Flexible Execution**: Sequential, parallel, and batch patterns
//! - **Memory Safe**: Leverages Rust's ownership system

pub mod batch;
pub mod error;
pub mod flow;
pub mod node;
pub mod tool;

// Re-export commonly used types for convenience
pub use batch::Batch;
pub use error::FlowError;
pub use flow::{Flow, ParallelFlow};
pub use node::Node;
pub use tool::{Tool, ToolNode};
