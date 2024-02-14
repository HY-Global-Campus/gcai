use crate::azure_openai::completions;
use crate::azure_openai::extensions;
use serde::{Deserialize, Serialize};

pub enum Url {
    CompletionUrl,
    ExtensionsUrl,
}

impl Url {
    pub fn to_string(&self, api_base: String, deployment: String) -> String {
        let api_base = api_base.trim_end_matches('/');
        match self {
            Url::CompletionUrl => {
                format!(
                    "{}/openai/deployments/{}/chat/completions?api-version=2023-07-01-preview",
                    api_base, deployment
                )
            }
            Url::ExtensionsUrl => {
                format!("{}/openai/deployments/{}/extensions/chat/completions?api-version=2023-07-01-preview", api_base, deployment)
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RequestBody {
    CompletionsRequestBody(completions::types::RequestBody),
    ExtensionsRequestBody(extensions::types::RequestBody),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseBody {
    CompletionsResponseBody(completions::types::ResponseBody),
    ExtensionsResponseBody(extensions::types::ResponseBody),
}

impl From<completions::types::ResponseBody> for ResponseBody {
    fn from(item: completions::types::ResponseBody) -> Self {
        ResponseBody::CompletionsResponseBody(item)
    }
}

// Implement `From` for the extensions response body
impl From<extensions::types::ResponseBody> for ResponseBody {
    fn from(item: extensions::types::ResponseBody) -> Self {
        ResponseBody::ExtensionsResponseBody(item)
    }
}
