use anyhow::{Context, Result};
use async_nats::{Client, ConnectOptions};
use futures_util::StreamExt;
use serde_json::{Value, json};

#[tokio::main]
async fn main() -> Result<()> {
    let client = async_nats::connect_with_options(
        "nats://localhost:4222",
        ConnectOptions::new().retry_on_initial_connect(),
    )
    .await
    .context("Failed to connect to NATS")?;

    println!("Connected to NATS server");

    async fn send_request(client: &Client, method: &str, params: Value, id: &str) -> Result<Value> {
        let request = json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params
        });

        let inbox = client.new_inbox();
        let mut sub = client.subscribe(inbox.clone()).await?;

        client
            .publish_with_reply(
                "mcp.requests".to_string(),
                inbox,
                serde_json::to_vec(&request)?.into(),
            )
            .await?;

        let msg = sub
            .next()
            .await
            .ok_or_else(|| anyhow::anyhow!("No response"))?;
        serde_json::from_slice(&msg.payload).context("Failed to parse response")
    }

    // Test 1: Initialize
    println!("Testing initialize...");
    let init_response = send_request(
        &client,
        "initialize",
        json!({"clientInfo": {"name": "tavily-test-client"}}),
        "1",
    )
    .await?;
    println!(
        "Initialize response: {}",
        serde_json::to_string_pretty(&init_response)?
    );

    // Test 2: List Tools
    println!("\nTesting listTools...");
    let tools_response = send_request(&client, "listTools", json!({}), "2").await?;
    println!(
        "ListTools response: {}",
        serde_json::to_string_pretty(&tools_response)?
    );

    // Test 3: Call Tavily Search Tool
    println!("\nTesting callTool with tavily-search...");
    let search_response = send_request(
        &client,
        "callTool",
        json!({
            "name": "tavily-search",
            "arguments": {
                "query": "Rust programming language",
                "max_results": 5
            }
        }),
        "3",
    )
    .await?;
    println!(
        "Tavily search response: {}",
        serde_json::to_string_pretty(&search_response)?
    );

    // Test 4: Call Tavily Extract Tool
    println!("\nTesting callTool with tavily-extract...");
    let extract_response = send_request(
        &client,
        "callTool",
        json!({
            "name": "tavily-extract",
            "arguments": {
                "urls": ["https://www.rust-lang.org/"]
            }
        }),
        "4",
    )
    .await?;
    println!(
        "Tavily extract response: {}",
        serde_json::to_string_pretty(&extract_response)?
    );

    Ok(())
}
