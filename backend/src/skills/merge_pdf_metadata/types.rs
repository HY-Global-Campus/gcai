use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SkillRequest {
    pub values: Vec<Record>,
}

#[derive(Deserialize, Serialize)]
pub struct Record {
    pub record_id: String,
    pub data: RecordData,
}

#[derive(Deserialize, Serialize)]
pub struct RecordData {
    pub metadata_title: Option<String>,
    pub metadata_author: Option<String>,
    pub content: Option<String>,
}

#[derive(Serialize)]
pub struct SkillResponse {
    pub values: Vec<SkillResponseRecord>,
}

#[derive(Serialize)]
pub struct SkillResponseRecord {
    pub record_id: String,
    pub data: SkillResponseData,
}

#[derive(Serialize)]
pub struct SkillResponseData {
    pub merged_content: String,
}
