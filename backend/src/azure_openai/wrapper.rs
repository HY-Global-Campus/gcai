use crate::azure_openai::types;
use reqwest;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OpenAiError {
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Environment variable error: {0}")]
    EnvVar(#[from] std::env::VarError),
}

pub async fn send_request_to_openai(
    body: types::RequestBody,
) -> Result<types::ResponseBody, OpenAiError> {
    let client = reqwest::Client::new();

    let api_key = std::env::var("OPENAI_API_KEY")?;
    let api_base = std::env::var("OPENAI_API_BASE")?;

    let response = client
        .post(api_base)
        .header("api-key", &api_key)
        .json(&body)
        .send()
        .await?;

    response
        .json::<types::ResponseBody>()
        .await
        .map_err(OpenAiError::from)
}
