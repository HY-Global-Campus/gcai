use crate::entities::{TextCompletionRequest, TextCompletionResponse};
use crate::use_cases::interactors::TextCompletionInteractor;
use crate::use_cases::traits::TextCompletionService;

pub struct TextCompletionController<T: TextCompletionService> {
    interactor: TextCompletionInteractor<T>,
}

impl<T: TextCompletionService> TextCompletionController<T> {
    pub fn new(interactor: TextCompletionInteractor<T>) -> Self {
        Self { interactor }
    }

    pub async fn handle_request(&self, input: TextCompletionRequest) -> TextCompletionResponse {
        self.interactor.handle(&input).await
    }
}
