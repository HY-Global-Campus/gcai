use crate::entities::Message;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct TextCompletionRequest {
    // Either `prompt` or `messages` should be provided
    pub prompt: Option<String>,
    pub messages: Option<Vec<Message>>,

    // Common parameters
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub n: Option<u32>,
    pub stop_sequences: Option<Vec<String>>,
    pub stream: Option<bool>,
    pub presence_penalty: Option<f32>,
    pub frequency_penalty: Option<f32>,
    pub logprobs: Option<u32>,

    // Additional parameters for plugin-specific needs
    pub additional_parameters: Option<HashMap<String, serde_json::Value>>,
}

// Validate that either `prompt` or `messages` is provided, but not both
impl TextCompletionRequest {
    pub fn validate(&self) -> Result<(), String> {
        match (&self.prompt, &self.messages) {
            (Some(_), Some(_)) => {
                Err("Both `prompt` and `messages` cannot be provided at the same time.".to_string())
            }
            (None, None) => Err("Either `prompt` or `messages` must be provided.".to_string()),
            _ => Ok(()),
        }
    }
}
