mod models;
mod tools;
mod utils;

use anet_mcp_server::{ServerBuilder, ServerCapabilities, transport::nats::NatsTransport};
use anyhow::Result;
use dotenv::dotenv;
use serde_json::json;
use std::env;
use tracing::{debug, error, info, warn};
use tracing_subscriber::{EnvFilter, fmt::format::FmtSpan};

use crate::tools::extract::TavilyExtractTool;
use crate::tools::search::TavilySearchTool;

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file if it exists
    let dotenv_result = dotenv();
    match dotenv_result {
        Ok(_) => debug!("Loaded environment from .env file"),
        Err(e) => warn!("Could not load .env file: {}", e),
    }

    // Initialize logging with more details
    let filter = if let Ok(log_level) = env::var("RUST_LOG") {
        EnvFilter::new(log_level)
    } else {
        EnvFilter::new("debug")
    };

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    info!("Starting Tavily MCP server");

    // Get API key from environment variable with detailed error handling
    let api_key = match env::var("TAVILY_API_KEY") {
        Ok(key) => {
            if key.trim().is_empty() {
                error!("TAVILY_API_KEY is set but empty!");
                return Err(anyhow::anyhow!(
                    "TAVILY_API_KEY environment variable is empty"
                ));
            }

            if key == "your_api_key_here" {
                error!(
                    "TAVILY_API_KEY is set to the placeholder 'your_api_key_here'. Please replace with your actual API key."
                );
                return Err(anyhow::anyhow!(
                    "TAVILY_API_KEY is set to the example placeholder. Please use your actual API key."
                ));
            }

            info!(
                "Found TAVILY_API_KEY: {}...",
                key.chars().take(5).collect::<String>()
            );
            key
        }
        Err(e) => {
            error!("Failed to get TAVILY_API_KEY environment variable: {}", e);
            error!("Please set your API key in the .env file or environment");
            return Err(anyhow::anyhow!(
                "TAVILY_API_KEY environment variable is required"
            ));
        }
    };

    // Create NATS transport
    let nats_url = env::var("NATS_URL").unwrap_or_else(|_| "nats://localhost:4222".to_string());
    let subject = env::var("MCP_SUBJECT").unwrap_or_else(|_| "mcp.requests".to_string());

    info!("Connecting to NATS at {} on subject {}", nats_url, subject);
    let transport = NatsTransport::new(&nats_url, &subject).await?;
    info!("Successfully connected to NATS");

    // Initialize tools
    info!("Initializing Tavily Search tool...");
    let search_tool = TavilySearchTool::new(api_key.clone())?;
    info!("Initializing Tavily Extract tool...");
    let extract_tool = TavilyExtractTool::new(api_key)?;

    // Build and run server
    info!("Building MCP server...");
    let server = ServerBuilder::new()
        .transport(transport)
        .name("tavily-mcp")
        .version("0.1.0")
        .capabilities(ServerCapabilities {
            tools: Some(json!({})),
            prompts: Some(json!({})),
            resources: Some(json!({})),
            notification_options: None,
            experimental_capabilities: None,
        })
        .add_tool(search_tool)
        .add_tool(extract_tool)
        .build()?;

    info!("Server built, ready to run!");
    info!("Listening for requests on NATS subject: {}", subject);
    server.run().await
}
