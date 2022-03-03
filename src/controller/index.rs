use sha3::{Digest, Sha3_256};

use crate::usecase;
use crate::{views, views::TemplateToResponse};
use actix_web::{web, HttpResponse, Responder};
use sea_orm::Database;
use serde::Deserialize;
use ulid::Ulid;

pub async fn index() -> impl Responder {
    views::index::IndexTemplate {}.to_response()
}

pub async fn index_post(req: web::Form<Req>) -> impl Responder {
    let db = Database::connect("mysql://user:password@db/local_db")
        .await
        .unwrap();
    let mut hasher = Sha3_256::new();
    hasher.update(&req.password);
    let password_hash = hex::encode(hasher.finalize());
    let select_result = usecase::user::select_by_e_mail(&db, &req.email).await;
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
    HttpResponse::SeeOther()
        .insert_header(("Location", "timeline"))
        .finish()
}

pub struct Req {
    pub email: String,
    password: String,
}
