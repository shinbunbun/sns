use crate::{views, views::TemplateToResponse};
use actix_web::Responder;

pub async fn index() -> impl Responder {
    views::index::IndexTemplate {
        name: String::from("rust"),
    }
    .to_response()
}
