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
    let client = reqwest::Client::new();

    let api_key = std::env::var("OPENAI_API_KEY")?;
    let api_base = std::env::var("OPENAI_API_BASE")?;
    let api_url = if body.extensions.is_some() {
        types::Url::ExtensionsUrl.to_string(
            api_base,
            body.extensions.as_ref().unwrap().deployment.clone(),
        )
    } else {
        types::Url::CompletionUrl.to_string(
            api_base,
            body.extensions.as_ref().unwrap().deployment.clone(),
        )
    };

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

    match serde_json::from_str::<types::ResponseBody>(&response_text) {
        Ok(parsed_response) => Ok(parsed_response),
        Err(e) => {
            eprintln!("Failed to deserialize response: {:?}", e);
            eprintln!("Raw response text: {}", response_text);
            Err(OpenAiError::from(e))
        }
    }
}
