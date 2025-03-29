use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TavilyResult {
    pub title: String,
    pub url: String,
    pub content: String,
    pub score: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_content: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum TavilyImage {
    String(String),
    Object { url: String, description: Option<String> },
}

#[derive(Debug, Deserialize)]
pub struct TavilyResponse {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow_up_questions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<TavilyImage>>,
    pub results: Vec<TavilyResult>,
}

#[derive(Debug, Deserialize)]
pub struct TavilyExtractResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow_up_questions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<TavilyImage>>,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<TavilyResult>>,
}
