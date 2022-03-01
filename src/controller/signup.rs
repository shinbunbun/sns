use crate::{views, views::TemplateToResponse};
use actix_web::Responder;

pub async fn signup() -> impl Responder {
    views::signup::SignUpTemplate {}.to_response()
}
