
use crate::models::tavily::{TavilyExtractResponse, TavilyResponse};

// Helper function to format Tavily API search responses
pub fn format_tavily_results(response: &TavilyResponse) -> String {
    let mut output = Vec::new();

    // Include answer if available
    if let Some(answer) = &response.answer {
        output.push(format!("Answer: {}", answer));
        output.push("\nSources:".to_string());

        for result in &response.results {
            output.push(format!("- {}: {}", result.title, result.url));
        }

        output.push("".to_string());
    }

    // Format detailed search results
    output.push("Detailed Results:".to_string());

    for result in &response.results {
        output.push(format!("\nTitle: {}", result.title));
        output.push(format!("URL: {}", result.url));
        output.push(format!("Content: {}", result.content));

        if let Some(raw_content) = &result.raw_content {
            output.push(format!("Raw Content: {}", raw_content));
        }
    }

    output.join("\n")
}

pub fn format_tavily_extract_results(response: &TavilyExtractResponse) -> String {
    let mut output = Vec::new();

    output.push("Extracted Results:".to_string());

    for result in &response.results {
        output.push(format!("\nURL: {}", result.url));
        output.push(format!("Raw Content: {}", result.raw_content));

        if let Some(images) = &result.images {
            output.push(format!("Images: {}", images.join(", ")));
        }
    }

    if let Some(failed_results) = &response.failed_results {
        output.push("\nFailed Results:".to_string());

        for failed in failed_results {
            output.push(format!("\nURL: {}", failed.url));
            output.push(format!("Error: {}", failed.error));
        }
    }

    output.push(format!(
        "\nResponse Time: {} seconds",
        response.response_time
    ));

    output.join("\n")
}
