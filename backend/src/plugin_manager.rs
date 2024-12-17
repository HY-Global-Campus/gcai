use crate::use_cases::traits::TextCompletionService;
use std::collections::HashMap;
use std::sync::Arc;

pub struct PluginManager {
    pub text_completion_plugins: HashMap<String, Arc<dyn TextCompletionService + Send + Sync>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            text_completion_plugins: HashMap::new(),
        }
    }

    pub fn register_text_completion_plugin(
        &mut self,
        name: &str,
        plugin: Arc<dyn TextCompletionService + Send + Sync>,
    ) {
        self.text_completion_plugins
            .insert(name.to_string(), plugin);
    }

    pub fn get_text_completion_plugin(
        &self,
        name: &str,
    ) -> Option<&Arc<dyn TextCompletionService + Send + Sync>> {
        self.text_completion_plugins.get(name)
    }
}
