use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use dotenv;
pub mod auth;
pub mod azure_wrapper;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/api/chat")]
async fn chat(data: web::Json<azure_wrapper::RequestBody>, req: HttpRequest) -> impl Responder {
    if let Err(response) = auth::check_token(req) {
        return response;
    }

    let message = data.into_inner();
    match azure_wrapper::send_request_to_openai(message).await {
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
