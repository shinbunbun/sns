use crate::app_context::AppContext;
use crate::session;
use crate::usecase;
use crate::{views, views::TemplateToResponse};
use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};

pub async fn timeline(context: web::Data<AppContext>, session: Session) -> impl Responder {
    let db = &context.db;

    let user = session::get_user(db, &session).await;
    let user = match user {
        Some(x) => x,
        None => return HttpResponse::InternalServerError().body("session error"),
    };

    let messages = usecase::message::select_all_messages_info(db, &user.user_id).await;
    let messages = match messages {
        Ok(res) => res,
        Err(e) => {
            println!("{:#?}", e);
            return HttpResponse::InternalServerError().body("db select error");
        }
    };

    views::timeline::TimelineTemplate {
        user_name: user.user_name,
        messages,
    }
    .to_response()
}
