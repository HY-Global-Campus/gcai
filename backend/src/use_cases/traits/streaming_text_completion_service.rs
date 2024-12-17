use crate::entities::TextCompletionRequest;
use crate::use_cases::traits::TextCompletionService;
use std::any::Any;

pub trait StreamingTextCompletionService: TextCompletionService + Any {
    fn stream_text(
        &self,
        request: &TextCompletionRequest,
    ) -> Box<dyn Iterator<Item = String> + Send>;
}
