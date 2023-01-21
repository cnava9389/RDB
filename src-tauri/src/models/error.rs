use actix_web::{HttpRequest, HttpResponse, http::header::ContentType};
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]

pub struct Error {
    message: String,
}

impl actix_web::Responder for Error {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::BadRequest()
            .content_type(ContentType::json())
            .body(body)
    }
}