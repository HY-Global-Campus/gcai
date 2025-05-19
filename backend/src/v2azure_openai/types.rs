pub enum Url {
    CompletionUrl,
}

impl Url {
    pub fn to_string(&self, api_base: String, deployment: String) -> String {
        let api_base = api_base.trim_end_matches('/');
        match self {
            Url::CompletionUrl => {
                format!(
                    "{}/openai/deployments/{}/chat/completions?api-version=2024-10-21",
                    api_base, deployment
                )
            }
        }
    }
}
