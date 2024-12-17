use crate::entities::{TextCompletionRequest, TextCompletionResponse};
use crate::use_cases::traits::TextCompletionService;

pub struct TextCompletionInteractor<T: TextCompletionService> {
    service: T,
}

impl<T: TextCompletionService> TextCompletionInteractor<T> {
    pub fn new(service: T) -> Self {
        Self { service }
    }

    pub async fn handle(&self, request: &TextCompletionRequest) -> TextCompletionResponse {
        self.service.complete_text(request).await
    }
}
