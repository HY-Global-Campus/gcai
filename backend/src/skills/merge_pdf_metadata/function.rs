use crate::skills::merge_pdf_metadata::types::*;
use actix_web::{web, HttpResponse, Responder};

pub async fn run(req: web::Json<SkillRequest>) -> impl Responder {
    let mut results = Vec::new();

    for record in &req.values {
        let title = record.data.metadata_title.clone().unwrap_or_default();
        let author = record.data.metadata_author.clone().unwrap_or_default();
        let content = record.data.content.clone().unwrap_or_default();

        let merged = format!("Title: {}\nAuthor: {}\n\n{}", title, author, content);

        results.push(SkillResponseRecord {
            record_id: record.record_id.clone(),
            data: SkillResponseData {
                merged_content: merged,
            },
        });
    }

    HttpResponse::Ok().json(SkillResponse { values: results })
}
