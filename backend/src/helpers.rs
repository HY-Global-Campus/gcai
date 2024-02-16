use std::collections::HashMap;

use crate::api::types::ApiRequestBody;
use crate::azure_openai;
use crate::azure_openai::completions;
use crate::azure_openai::extensions;

pub fn convert_api_request_to_request_body(
    api_request: ApiRequestBody,
) -> azure_openai::types::RequestBody {
    if api_request.azure_search_index_name.is_some() {
        azure_openai::types::RequestBody::ExtensionsRequestBody(
            convert_api_request_to_extensions_request_body(api_request),
        )
    } else {
        azure_openai::types::RequestBody::CompletionsRequestBody(
            convert_api_request_to_completions_request_body(api_request),
        )
    }
}

fn convert_api_request_to_completions_request_body(
    api_request: ApiRequestBody,
) -> completions::types::RequestBody {
    completions::types::RequestBody {
        messages: api_request
            .messages
            .iter()
            .map(convert_message_from_api_to_azure_openai_completion)
            .collect(),
        temperature: api_request.temperature.unwrap_or(0.7),
        top_p: api_request.top_p.unwrap_or(1.0),
        frequency_penalty: api_request.frequency_penalty.unwrap_or(0.0),
        presence_penalty: api_request.presence_penalty.unwrap_or(0.0),
        max_tokens: api_request.max_tokens.unwrap_or(1000),
        stop: api_request.stop,
        stream: false,
    }
}

fn convert_api_request_to_extensions_request_body(
    api_request: ApiRequestBody,
) -> extensions::types::RequestBody {
    extensions::types::RequestBody {
        messages: api_request
            .messages
            .iter()
            .map(convert_message_from_api_to_openai_extension)
            .collect(),
        temperature: api_request.temperature.unwrap_or(0.7),
        top_p: api_request.top_p.unwrap_or(1.0),
        frequency_penalty: api_request.frequency_penalty.unwrap_or(0.0),
        presence_penalty: api_request.presence_penalty.unwrap_or(0.0),
        max_tokens: api_request.max_tokens.unwrap_or(150),
        stop: api_request.stop,
        stream: false,
        extensions: Some(get_azure_search_extensions(
            api_request.azure_search_index_name,
        )),
    }
}

fn get_azure_search_extensions(indexe: Option<String>) -> extensions::types::Extensions {
    extensions::types::Extensions {
        data_sources: vec![extensions::types::DataSource {
            data_type: "AzureCognitiveSearch".to_string(),
            parameters: get_azure_search_parameters(),
        }],
        azure_search_endpoint: "https://hy-ai-cognitive-search.search.windows.net".to_string(),
        azure_search_key: std::env::var("AZURE_SEARCH_KEY").unwrap(),
        azure_search_index_name: if indexe.is_some() {
            indexe.unwrap()
        } else {
            "mooc-5g-index".to_string()
        },
        deployment: "hy-gpt4-deploy".to_string(),
    }
}

fn get_azure_search_parameters() -> extensions::types::DataSourceParameters {
    extensions::types::DataSourceParameters {
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

fn convert_message_from_api_to_azure_openai_completion(
    api_message: &crate::api::types::Message,
) -> completions::types::Message {
    completions::types::Message {
        role: api_message.role.clone(),
        content: api_message.content.clone(),
    }
}

fn convert_message_from_api_to_openai_extension(
    api_message: &crate::api::types::Message,
) -> extensions::types::Message {
    extensions::types::Message {
        role: api_message.role.clone(),
        content: api_message.content.clone(),
    }
}

pub fn convert_response_body_to_api_response(
    response_body: azure_openai::types::ResponseBody,
) -> crate::api::types::ApiResponseBody {
    match response_body {
        azure_openai::types::ResponseBody::CompletionsResponseBody(body) => {
            convert_completions_response_body_to_api_response(body)
        }
        azure_openai::types::ResponseBody::ExtensionsResponseBody(body) => {
            convert_extensions_response_body_to_api_response(body)
        }
    }
}

fn convert_completions_response_body_to_api_response(
    response_body: completions::types::ResponseBody,
) -> crate::api::types::ApiResponseBody {
    crate::api::types::ApiResponseBody {
        id: response_body.id,
        created: response_body.created,
        choices: response_body
            .choices
            .iter()
            .map(convert_choice_from_azure_openai_completion_to_api)
            .collect(),
    }
}

fn convert_extensions_response_body_to_api_response(
    response_body: extensions::types::ResponseBody,
) -> crate::api::types::ApiResponseBody {
    crate::api::types::ApiResponseBody {
        id: response_body.id,
        created: response_body.created,
        choices: response_body
            .choices
            .iter()
            .map(convert_choice_from_openai_extension_to_api)
            .collect(),
    }
}

fn convert_choice_from_azure_openai_completion_to_api(
    choice: &completions::types::Choice,
) -> crate::api::types::Choice {
    crate::api::types::Choice {
        finish_reason: choice.finish_reason.clone().unwrap_or("".to_string()),
        index: choice.index,
        messages: vec![convert_message_from_azure_openai_completion_to_api(
            &choice.message,
        )],
    }
}

fn convert_choice_from_openai_extension_to_api(
    choice: &extensions::types::Choice,
) -> crate::api::types::Choice {
    crate::api::types::Choice {
        finish_reason: choice.finish_reason.clone().unwrap_or("".to_string()),
        index: choice.index,
        messages: choice
            .messages
            .iter()
            .map(convert_message_from_openai_extension_to_api)
            .collect(),
    }
}

fn convert_message_from_azure_openai_completion_to_api(
    message: &completions::types::Message,
) -> crate::api::types::Message {
    crate::api::types::Message {
        role: message.role.clone(),
        content: message.content.clone(),
    }
}

fn convert_message_from_openai_extension_to_api(
    message: &extensions::types::ResponseMessage,
) -> crate::api::types::Message {
    crate::api::types::Message {
        role: message.role.clone(),
        content: message.content.clone(),
    }
}
