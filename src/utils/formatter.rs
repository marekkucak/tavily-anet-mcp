 use crate::models::tavily::{TavilyResponse, TavilyExtractResponse};

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

// Helper function to format Tavily API extract responses
pub fn format_tavily_extract_results(response: &TavilyExtractResponse) -> String {
    let mut output = Vec::new();
    
    output.push("Extracted Content:".to_string());
    output.push(format!("\n{}", response.content));
    
    if let Some(answer) = &response.answer {
        output.push(format!("\nAnswer: {}", answer));
    }
    
    if let Some(results) = &response.results {
        output.push("\nAdditional Results:".to_string());
        
        for result in results {
            output.push(format!("\nTitle: {}", result.title));
            output.push(format!("URL: {}", result.url));
            output.push(format!("Content: {}", result.content));
            
            if let Some(raw_content) = &result.raw_content {
                output.push(format!("Raw Content: {}", raw_content));
            }
        }
    }
    
    output.join("\n")
}
