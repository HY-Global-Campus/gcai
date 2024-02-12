use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub enum Url {
    CompletionUrl,
    ExtensionsUrl,
}

impl Url {
    pub fn to_string(&self, api_base: String, deplpyment: String) -> String {
        let api_base = api_base.trim_end_matches('/');
        match self {
            Url::CompletionUrl => {
                format!(
                    "{}/openai/deployments/{}/chat/completions?api-version=2023-07-01-preview",
                    api_base, deplpyment
                )
            }
            Url::ExtensionsUrl => {
                format!("{}/openai/deployments/{}/extensions/chat/completions?api-version=2023-07-01-preview", api_base, deplpyment)
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseMessage {
    pub role: String,
    pub content: String,
    index: u32,
    end_turn: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestBody {
    pub messages: Vec<Message>,
    pub temperature: f32,
    pub top_p: f32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,
    pub max_tokens: u32,
    pub stop: Option<String>,
    pub stream: bool,
    #[serde(flatten)]
    pub extensions: Option<Extensions>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Extensions {
    #[serde(rename = "dataSources")]
    pub data_sources: Vec<DataSource>,
    #[serde(rename = "azureSearchEndpoint")]
    pub azure_search_endpoint: String,
    #[serde(rename = "azureSearchKey")]
    pub azure_search_key: String,
    #[serde(rename = "azureSearchIndexName")]
    pub azure_search_index_name: String,
    pub deployment: String,
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
    #[serde(rename = "indexName")]
    pub index_name: String,
    #[serde(rename = "semanticConfiguration")]
    pub semantic_configuration: String,
    #[serde(rename = "queryType")]
    pub query_type: String,
    #[serde(rename = "fieldsMapping")]
    pub fields_mapping: HashMap<String, String>,
    #[serde(rename = "inScope")]
    pub in_scope: bool,
    #[serde(rename = "roleInformation")]
    pub role_information: String,
    pub filter: Option<String>,
    pub strictness: u8,
    #[serde(rename = "topNDocuments")]
    pub top_n_documents: u8,
    pub key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseBody {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub prompt_filter_results: Option<Vec<PromptFilterResult>>,
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
    pub finish_reason: Option<String>,
    pub index: u32,
    pub messages: Option<Vec<ResponseMessage>>,
    pub message: Option<Message>,
    pub content_filter_results: Option<HashMap<String, ContentFilter>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}
