use crate::entities::{
    CompletionChoice, Message, TextCompletionRequest, TextCompletionResponse, UsageInfo,
};
use azure_openai_api::models::chat_completions_options::ChatCompletionsOptions;
use serde_json::Value;

pub fn map_to_openai_request(request: &TextCompletionRequest) -> Value {
    // Map the generic request to OpenAI's specific request format
    unimplemented!();
}

pub fn map_to_text_completion_response(api_response: &Value) -> TextCompletionResponse {
    // Map OpenAI's response to the generic TextCompletionResponse
    unimplemented!();
}
