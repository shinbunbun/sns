use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

use crate::app_context::AppContext;

pub async fn post(
    req: web::Form<Req>,
    context: web::Data<AppContext>,
    session: Session,
) -> impl Responder {
    HttpResponse::SeeOther()
        .insert_header(("Location", "timeline"))
        .finish()
}

#[derive(Deserialize)]
pub struct Req {
    pub message_contnt: String,
}
