# Tavily-Anet-MCP Server

A Rust implementation of the **Model Control Protocol (MCP)** server that provides Tavily search and content extraction capabilities via a standardized protocol.

This server integrates the powerful Tavily API with the Anet MCP framework, enabling AI agents to perform web searches and extract content from URLs. It is designed for developers building **AI agent systems**, **LLM-based tools**, or **research automation** that requires up-to-date web information.

---

## Features

- ✅ Tavily Search integration with comprehensive parameters  
- 📄 Tavily Extract for content retrieval from URLs  
- 🔄 NATS transport layer for message passing  
- 🛠️ JSON-RPC 2.0 compatible API  
- ⚡ Asynchronous request handling with Tokio  
- 🔍 Advanced search capabilities including domain filtering, time ranges, and topic selection

---

## Requirements

- **Rust** 1.70+  
- **NATS** server running locally or accessible via network
- **Tavily API Key** (get one from [Tavily's website](https://tavily.com))

---

## Installation

Clone the repository and build the server:

```bash
git clone https://github.com/yourusername/tavily-anet-mcp.git
cd tavily-anet-mcp
```

Add your Tavily API key to a `.env` file:

```
TAVILY_API_KEY=your_api_key_here
NATS_URL=nats://localhost:4222
MCP_SUBJECT=mcp.requests
```

---

## Getting Started

### Running the Server

```bash
# Start a NATS server in another terminal or ensure one is already running
# Example:
nats-server

# Run the Tavily MCP server
cargo run
```

### Testing the Server

You can test the server using the included test client:

```bash
cargo run --example test_client
```

This will send various requests to the server (initialize, listTools, search, extract) and print the responses.

---

## Available Tools

### 1. Tavily Search

A powerful web search tool that provides comprehensive, real-time results using Tavily's AI search engine.

**Parameters:**

- `query` (required): Search query string
- `search_depth`: "basic" or "advanced" (default: "basic")  
- `topic`: "general" or "news" (default: "general")
- `days`: Number of days back for results (for news topic)
- `time_range`: "day", "week", "month", "year"
- `max_results`: 5-20 (default: 10)
- `include_images`: Boolean
- `include_raw_content`: Boolean
- `include_domains`: Array of domains to include
- `exclude_domains`: Array of domains to exclude

**Example:**

```json
{
  "name": "tavily-search",
  "arguments": {
    "query": "Latest developments in AI",
    "max_results": 5,
    "topic": "news",
    "days": 7
  }
}
```

### 2. Tavily Extract

A tool for extracting raw content from web pages.

**Parameters:**

- `urls` (required): Array of URLs to extract content from
- `extract_depth`: "basic" or "advanced" (default: "basic")
- `include_images`: Boolean (default: false)

**Example:**

```json
{
  "name": "tavily-extract",
  "arguments": {
    "urls": ["https://www.rust-lang.org/"],
    "extract_depth": "advanced",
    "include_images": true
  }
}
```

---

## Architecture

The server follows a modular design:

- **tools** – Tavily Search and Extract implementations
- **models** – Tavily API response structures
- **utils** – Formatting and helper functions
- **transport** – NATS message transport layer

---

## Development

### Adding New Features

To extend the server with additional Tavily capabilities:

1. Define response structures in `src/models/tavily.rs`
2. Implement the tool in `src/tools/` following the Tool trait
3. Add formatting functions in `src/utils/formatter.rs`
4. Register the tool in `src/main.rs`

---

## Troubleshooting

- Ensure your Tavily API key is valid and correctly set in the environment variables
- Check that the NATS server is running and accessible
- Verify the request format matches the expected input schema for each tool

---

## License

MIT License

---

## Acknowledgements

This project is built on top of the [Anet MCP Server](https://github.com/yourusername/anet-mcp-server) framework and integrates with the [Tavily API](https://tavily.com).
