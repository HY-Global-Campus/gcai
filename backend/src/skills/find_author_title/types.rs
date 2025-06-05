use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct FindAuthorTitleData {
    pub content: Option<String>,
}

#[derive(Serialize)]
pub struct FindAuthorTitleResponseData {
    pub author_title: String,
}
