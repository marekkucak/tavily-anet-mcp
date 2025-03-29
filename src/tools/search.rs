use anyhow::Result;
use async_trait::async_trait;
use anet_mcp_server::{Content, Tool};
use reqwest::Client;
use serde_json::{json, Value};
use tracing::{error, debug};

use crate::models::tavily::TavilyResponse;
use crate::utils::formatter::format_tavily_results;

// Tavily Search Tool
pub struct TavilySearchTool {
    api_key: String,
    client: Client,
}

impl TavilySearchTool {
    pub fn new(api_key: String) -> Result<Self> {
        debug!("Creating TavilySearchTool with API key: {}", api_key.chars().take(5).collect::<String>() + "...");
        
        let client = Client::new();

        Ok(Self {
            api_key,
            client,
        })
    }

    async fn search(&self, params: Value) -> Result<TavilyResponse> {
        // Add API key to parameters in the JSON body
        let mut search_params = params.clone();
        search_params["api_key"] = json!(self.api_key);
        
        debug!("Search parameters: {}", serde_json::to_string_pretty(&search_params)?);
        
        // Add news topic if query contains "news"
        if let Some(query) = params.get("query").and_then(|q| q.as_str()) {
            if query.to_lowercase().contains("news") && !search_params.get("topic").is_some() {
                search_params["topic"] = json!("news");
            }
        }

        debug!("Sending request to Tavily API with API key in request body");
        
        let response = self.client
            .post("https://api.tavily.com/search")
            .json(&search_params)
            .send()
            .await?;

        debug!("Tavily API response status: {}", response.status());
        
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            error!("Tavily API error: {} - {}", status, error_text);
            return Err(anyhow::anyhow!("Tavily API error: {}", error_text));
        }

        let tavily_response = response.json::<TavilyResponse>().await?;
        debug!("Successfully parsed Tavily API response");
        Ok(tavily_response)
    }
}

#[async_trait]
impl Tool for TavilySearchTool {
    fn name(&self) -> String {
        "tavily-search".to_string()
    }

    fn description(&self) -> String {
        "A powerful web search tool that provides comprehensive, real-time results using Tavily's AI search engine. Returns relevant web content with customizable parameters for result count, content type, and domain filtering. Ideal for gathering current information, news, and detailed web content analysis.".to_string()
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "Search query"
                },
                "search_depth": {
                    "type": "string",
                    "enum": ["basic", "advanced"],
                    "description": "The depth of the search. It can be 'basic' or 'advanced'",
                    "default": "basic"
                },
                "topic": {
                    "type": "string",
                    "enum": ["general", "news"],
                    "description": "The category of the search. This will determine which of our agents will be used for the search",
                    "default": "general"
                },
                "days": {
                    "type": "number",
                    "description": "The number of days back from the current date to include in the search results. This specifies the time frame of data to be retrieved. Please note that this feature is only available when using the 'news' search topic",
                    "default": 3
                },
                "time_range": {
                    "type": "string",
                    "description": "The time range back from the current date to include in the search results. This feature is available for both 'general' and 'news' search topics",
                    "enum": ["day", "week", "month", "year", "d", "w", "m", "y"]
                },
                "max_results": {
                    "type": "number",
                    "description": "The maximum number of search results to return",
                    "default": 10,
                    "minimum": 5,
                    "maximum": 20
                },
                "include_images": {
                    "type": "boolean",
                    "description": "Include a list of query-related images in the response",
                    "default": false
                },
                "include_image_descriptions": {
                    "type": "boolean",
                    "description": "Include a list of query-related images and their descriptions in the response",
                    "default": false
                },
                "include_raw_content": {
                    "type": "boolean",
                    "description": "Include the cleaned and parsed HTML content of each search result",
                    "default": false
                },
                "include_domains": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "A list of domains to specifically include in the search results, if the user asks to search on specific sites set this to the domain of the site",
                    "default": []
                },
                "exclude_domains": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "List of domains to specifically exclude, if the user asks to exclude a domain set this to the domain of the site",
                    "default": []
                }
            },
            "required": ["query"]
        })
    }

    async fn call(&self, input: Option<Value>) -> Result<Vec<Content>> {
        let params = input.unwrap_or_else(|| json!({}));
        
        debug!("TavilySearchTool call with params: {}", serde_json::to_string_pretty(&params)?);
        
        match self.search(params).await {
            Ok(response) => {
                // Format the response similar to the JS implementation
                let formatted = format_tavily_results(&response);
                debug!("Successfully formatted Tavily search results");
                Ok(vec![Content::Text { text: formatted }])
            },
            Err(e) => {
                error!("Tavily search error: {}", e);
                Err(e)
            }
        }
    }
}
