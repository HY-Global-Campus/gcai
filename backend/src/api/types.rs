use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiRequestBody {
    pub messages: Vec<Message>,
    pub azure_search_index_name: Option<String>,
    pub deployment: Option<String>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub frequency_penalty: Option<f32>,
    pub presence_penalty: Option<f32>,
    pub max_tokens: Option<u32>,
    pub stop: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseBody {
    pub id: String,
    pub created: u64,
    pub choices: Vec<Choice>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    finish_reason: String,
    index: u32,
    message: Message,
}
