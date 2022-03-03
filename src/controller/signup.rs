use sha3::{Digest, Sha3_256};

use crate::usecase;
use crate::{views, views::TemplateToResponse};
use actix_web::{web, HttpResponse, Responder};
use sea_orm::Database;
use serde::Deserialize;
use ulid::Ulid;

pub async fn signup() -> impl Responder {
    views::signup::SignUpTemplate {}.to_response()
}

pub async fn signup_post(req: web::Form<Req>) -> impl Responder {
    let db = Database::connect("mysql://user:password@db/local_db")
        .await
        .unwrap();
    let ulid = Ulid::new().to_string();
    let mut hasher = Sha3_256::new();
    hasher.update(&req.password);
    let password_hash = hex::encode(hasher.finalize());
    let insert_result = usecase::user::insert(&db, &req, &ulid, &password_hash).await;
    match insert_result {
        Ok(_) => HttpResponse::SeeOther()
            .insert_header(("Location", "timeline"))
            .finish(),
        Err(_) => HttpResponse::InternalServerError().body("database insert error"),
    }
}

#[derive(Deserialize)]
pub struct Req {
    pub email: String,
    password: String,
    pub name: String,
}
