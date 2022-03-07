use crate::{views, views::TemplateToResponse};
use actix_web::Responder;
use chrono::Local;

pub async fn timeline() -> impl Responder {
    views::timeline::TimelineTemplate {
        user_name: String::from("rust"),
        posts: vec![
            views::timeline::Post {
                message_id: String::from("1"),
                user_name: String::from("test"),
                message_text: String::from("content"),
                created_at: Local::now(),
                likes: 3,
                is_like: true,
            },
            views::timeline::Post {
                message_id: String::from("2"),
                user_name: String::from("test2"),
                message_text: String::from("content2"),
                created_at: Local::now(),
                likes: 0,
                is_like: false,
            },
        ],
    }
    .to_response()
}
