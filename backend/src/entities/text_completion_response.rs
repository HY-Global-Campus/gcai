use crate::entities::Message;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Clone)]
pub struct TextCompletionResponse {
    pub text: Option<String>,
    pub choices: Option<Vec<CompletionChoice>>,
    pub usage: Option<UsageInfo>,
    pub finish_reason: Option<String>,

    // Additional data for plugin-specific responses
    pub additional_data: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Deserialize, Clone)]
pub struct CompletionChoice {
    pub messages: Vec<Message>,
    pub index: u32,
    pub logprobs: Option<serde_json::Value>, // Depending on API, this could be detailed
    pub finish_reason: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct UsageInfo {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}
