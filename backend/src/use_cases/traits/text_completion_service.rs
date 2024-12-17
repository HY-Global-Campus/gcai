use crate::entities::{TextCompletionRequest, TextCompletionResponse};
use async_trait::async_trait;
use std::any::Any;

#[async_trait]
pub trait TextCompletionService: Any + Send + Sync {
    async fn complete_text(&self, request: &TextCompletionRequest) -> TextCompletionResponse;
}
