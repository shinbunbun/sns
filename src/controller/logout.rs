use actix_session::Session;
use actix_web::{HttpResponse, Responder};

use crate::session;

pub async fn logout_post(session: Session) -> impl Responder {
    session::remove(&session);
    HttpResponse::SeeOther()
        .insert_header(("Location", "/"))
        .finish()
}
