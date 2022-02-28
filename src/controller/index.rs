use crate::{http_response::TemplateToResponse, views};
use actix_web::Responder;

pub async fn index() -> impl Responder {
    views::index::IndexTemplate {
        name: String::from("rust"),
    }
    .to_response()
}
