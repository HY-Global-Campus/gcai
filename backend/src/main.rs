use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use dotenv;
pub mod api;
pub mod auth;
pub mod azure_openai;
pub mod helpers;

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
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            eprintln!("Error sending request to OpenAI: {:?}", e);
            HttpResponse::InternalServerError().body("Error: Invalid request.")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    HttpServer::new(|| App::new().service(chat).service(index))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
