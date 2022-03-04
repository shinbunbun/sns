use actix_session::Session;
use sha3::{Digest, Sha3_256};

use crate::app_context::AppContext;
use crate::usecase;
use crate::{views, views::TemplateToResponse};
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

pub async fn index() -> impl Responder {
    views::index::IndexTemplate {}.to_response()
}

pub async fn index_post(
    req: web::Form<Req>,
    context: web::Data<AppContext>,
    session: Session,
) -> impl Responder {
    let db = &context.db;
    let mut hasher = Sha3_256::new();
    hasher.update(&req.password);
    let password_hash = hex::encode(hasher.finalize());
    let select_result = usecase::user::select_by_e_mail(db, &req.email).await;
    let select_result = match select_result {
        Ok(res) => res,
        Err(_) => return HttpResponse::InternalServerError().body("database select error"),
    };
    let user = match select_result {
        Some(x) => x,
        None => return HttpResponse::InternalServerError().body("email or password is invalid"),
    };
    if user.password_hash != password_hash {
        return HttpResponse::InternalServerError().body("email or password is invalid");
    }
    match session.insert("user", user.user_id) {
        Ok(_) => (),
        Err(_) => return HttpResponse::InternalServerError().body("session insert failed"),
    };
    HttpResponse::SeeOther()
        .insert_header(("Location", "timeline"))
        .finish()
}

#[derive(Deserialize)]
pub struct Req {
    pub email: String,
    password: String,
}
