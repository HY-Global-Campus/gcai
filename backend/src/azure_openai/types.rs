use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub deployment: String,
    pub stream: bool,
    pub data_sources: Vec<DataSource>,
    pub azure_search_endpoint: String,
    pub azure_search_key: String,
    pub azure_search_index_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataSource {
    #[serde(rename = "type")]
    pub data_type: String,
    pub parameters: DataSourceParameters,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataSourceParameters {
    pub endpoint: String,
    pub index_name: String,
    pub semantic_configuration: String,
    pub query_type: String,
    pub fields_mapping: HashMap<String, String>,
    pub in_scope: bool,
    pub role_information: String,
    pub filter: Option<String>,
    pub strictness: u8,
    pub top_n_documents: u8,
    pub key: String,
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
    pub prompt_index: u32,
    pub content_filter_results: HashMap<String, ContentFilter>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentFilter {
    pub filtered: bool,
    pub severity: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    pub finish_reason: String,
    pub index: u32,
    pub message: Message,
    pub content_filter_results: HashMap<String, ContentFilter>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}
