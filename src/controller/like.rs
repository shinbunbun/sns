use crate::usecase;
use crate::{app_context::AppContext, session};
use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

pub async fn like_post(
    req: web::Form<Req>,
    context: web::Data<AppContext>,
    session: Session,
) -> impl Responder {
    let user = session::get_user(&context.db, &session).await;
    let user = match user {
        Some(x) => x,
        None => return HttpResponse::InternalServerError().body("failed to get user info"),
    };

    let db = &context.db;
    match usecase::like::insert(db, &user.user_id, &req.message_id).await {
        Ok(_) => HttpResponse::SeeOther()
            .insert_header(("Location", "/timeline"))
            .finish(),
        Err(_) => HttpResponse::InternalServerError().body("db insert error"),
    }
}

pub async fn like_delete(
    req: web::Json<Req>,
    context: web::Data<AppContext>,
    session: Session,
) -> impl Responder {
    let user = session::get_user(&context.db, &session).await;
    let user = match user {
        Some(x) => x,
        None => return HttpResponse::InternalServerError().body("failed to get user info"),
    };
    let db = &context.db;
    match usecase::like::delete(db, &user.user_id, &req.message_id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("db delete error"),
    }
}

#[derive(Deserialize)]
pub struct Req {
    pub message_id: String,
}
