use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseMessage {
    pub role: String,
    pub content: String,
    pub index: u32,
    pub end_turn: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
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
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    pub finish_reason: Option<String>,
    pub index: u32,
    pub messages: Vec<ResponseMessage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}
