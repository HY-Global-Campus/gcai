use crate::v3api::types::InferenceRequest;
use log::{error, info};
use reqwest;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InferenceError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Missing gateway URL")]
    MissingUrl,
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

impl actix_web::ResponseError for InferenceError {}

pub async fn inference_route(
    payload: InferenceRequest,
) -> Result<crate::v3api::types::InferenceResponse, InferenceError> {
    let client = reqwest::Client::new();
    let gateway =
        std::env::var("TENSORZERO_GATEWAY_URL").map_err(|_| InferenceError::MissingUrl)?;
    let url = format!("{}/inference", gateway);

    info!("POST {}", url);
    let response = client.post(&url).json(&payload).send().await?;

    let response_text = response.text().await?;

    match serde_json::from_str::<crate::v3api::types::InferenceResponse>(&response_text) {
        Ok(parsed_response) => {
            info!("Inference response: {:?}", parsed_response);
            Ok(parsed_response)
        }
        Err(e) => {
            error!("Failed to deserialize response: {:?}", e);
            error!("Raw response text: {}", response_text);
            Err(InferenceError::from(e))
        }
    }
}
