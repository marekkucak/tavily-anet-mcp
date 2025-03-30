use anet_mcp_server::{Content, Tool};
use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::{Value, json};
use tracing::{debug, error};

use crate::models::tavily::TavilyExtractResponse;
use crate::utils::formatter::format_tavily_extract_results;

// Tavily Extract Tool
pub struct TavilyExtractTool {
    api_key: String,
    client: Client,
}

impl TavilyExtractTool {
    pub fn new(api_key: String) -> Result<Self> {
        debug!(
            "Creating TavilyExtractTool with API key: {}",
            api_key.chars().take(5).collect::<String>() + "..."
        );

        let client = Client::new();

        Ok(Self { api_key, client })
    }

    async fn extract(&self, params: Value) -> Result<TavilyExtractResponse> {
        let extract_params = params.clone();

        debug!(
            "Extract parameters: {}",
            serde_json::to_string_pretty(&extract_params)?
        );

        let response = self
            .client
            .post("https://api.tavily.com/extract")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&extract_params)
            .send()
            .await?;

        debug!("Tavily API response status: {}", response.status());

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            error!("Tavily API error: {} - {}", status, error_text);
            return Err(anyhow::anyhow!("Tavily API error: {}", error_text));
        }

        // For debugging the exact response format
        let response_text = response.text().await?;
        debug!("Raw extract API response: {}", response_text);

        let extract_response = serde_json::from_str::<TavilyExtractResponse>(&response_text)
            .context("Failed to parse Tavily extract response")?;

        debug!("Successfully parsed Tavily API extract response");
        Ok(extract_response)
    }
}

#[async_trait]
impl Tool for TavilyExtractTool {
    fn name(&self) -> String {
        "tavily-extract".to_string()
    }

    fn description(&self) -> String {
        "A powerful web content extraction tool that retrieves and processes raw content from specified URLs, ideal for data collection, content analysis, and research tasks.".to_string()
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "urls": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "List of URLs to extract content from"
                },
                "extract_depth": {
                    "type": "string",
                    "enum": ["basic", "advanced"],
                    "description": "Depth of extraction - 'basic' or 'advanced', if urls are linkedin use 'advanced' or if explicitly told to use advanced",
                    "default": "basic"
                },
                "include_images": {
                    "type": "boolean",
                    "description": "Include a list of images extracted from the urls in the response",
                    "default": false
                }
            },
            "required": ["urls"]
        })
    }

    async fn call(&self, input: Option<Value>) -> Result<Vec<Content>> {
        let params = input.unwrap_or_else(|| json!({}));

        match self.extract(params).await {
            Ok(response) => {
                // Format the response with the extract-specific formatter
                let formatted = format_tavily_extract_results(&response);
                Ok(vec![Content::Text { text: formatted }])
            }
            Err(e) => {
                error!("Tavily extract error: {}", e);
                Err(e)
            }
        }
    }
}
