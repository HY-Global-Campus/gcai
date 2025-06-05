use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct MergeData {
    pub metadata_title: Option<String>,
    pub metadata_author: Option<String>,
    pub content: Option<String>,
}

#[derive(Serialize)]
pub struct MergeResponseData {
    pub merged_content: String,
}
