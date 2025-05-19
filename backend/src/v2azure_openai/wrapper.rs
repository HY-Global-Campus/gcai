use crate::v2azure_openai::types;
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

pub async fn send_completion_request_to_openai(
    mut body: crate::v2api::types::ApiRequestBody,
) -> Result<crate::v2api::types::ApiResponseBody, OpenAiError> {
    inject_azure_search_key(&mut body)?;
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

    match serde_json::from_str::<crate::v2api::types::ApiResponseBody>(&response_text) {
        Ok(parsed_response) => Ok(parsed_response),
        Err(e) => {
            eprintln!("Failed to deserialize response: {:?}", e);
            eprintln!("Raw response text: {}", response_text);
            Err(OpenAiError::from(e))
        }
    }
}

fn inject_azure_search_key(
    body: &mut crate::v2api::types::ApiRequestBody,
) -> Result<(), OpenAiError> {
    let azure_search_key = std::env::var("AZURE_SEARCH_KEY")?;
    if let Some(data_sources) = &mut body.data_sources {
        // Try to interpret it as an array
        if let Some(data_sources_array) = data_sources.as_array_mut() {
            for source in data_sources_array {
                if let Some(parameters) = source.get_mut("parameters") {
                    if let Some(authentication) = parameters.get_mut("authentication") {
                        if let Some(auth_type) = authentication.get("type") {
                            if auth_type == "api_key" {
                                // Set the key value directly
                                authentication["key"] =
                                    serde_json::Value::String(azure_search_key.clone());
                                parameters["endpoint"] = serde_json::Value::String(
                                    "https://hy-ai-cognitive-search.search.windows.net".to_string(),
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    println!("body after azure key injection: {:?}", body);
    Ok(())
}
