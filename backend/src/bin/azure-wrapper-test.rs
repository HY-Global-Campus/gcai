use dotenv;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody {
    messages: Vec<Message>,
    temperature: f32,
    top_p: f32,
    frequency_penalty: f32,
    presence_penalty: f32,
    max_tokens: u32,
    stop: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseBody {
    id: String,
    object: String,
    created: u64,
    model: String,
    prompt_filter_results: Vec<PromptFilterResult>,
    choices: Vec<Choice>,
    usage: Usage,
}

#[derive(Serialize, Deserialize, Debug)]
struct PromptFilterResult {
    prompt_index: u32,
    content_filter_results: HashMap<String, ContentFilter>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ContentFilter {
    filtered: bool,
    severity: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Choice {
    finish_reason: String,
    index: u32,
    message: Message, // This is the same Message struct you defined earlier
    content_filter_results: HashMap<String, ContentFilter>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

pub async fn send_request_to_openai(
    api_key: String,
    body: RequestBody,
) -> Result<ResponseBody, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client
        .post(std::env::var("OPENAI_API_BASE").unwrap())
        .header("api-key", format!("{}", api_key))
        .json(&body)
        .send()
        .await?;
    response.json::<ResponseBody>().await
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    // Example usage
    let api_key = std::env::var("OPENAI_API_KEY").unwrap();
    let request_body = RequestBody {
        messages: vec![
            Message {
                role: "system".to_string(),
                content: "You are an AI assistant that helps people find information.".to_string(),
            },
            Message {
                role: "user".to_string(),
                content: "Hello, are you receiving my message? I sent it from rust app!"
                    .to_string(),
            },
        ],
        temperature: 0.7,
        top_p: 0.95,
        frequency_penalty: 0.0,
        presence_penalty: 0.0,
        max_tokens: 800,
        stop: None,
    };

    match send_request_to_openai(api_key, request_body).await {
        Ok(response) => println!("Response: {:?}", response),
        Err(e) => eprintln!("Error: {}", e),
    }
}
