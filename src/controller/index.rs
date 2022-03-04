use actix_session::Session;
use sha3::{Digest, Sha3_256};

use crate::app_context::AppContext;
use crate::usecase;
use crate::{views, views::TemplateToResponse};
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use validator::Validate;

const SESSION_KEY: &str = "user";

pub async fn index(context: web::Data<AppContext>, session: Session) -> impl Responder {
    let user_session = match session.get::<String>(SESSION_KEY) {
        Ok(res) => res,
        Err(_) => return HttpResponse::InternalServerError().body("failed to get session"),
    };
    let user_id = match user_session {
        Some(x) => x,
        None => return views::index::IndexTemplate {}.to_response(),
    };

    let db = &context.db;
    let select_result = usecase::user::select_by_user_id(db, &user_id).await;
    if select_result.is_err() {
        return HttpResponse::BadRequest().body("user not found");
    };
    HttpResponse::Found()
        .insert_header(("Location", "timeline"))
        .finish()
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
