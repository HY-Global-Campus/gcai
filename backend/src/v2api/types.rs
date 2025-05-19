use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct ApiRequestBody {
    pub messages: Vec<Message>,
    pub azure_search_index_name: Option<String>,
    pub deployment: Option<String>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub frequency_penalty: Option<f32>,
    pub presence_penalty: Option<f32>,
    pub max_tokens: Option<u32>,
    pub max_completion_tokens: Option<u32>,
    pub stop: Option<String>,
    pub stream: Option<bool>,
    pub logit_bias: Option<serde_json::Value>,
    pub user: Option<String>,
    pub data_sources: Option<serde_json::Value>,
    pub logprobs: Option<bool>,
    pub top_logprobs: Option<u32>,
    pub n: Option<u32>,
    pub parallel_tool_calls: Option<bool>,
    pub seed: Option<u32>,
    pub tools: Option<Vec<String>>,
    pub tool_choice: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponseBody {
    pub id: String,
    pub prompt_filter_results: Option<serde_json::Value>,
    pub created: u64,
    pub choices: Vec<serde_json::Value>,
    pub model: String,
    pub system_fingerprint: Option<String>,
    pub object: String,
    pub usage: Option<serde_json::Value>,
}
