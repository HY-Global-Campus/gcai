use crate::skills::find_author_title::types::{FindAuthorTitleData, FindAuthorTitleResponseData};
use crate::skills::types::{SkillError, SkillRequest, SkillResponse, SkillResponseRecord};
use crate::v2api::types::{ApiRequestBody, Message};
use crate::v2azure_openai::wrapper;
use actix_web::{web, HttpResponse, Responder};
use futures::future::join_all;
use log::{info, warn};

pub async fn run(
    req: web::Json<SkillRequest<FindAuthorTitleData>>,
) -> Result<impl Responder, SkillError> {
    if req.values.is_empty() {
        warn!("skill::find_title_author::Received an empty `values` array—returning an empty response.");
        return Ok(HttpResponse::Ok()
            .json(SkillResponse::<FindAuthorTitleResponseData> { values: Vec::new() }));
    }

    for record in &req.values {
        if record.record_id.trim().is_empty() {
            return Err(SkillError::MissingRecordId);
        }
    }

    let futures_vec = req
        .values
        .iter()
        .map(|record| {
            let record_id = record.record_id.clone();
            let content = record.data.content.clone().unwrap_or_default();

            async move {
                let author_title = ask_openai(content).await;
                Ok::<SkillResponseRecord<FindAuthorTitleResponseData>, SkillError>(
                    SkillResponseRecord {
                        record_id,
                        data: FindAuthorTitleResponseData {
                            author_title: format!("{}", author_title),
                        },
                    },
                )
            }
        })
        .collect::<Vec<_>>();

    let results: Vec<Result<SkillResponseRecord<FindAuthorTitleResponseData>, SkillError>> =
        join_all(futures_vec).await;

    let mut responses = Vec::with_capacity(results.len());
    for res in results.into_iter() {
        match res {
            Ok(record) => responses.push(record),
            Err(e) => {
                return Err(e);
            }
        }
    }

    info!(
        "skill::find_title_author::Successfully processed {} records",
        responses.len()
    );
    Ok(HttpResponse::Ok().json(SkillResponse { values: responses }))
}

async fn ask_openai(content: String) -> String {
    if content.is_empty() {
        return String::new();
    }
    let api_request_body = ApiRequestBody {
        messages: vec![
            Message {
                role: "system".to_string(),
                content: "You will get a contents of a article. Find the authors and title of the article.
                            Answer strictly in this format:
                            <authors> | <title>
                            If you cannot find any author or title, put uknown instead on its place.".to_string(),
            },
            Message {
            role: "user".to_string(),
            content,
        }],
        ..Default::default()
    };
    let response = wrapper::send_completion_request_to_openai(api_request_body).await;

    match response {
        Ok(api_response) => {
            if let Some(choice) = api_response.choices.first() {
                if let Some(message) = choice.get("message") {
                    if let Some(content) = message.get("content") {
                        if let Some(author_title) = content.as_str() {
                            return author_title.to_string();
                        }
                    }
                }
            }
            String::new()
        }
        Err(e) => {
            eprintln!("Error sending request to OpenAI: {:?}", e);
            "error in parsing title and author data".to_string()
        }
    }
}
