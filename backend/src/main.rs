use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use dotenv;
pub mod api;
pub mod auth;
pub mod azure_openai;
pub mod helpers;
pub mod skills;
pub mod v2api;
pub mod v2azure_openai;
pub mod v3_tensorzero;
pub mod v3api;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/api/chat")]
async fn chat(data: web::Json<api::types::ApiRequestBody>, req: HttpRequest) -> impl Responder {
    if let Err(response) = auth::check_token(req) {
        return response;
    }

    let api_request_body = data.into_inner();
    let openai_request_body = helpers::convert_api_request_to_request_body(api_request_body);
    match azure_openai::wrapper::send_request_to_openai(openai_request_body).await {
        Ok(response) => {
            println!("Response: {:?}", response);
            let api_response = helpers::convert_response_body_to_api_response(response);
            HttpResponse::Ok().json(api_response)
        }
        Err(e) => {
            eprintln!("Error sending request to OpenAI: {:?}", e);
            HttpResponse::InternalServerError().body("Error: Invalid request.")
        }
    }
}

#[post("/api/v2/azure_openai/chat_completion")]
async fn v2_chat_completion(
    data: web::Json<v2api::types::ApiRequestBody>,
    req: HttpRequest,
) -> impl Responder {
    if let Err(response) = auth::check_token(req) {
        return response;
    }
    println!("v2_chat_completion data: {:?}", data);

    let api_request_body = data.into_inner();
    match v2azure_openai::wrapper::send_completion_request_to_openai(api_request_body).await {
        Ok(response) => {
            println!("Response: {:?}", response);
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            eprintln!("Error sending request to OpenAI: {:?}", e);
            HttpResponse::InternalServerError().body("Error: Invalid request.")
        }
    }
}

#[post("/api/v3/tensorzero/inference")]
async fn v3_tensorzero_inference(
    data: web::Json<v3api::types::InferenceRequest>,
    req: HttpRequest,
) -> impl Responder {
    if let Err(response) = auth::check_token(req) {
        return response;
    }
    let inference_request = data.into_inner();
    match v3_tensorzero::wrapper::inference_route(inference_request).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => {
            eprintln!("Error during inference: {:?}", e);
            HttpResponse::InternalServerError().body("Error: Invalid request.")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .service(chat)
            .service(index)
            .service(v2_chat_completion)
            .service(v3_tensorzero_inference)
            .route(
                "/skills/merge_pdf_metadata",
                web::post().to(skills::merge_pdf_metadata::function::run),
            )
            .route(
                "/skills/find_author_title",
                web::post().to(skills::find_author_title::function::run),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
