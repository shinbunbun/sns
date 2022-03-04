use actix_session::Session;
use sha3::{Digest, Sha3_256};

use crate::app_context::AppContext;
use crate::session;
use crate::usecase;
use crate::views;
use crate::views::TemplateToResponse;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use validator::Validate;

pub async fn index(context: web::Data<AppContext>, session: Session) -> impl Responder {
    if session::is_valid(&context, &session).await {
        HttpResponse::Found()
            .insert_header(("Location", "timeline"))
            .finish()
    } else {
        views::index::IndexTemplate {}.to_response()
    }
}

pub async fn index_post(
    req: web::Form<Req>,
    context: web::Data<AppContext>,
    session: Session,
) -> impl Responder {
    if req.validate().is_err() {
        return HttpResponse::BadRequest().body("validate error");
    }
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

#[derive(Debug, Validate, Deserialize)]
pub struct Req {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    password: String,
}
