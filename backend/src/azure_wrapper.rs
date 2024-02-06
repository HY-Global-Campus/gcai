use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OpenAiError {
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Environment variable error: {0}")]
    EnvVar(#[from] std::env::VarError),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody {
    pub messages: Vec<Message>,
    pub temperature: f32,
    pub top_p: f32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,
    pub max_tokens: u32,
    pub stop: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseBody {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub prompt_filter_results: Vec<PromptFilterResult>,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PromptFilterResult {
    prompt_index: u32,
    content_filter_results: HashMap<String, ContentFilter>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentFilter {
    filtered: bool,
    severity: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    finish_reason: String,
    index: u32,
    message: Message, // This is the same Message struct you defined earlier
    content_filter_results: HashMap<String, ContentFilter>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

pub async fn send_request_to_openai(body: RequestBody) -> Result<ResponseBody, OpenAiError> {
    let client = reqwest::Client::new();

    let api_key = std::env::var("OPENAI_API_KEY")?;
    let api_base = std::env::var("OPENAI_API_BASE")?;

    let response = client
        .post(api_base)
        .header("api-key", &api_key)
        .json(&body)
        .send()
        .await?;

    response
        .json::<ResponseBody>()
        .await
        .map_err(OpenAiError::from)
}
