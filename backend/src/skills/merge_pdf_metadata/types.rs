use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SkillRequest {
    values: Vec<Record>,
}

#[derive(Deserialize)]
pub struct Record {
    record_id: Option<String>,
    data: Option<RecordData>,
}

#[derive(Deserialize)]
pub struct RecordData {
    metadata_title: Option<String>,
    metadata_author: Option<String>,
    content: Option<String>,
}

#[derive(Serialize)]
pub struct SkillResponse {
    values: Vec<SkillResponseRecord>,
}

#[derive(Serialize)]
pub struct SkillResponseRecord {
    record_id: String,
    data: SkillResponseData,
}

#[derive(Serialize)]
pub struct SkillResponseData {
    merged_content: String,
}
