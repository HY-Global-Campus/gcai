use actix_web::{HttpRequest, HttpResponse};

pub fn check_token(req: HttpRequest) -> Result<(), HttpResponse> {
    // Retrieve the Authorization header and handle the case where it's missing or invalid
    let auth_header = match req.headers().get("Authorization") {
        Some(header) => match header.to_str() {
            Ok(h) => h,
            Err(_) => {
                return Err(
                    HttpResponse::BadRequest().body("Authorization header is not a valid string")
                )
            }
        },
        None => return Err(HttpResponse::BadRequest().body("No Authorization header found")),
    };

    // Compare the Authorization header with the expected token
    let expected_auth = format!(
        "Bearer {}",
        std::env::var("AUTH_TOKEN").unwrap_or_else(|_| {
            eprintln!("No AUTH_TOKEN found in environment variables");
            "".to_string()
        })
    );
    if auth_header != expected_auth {
        return Err(HttpResponse::Unauthorized().finish());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::header;
    use actix_web::test;

    #[actix_web::test]
    async fn test_check_token() {
        let req = test::TestRequest::default()
            .insert_header((header::AUTHORIZATION, "Bearer token"))
            .to_http_request();
        assert_eq!(check_token(req).unwrap_err().status(), 401);

        let req = test::TestRequest::default().to_http_request();
        assert_eq!(check_token(req).unwrap_err().status(), 400);
    }
}
