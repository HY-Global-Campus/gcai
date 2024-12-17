mod client;
mod mappers;
mod utils;

use crate::entities::{TextCompletionRequest, TextCompletionResponse};
use crate::use_cases::traits::TextCompletionService;
use async_trait::async_trait;
use azure_openai_api;

pub struct AzureOpenAIPluginParams {
    pub token: String,
    pub deployment: String,
    pub search_key: Option<String>,
    pub search_endpoint: Option<String>,
    pub http_client: reqwest::Client,
}

pub struct AzureOpenAIPlugin {
    // Fields for configuration, authentication, etc.
    token: String,
    deployment: String,
    search_key: Option<String>,
    search_endpoint: Option<String>,
    http_client: reqwest::Client,
}

impl AzureOpenAIPlugin {
    pub fn new(params: AzureOpenAIPluginParams) -> Self {
        Self {
            token: params.token,
            deployment: params.deployment,
            search_key: params.search_key,
            search_endpoint: params.search_endpoint,
            http_client: params.http_client,
        }
    }
}

// Implement the service traits
#[async_trait]
impl TextCompletionService for AzureOpenAIPlugin {
    async fn complete_text(&self, request: &TextCompletionRequest) -> TextCompletionResponse {
        let api_request = mappers::map_to_openai_request(request);
        let api_response = client::send_request(&api_request);
        mappers::map_to_text_completion_response(&api_response)
    }
}
