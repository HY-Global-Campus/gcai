use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize)]
pub struct SkillRequest {
    pub values: Vec<Record>,
}

#[derive(Deserialize)]
pub struct Record {
    #[serde(rename = "recordId")]
    pub record_id: String,
    pub data: RecordData,
}

#[derive(Deserialize)]
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
    #[serde(rename = "recordId")]
    pub record_id: String,
    pub data: SkillResponseData,
}

#[derive(Serialize)]
pub struct SkillResponseData {
    pub merged_content: String,
}

#[derive(Debug)]
pub enum MergeError {
    MissingRecordId,
}

impl fmt::Display for MergeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MergeError::MissingRecordId => {
                write!(f, "Each record must have a non-empty `record_id`")
            }
        }
    }
}

impl std::error::Error for MergeError {}

impl actix_web::ResponseError for MergeError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            MergeError::MissingRecordId => actix_web::http::StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::BadRequest().body(self.to_string())
    }
}
