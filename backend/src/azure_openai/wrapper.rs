use crate::azure_openai::completions;
use crate::azure_openai::extensions;
use crate::azure_openai::types;
use reqwest;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OpenAiError {
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Environment variable error: {0}")]
    EnvVar(#[from] std::env::VarError),
}

pub async fn send_request_to_openai(
    body: types::RequestBody,
) -> Result<types::ResponseBody, OpenAiError> {
    match body {
        types::RequestBody::CompletionsRequestBody(body) => send_completion_request_to_openai(body)
            .await
            .map(types::ResponseBody::CompletionsResponseBody),
        types::RequestBody::ExtensionsRequestBody(body) => send_extension_request_to_openai(body)
            .await
            .map(types::ResponseBody::ExtensionsResponseBody),
    }
}
pub async fn send_completion_request_to_openai(
    body: completions::types::RequestBody,
) -> Result<completions::types::ResponseBody, OpenAiError> {
    let client = reqwest::Client::new();

    let api_key = std::env::var("OPENAI_API_KEY")?;
    let api_base = std::env::var("OPENAI_API_BASE")?;
    let api_url = types::Url::CompletionUrl.to_string(api_base, "gpt-4o".to_string());
    println!("api_url {}", api_url);
    match serde_json::to_string(&body) {
        Ok(json_string) => println!("Request body as JSON: {}", json_string),
        Err(e) => println!("Error serializing body to JSON: {}", e),
    }

    let response = client
        .post(api_url)
        .header("api-key", &api_key)
        .json(&body)
        .send()
        .await?;

    let response_text = response.text().await?;

    match serde_json::from_str::<completions::types::ResponseBody>(&response_text) {
        Ok(parsed_response) => Ok(parsed_response),
        Err(e) => {
            eprintln!("Failed to deserialize response: {:?}", e);
            eprintln!("Raw response text: {}", response_text);
            Err(OpenAiError::from(e))
        }
    }
}

pub async fn send_extension_request_to_openai(
    body: extensions::types::RequestBody,
) -> Result<extensions::types::ResponseBody, OpenAiError> {
    let client = reqwest::Client::new();

    let api_key = std::env::var("OPENAI_API_KEY")?;
    let api_base = std::env::var("OPENAI_API_BASE")?;
    let api_url = types::Url::ExtensionsUrl.to_string(api_base, "gpt-4o".to_string());

    match serde_json::to_string(&body) {
        Ok(json_string) => println!("Request body as JSON: {}", json_string),
        Err(e) => println!("Error serializing body to JSON: {}", e),
    }

    let response = client
        .post(api_url)
        .header("api-key", &api_key)
        .json(&body)
        .send()
        .await?;

    let response_text = response.text().await.map_err(OpenAiError::from)?;

    match serde_json::from_str::<extensions::types::ResponseBody>(&response_text) {
        Ok(parsed_response) => Ok(parsed_response),
        Err(e) => {
            eprintln!("Failed to deserialize response: {:?}", e);
            eprintln!("Raw response text: {}", response_text);
            Err(OpenAiError::from(e))
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use std::collections::HashMap;
//
//     use super::*;
//     use mockito;
//
//     fn init(address: &str) {
//         std::env::set_var("OPENAI_API_KEY", "test-api-key");
//         std::env::set_var("OPENAI_API_BASE", address);
//     }
//
//     #[actix_web::test]
//     async fn test_send_completion_request_to_openai() {
//         let mut server = mockito::Server::new();
//         let address = format!("http://{}", server.host_with_port());
//         init(&address);
//         let completion_url = types::Url::CompletionUrl.to_string(address, "gpt-4o".to_string());
//         println!("Completion URL: {}", completion_url);
//
//         let mock = server
//             .mock(
//                 "POST",
//                 "/openai/deployments/gpt-4o/chat/completions?api-version=2024-02-01",
//             )
//             .match_header("api-key", "test-api-key")
//             .with_header("content-type", "application/json")
//             .with_body(
//                 r#"
//                 {
//                     "choices": [
//                         {
//                             "finish_reason": "length",
//                             "index": 1,
//                             "content_filter_results": null,
//                             "message": {
//                                 "role": "system",
//                                 "content": "Once upon a time"
//                             }
//                         }
//                     ],
//                     "id": "test-id",
//                     "object": "text_completion",
//                     "created": 1630000,
//                     "model": "gpt-4o",
//                     "prompt_filter_results": null,
//                     "usage": {
//                         "total_tokens": 10,
//                         "prompt_tokens": 3,
//                         "completion_tokens": 7
//                     }
//                 }
//             "#,
//             )
//             .create();
//
//         let messages = vec![completions::types::Message {
//             role: "system".to_string(),
//             content: "Once upon a time".to_string(),
//         }];
//         let req = completions::types::RequestBody {
//             max_tokens: 10,
//             temperature: 0.7,
//             top_p: 1.0,
//             frequency_penalty: 0.0,
//             presence_penalty: 0.0,
//             stop: None,
//             stream: false,
//             messages,
//         };
//         let response = send_completion_request_to_openai(req).await;
//         mock.assert();
//         let response = response.unwrap();
//         assert_eq!(response.choices.len(), 1);
//     }
//
//     #[actix_web::test]
//     async fn test_send_extension_request_to_openai() {
//         let mut server = mockito::Server::new();
//         let address = format!("http://{}", server.host_with_port());
//         init(&address);
//         let extension_url = types::Url::ExtensionsUrl.to_string(address, "gpt-4o".to_string());
//         println!("Extension URL: {}", extension_url);
//
//         let mock = server
//             .mock(
//                 "POST",
//                 "/openai/deployments/gpt-4o/extensions/chat/completions?api-version=2024-02-01",
//             )
//             .match_header("api-key", "test-api-key")
//             .with_header("content-type", "application/json")
//             .with_body(
//                 r#"
//                 {
//                     "choices": [
//                         {
//                             "finish_reason": "length",
//                             "index": 0,
//                             "messages": [
//                             {
//                                 "role": "system",
//                                 "content": "Once upon a time",
//                                 "index": 0,
//                                 "end_turn": true
//                             }
//                             ]
//                         }
//                     ],
//                     "id": "test-id",
//                     "object": "text_completion",
//                     "created": 1630000,
//                     "model": "gpt-4o",
//                     "usage": {
//                         "total_tokens": 10,
//                         "prompt_tokens": 3,
//                         "completion_tokens": 7
//                     }
//                 }
//             "#,
//             )
//             .create();
//
//         let messages = vec![extensions::types::Message {
//             role: "system".to_string(),
//             content: "Once upon a time".to_string(),
//         }];
//         let req = extensions::types::RequestBody {
//             max_tokens: 10,
//             temperature: 0.7,
//             top_p: 1.0,
//             frequency_penalty: 0.0,
//             presence_penalty: 0.0,
//             stop: None,
//             stream: false,
//             messages,
//             extensions: Some(extensions::types::Extensions {
//                 data_sources: vec![extensions::types::DataSource {
//                     data_type: "test".to_string(),
//                     parameters: extensions::types::DataSourceParameters {
//                         index_name: "test".to_string(),
//                         endpoint: "test".to_string(),
//                         semantic_configuration: "test".to_string(),
//                         query_type: "test".to_string(),
//                         fields_mapping: HashMap::new(),
//                         in_scope: true,
//                         role_information: "test".to_string(),
//                         filter: None,
//                         strictness: 0,
//                         top_n_documents: 5,
//                         key: "test".to_string(),
//                     },
//                 }],
//                 azure_search_index_name: "test".to_string(),
//                 azure_search_endpoint: "test".to_string(),
//                 azure_search_key: "test".to_string(),
//                 deployment: "test".to_string(),
//             }),
//         };
//
//         let response = send_extension_request_to_openai(req).await;
//         mock.assert();
//         let response = response.unwrap();
//         assert_eq!(response.choices.len(), 1);
//     }
// }
