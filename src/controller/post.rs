use crate::usecase;
use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

use crate::app_context::AppContext;

pub async fn post(
    req: web::Form<Req>,
    context: web::Data<AppContext>,
    session: Session,
) -> impl Responder {
    let user_session = match session.get::<String>("user") {
        Ok(res) => res,
        Err(_) => return HttpResponse::InternalServerError().body("failed to get session"),
    };
    let user_id = match user_session {
        Some(x) => x,
        None => return HttpResponse::InternalServerError().body("failed to get user session"),
    };
    let db = &context.db;
    let insert_result = usecase::message::insert(db, &user_id, &req.message_text).await;
    if insert_result.is_err() {
        return HttpResponse::InternalServerError().body("db insert error");
    }

    HttpResponse::SeeOther()
        .insert_header(("Location", "timeline"))
        .finish()
}

#[derive(Deserialize)]
pub struct Req {
    pub message_text: String,
}
