use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

use crate::{app_context::AppContext, session};

pub async fn like_post(
    req: web::Form<Req>,
    context: web::Data<AppContext>,
    session: Session,
) -> impl Responder {
    let user = session::get_user(&context.db, &session);

    HttpResponse::Ok().body("ok")
}

#[derive(Deserialize)]
pub struct Req {
    pub message_id: String,
}
