use sha3::{Digest, Sha3_256};

use crate::app_context::AppContext;
use crate::usecase;
use crate::{views, views::TemplateToResponse};
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use ulid::Ulid;
use validator::Validate;

pub async fn signup() -> impl Responder {
    views::signup::SignUpTemplate {}.to_response()
}

pub async fn signup_post(req: web::Form<Req>, context: web::Data<AppContext>) -> impl Responder {
    match req.validate() {
        Ok(_) => (),
        Err(_) => return HttpResponse::BadRequest().body("validate error"),
    }
    let db = &context.db;
    let ulid = Ulid::new().to_string();
    let mut hasher = Sha3_256::new();
    hasher.update(&req.password);
    let password_hash = hex::encode(hasher.finalize());
    let insert_result = usecase::user::insert(db, &req, &ulid, &password_hash).await;
    match insert_result {
        Ok(_) => HttpResponse::SeeOther()
            .insert_header(("Location", "timeline"))
            .finish(),
        Err(_) => HttpResponse::InternalServerError().body("database insert error"),
    }
}

#[derive(Debug, Validate, Deserialize)]
pub struct Req {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    password: String,

    pub name: String,
}
