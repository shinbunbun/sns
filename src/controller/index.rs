use actix_session::Session;
use sha3::{Digest, Sha3_256};

use crate::app_context::AppContext;
use crate::error::AppError;
use crate::session;
use crate::usecase;
use crate::views;
use crate::views::TemplateToResponse;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use validator::Validate;

pub async fn index(context: web::Data<AppContext>, session: Session) -> impl Responder {
    match session::get_user(&context.db, &session).await {
        Some(_) => HttpResponse::Found()
            .insert_header(("Location", "/timeline"))
            .finish(),
        None => views::index::IndexTemplate {}.to_response(),
    }
}

pub async fn index_post(
    req: web::Form<Req>,
    context: web::Data<AppContext>,
    session: Session,
) -> Result<impl Responder, AppError> {
    req.validate().map_err(AppError::Validation)?;

    let db = &context.db;
    let mut hasher = Sha3_256::new();
    hasher.update(&req.password);
    let password_hash = hex::encode(hasher.finalize());
    let select_result = usecase::user::select_by_e_mail(db, &req.email)
        .await
        .map_err(AppError::Db)?
        .ok_or_else(|| AppError::Internal("not found".to_string()))?;
    if select_result.password_hash != password_hash {
        return Err(AppError::Password("password is invalid".to_string()));
    }
    session::insert(&session, &select_result.user_id).map_err(AppError::Session)?;
    Ok(HttpResponse::SeeOther()
        .insert_header(("Location", "/timeline"))
        .finish())
}

#[derive(Debug, Validate, Deserialize)]
pub struct Req {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    password: String,
}
