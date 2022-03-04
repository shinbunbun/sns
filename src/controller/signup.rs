use actix_session::Session;
use sha3::{Digest, Sha3_256};

use crate::app_context::AppContext;
use crate::session;
use crate::usecase;
use crate::{views, views::TemplateToResponse};
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use validator::Validate;

pub async fn signup(context: web::Data<AppContext>, session: Session) -> impl Responder {
    match session::get_user(&context.db, &session).await {
        Some(_) => HttpResponse::Found()
            .insert_header(("Location", "timeline"))
            .finish(),
        None => views::signup::SignUpTemplate {}.to_response(),
    }
}

pub async fn signup_post(
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
    let insert_result = usecase::user::insert(db, &req.email, &password_hash, &req.name).await;
    let insert_result = match insert_result {
        Ok(res) => res,
        Err(_) => return HttpResponse::InternalServerError().body("database insert error"),
    };
    match session.insert("user", insert_result.last_insert_id) {
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
    #[validate(length(min = 3))]
    pub name: String,
}
