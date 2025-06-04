use crate::skills::merge_pdf_metadata::types::*;
use actix_web::{web, HttpResponse, Responder};
use log::{info, warn};

pub async fn run(req: web::Json<SkillRequest>) -> Result<impl Responder, MergeError> {
    if req.values.is_empty() {
        warn!("Received an empty `values` array—returning an empty response.");
        return Ok(HttpResponse::Ok().json(SkillResponse { values: Vec::new() }));
    }

    for record in &req.values {
        if record.record_id.trim().is_empty() {
            return Err(MergeError::MissingRecordId);
        }
    }

    let responses: Vec<SkillResponseRecord> = req
        .values
        .iter()
        .map(|record| {
            let title: String = record
                .data
                .metadata_title
                .as_deref()
                .unwrap_or("") // &str
                .trim() // &str
                .to_string(); // String

            let author: String = record
                .data
                .metadata_author
                .as_deref()
                .unwrap_or("") // &str
                .trim() // &str
                .to_string(); // String

            let content: String = record
                .data
                .content
                .as_ref()
                .map(|s| s.clone())
                .unwrap_or_default();

            let prefix_len = "Title: ".len() + "\nAuthor: ".len() + "\n\n".len();
            let capacity = prefix_len + title.len() + author.len() + content.len();
            let mut merged_content = String::with_capacity(capacity);
            merged_content.push_str("Title: ");
            merged_content.push_str(&title);
            merged_content.push('\n');
            merged_content.push_str("Author: ");
            merged_content.push_str(&author);
            merged_content.push_str("\n\n");
            merged_content.push_str(&content);

            SkillResponseRecord {
                record_id: record.record_id.clone(),
                data: SkillResponseData { merged_content },
            }
        })
        .collect();

    info!("Successfully merged {} records", responses.len());
    Ok(HttpResponse::Ok().json(SkillResponse { values: responses }))
}
