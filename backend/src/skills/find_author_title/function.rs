use crate::skills::find_author_title::types::{FindAuthorTitleData, FindAuthorTitleResponseData};
use crate::skills::types::{
    SkillError, SkillRequest, SkillResponse, SkillResponseRecord, SkillResponseRecordInfoMessage,
};
use crate::v2api::types::{ApiRequestBody, Message};
use crate::v2azure_openai::wrapper;
use crate::v2azure_openai::wrapper::OpenAiError;
use actix_web::{web, HttpResponse, Responder};
use futures::future::join_all;
use log::{error, info, warn};
use std::time::Duration;
use tokio::time::sleep;

const MAX_RETRIES: usize = 3;

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
                let mut attempts = 0;

                let author_title_result: Result<String, OpenAiError> = loop {
                    match ask_openai(content.clone()).await {
                        Ok(title) => {
                            break Ok(title);
                        }
                        Err(err) if attempts < MAX_RETRIES => {
                            attempts += 1;
                            warn!(
                                "skill::find_title_author::Error on record {} (attempt {}/{}): {:?}. Retrying...",
                                record_id, attempts, MAX_RETRIES, err
                            );
                            info!("Async sleeping for 45s in a future to avoid rate limiting");
                            sleep(Duration::from_secs(45)).await;
                        }
                        Err(err) => {
                            break Err(err);
                        }
                    }
                };

                match author_title_result {
                    Err(e) => {
                        error!(
                            "skill::find_title_author::Failed to process record {} after {} attempts: {:?}",
                            record_id, MAX_RETRIES, e
                        );
                        Ok(SkillResponseRecord {
                            record_id: record_id.clone(),
                            data: FindAuthorTitleResponseData {
                                author_title: "error with openai".to_string(),
                            },
                            warnings: Some(vec![SkillResponseRecordInfoMessage{message: format!(
                                "Failed to process record {} after {} attempts: {:?}",
                                record_id, MAX_RETRIES, e
                            )}]),
                            errors: None
                        })
                    }
                    Ok(author_title) if author_title.is_empty() => {
                        warn!(
                            "skill::find_title_author::No author or title found for record {}",
                            record_id
                        );
                        Ok(SkillResponseRecord {
                            record_id: record_id.clone(),
                            data: FindAuthorTitleResponseData {
                                author_title: "unknown | unknown".to_string(),
                            },
                            warnings: Some(vec![SkillResponseRecordInfoMessage{ message: format!(
                                "No author or title found for record {}",
                                record_id
                            )}]),
                            errors: None,

                        })
                    }
                    Ok(author_title) => {
                        info!(
                            "skill::find_title_author::Successfully processed record {}",
                            record_id
                        );
                        Ok::<SkillResponseRecord<FindAuthorTitleResponseData>, SkillError>(
                            SkillResponseRecord {
                                record_id,
                                data: FindAuthorTitleResponseData {
                                    author_title,
                                },
                                warnings: None,
                                errors: None,
                            },
                        )
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    let results: Vec<Result<SkillResponseRecord<FindAuthorTitleResponseData>, SkillError>> =
        join_all(futures_vec).await;

    let mut responses = Vec::with_capacity(results.len());
    for res in results {
        match res {
            Ok(rec) => responses.push(rec),
            Err(err) => {
                return Err(err);
            }
        }
    }

    info!(
        "skill::find_title_author::Successfully processed {} records",
        responses.len()
    );
    Ok(HttpResponse::Ok().json(SkillResponse { values: responses }))
}

async fn ask_openai(content: String) -> Result<String, OpenAiError> {
    if content.is_empty() {
        return Ok(String::new());
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
            let author_title = api_response
                .choices
                .first()
                .and_then(|choice| choice.get("message"))
                .and_then(|message| message.get("content"))
                .and_then(|content| content.as_str())
                .map(|s| s.to_string())
                .unwrap_or_default();

            Ok(author_title)
        }
        Err(e) => {
            error!("Error sending request to OpenAI: {:?}", e);
            Err(e)
        }
    }
}
