use crate::skills::merge_pdf_metadata::types::{MergeData, MergeResponseData};
use crate::skills::types::{SkillError, SkillRequest, SkillResponse, SkillResponseRecord};
use actix_web::{web, HttpResponse, Responder};
use log::{info, warn};

pub async fn run(req: web::Json<SkillRequest<MergeData>>) -> Result<impl Responder, SkillError> {
    if req.values.is_empty() {
        warn!("skill::merge_pdf_metadata::Received an empty `values` array—returning an empty response.");
        return Ok(
            HttpResponse::Ok().json(SkillResponse::<MergeResponseData> { values: Vec::new() })
        );
    }

    let mut responses = Vec::with_capacity(req.values.len());

    for record in &req.values {
        if record.record_id.trim().is_empty() {
            return Err(SkillError::MissingRecordId);
        }

        let title: &str = record.data.metadata_title.as_deref().unwrap_or("").trim();
        let author: &str = record.data.metadata_author.as_deref().unwrap_or("").trim();
        let content: &str = record.data.content.as_deref().unwrap_or("");
        let prefix_len = "Title: ".len() + "\nAuthor: ".len() + "\n\n".len();
        let capacity = prefix_len + title.len() + author.len() + content.len();

        let mut merged_content = String::with_capacity(capacity);
        merged_content.push_str("Title: ");
        merged_content.push_str(title);
        merged_content.push('\n');
        merged_content.push_str("Author: ");
        merged_content.push_str(author);
        merged_content.push_str("\n\n");
        merged_content.push_str(content);

        responses.push(SkillResponseRecord {
            record_id: record.record_id.clone(),
            data: MergeResponseData { merged_content },
            warnings: None,
            errors: None,
        });
    }

    info!(
        "skill::merge_pdf_metadata::Successfully merged {} records",
        responses.len()
    );
    Ok(HttpResponse::Ok().json(SkillResponse { values: responses }))
}
