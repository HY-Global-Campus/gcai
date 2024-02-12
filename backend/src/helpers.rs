use std::collections::HashMap;

use crate::api::types::ApiRequestBody;
use crate::azure_openai::types::{DataSource, RequestBody};

pub fn convert_api_request_to_request_body(api_request: ApiRequestBody) -> RequestBody {
    RequestBody {
        messages: api_request
            .messages
            .iter()
            .map(convert_message_from_api_to_azure_openai)
            .collect(),
        temperature: api_request.temperature.unwrap_or(0.0),
        top_p: api_request.top_p.unwrap_or(1.0),
        frequency_penalty: api_request.frequency_penalty.unwrap_or(0.0),
        presence_penalty: api_request.presence_penalty.unwrap_or(0.0),
        max_tokens: api_request.max_tokens.unwrap_or(100),
        stop: api_request.stop,
        stream: false,
        extensions: if api_request.use_own_data.unwrap_or(false) {
            Some(get_azure_search_extensions(
                api_request.azure_search_index_name.clone(),
            ))
        } else {
            None
        },
    }
}

fn get_azure_search_extensions(indexer: Option<String>) -> crate::azure_openai::types::Extensions {
    crate::azure_openai::types::Extensions {
        data_sources: vec![DataSource {
            data_type: "AzureCognitiveSearch".to_string(),
            parameters: get_azure_search_parameters(),
        }],
        azure_search_endpoint: "https://hy-ai-cognitive-search.search.windows.net".to_string(),
        azure_search_key: std::env::var("AZURE_SEARCH_KEY").unwrap(),
        azure_search_index_name: if indexer.is_some() {
            indexer.unwrap()
        } else {
            "mooc-5g-index".to_string()
        },
        deployment: "hy-gpt4-deploy".to_string(),
    }
}

fn get_azure_search_parameters() -> crate::azure_openai::types::DataSourceParameters {
    crate::azure_openai::types::DataSourceParameters {
        endpoint: "https://hy-ai-cognitive-search.search.windows.net".to_string(),
        index_name: "mooc-5g-index".to_string(),
        semantic_configuration: "default".to_string(),
        query_type: "simple".to_string(),
        fields_mapping: HashMap::new(),
        in_scope: true,
        role_information: "You are an AI assistant that helps people find information.".to_string(),
        filter: None,
        strictness: 3,
        top_n_documents: 5,
        key: std::env::var("AZURE_SEARCH_KEY").unwrap(),
    }
}

fn convert_message_from_api_to_azure_openai(
    api_message: &crate::api::types::Message,
) -> crate::azure_openai::types::Message {
    crate::azure_openai::types::Message {
        role: api_message.role.clone(),
        content: api_message.content.clone(),
    }
}
