use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize)]
pub struct SkillRequest<T> {
    pub values: Vec<SkillRecord<T>>,
}

#[derive(Deserialize)]
pub struct SkillRecord<T> {
    #[serde(rename = "recordId")]
    pub record_id: String,
    pub data: T,
}

#[derive(Serialize)]
pub struct SkillResponse<T> {
    pub values: Vec<SkillResponseRecord<T>>,
}

#[derive(Serialize)]
pub struct SkillResponseRecord<T> {
    #[serde(rename = "recordId")]
    pub record_id: String,
    pub data: T,
    pub errors: Option<Vec<SkillResponseRecordInfoMessage>>,
    pub warnings: Option<Vec<SkillResponseRecordInfoMessage>>,
}

#[derive(Serialize)]
pub struct SkillResponseRecordInfoMessage {
    pub message: String,
}

#[derive(Debug)]
pub enum SkillError {
    MissingRecordId,
}

impl fmt::Display for SkillError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SkillError::MissingRecordId => {
                write!(f, "Each record must have a non-empty `record_id`")
            }
        }
    }
}

impl std::error::Error for SkillError {}

impl actix_web::ResponseError for SkillError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            SkillError::MissingRecordId => actix_web::http::StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::BadRequest().body(self.to_string())
    }
}
