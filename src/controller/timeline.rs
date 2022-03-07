use crate::app_context::AppContext;
use crate::session;
use crate::usecase;
use crate::{views, views::TemplateToResponse};
use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use sea_orm::prelude::DateTimeWithTimeZone;

pub async fn timeline(context: web::Data<AppContext>, session: Session) -> impl Responder {
    let db = &context.db;

    let user = session::get_user(db, &session).await;
    let user = match user {
        Some(x) => x,
        None => return HttpResponse::InternalServerError().body("session error"),
    };

    let messages =
        usecase::message::select_all_posts_info(db, &String::from("01FX99PNZ5B0TTPC7R9F2CNQRG"))
            .await;
    let messages = match messages {
        Ok(res) => res,
        Err(e) => {
            println!("{:#?}", e);
            return HttpResponse::InternalServerError().body("db select error");
        }
    };

    match messages[0].try_get::<String>("", "message_id") {
        Ok(_) => (),
        Err(e) => println!("{:#?}", e),
    };

    let mut posts: Vec<views::timeline::Post> = Vec::new();

    for one_message in messages {
        let message_id = match one_message.try_get::<String>("", "message_id") {
            Ok(res) => res,
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
        };
        let user_name = match one_message.try_get::<String>("", "user_name") {
            Ok(res) => res,
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
        };
        let message_text = match one_message.try_get::<String>("", "message_text") {
            Ok(res) => res,
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
        };
        let created_at = match one_message.try_get::<DateTimeWithTimeZone>("", "created_at") {
            Ok(res) => res,
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
        };
        let likes = match one_message.try_get::<i64>("", "likes_count") {
            Ok(res) => res,
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
        };
        let is_like = match one_message.try_get::<i64>("", "is_like") {
            Ok(res) => res,
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
        };
        posts.push(views::timeline::Post {
            message_id,
            user_name,
            message_text,
            created_at,
            likes,
            is_like,
        });
    }
    views::timeline::TimelineTemplate {
        user_name: user.user_name,
        posts,
    }
    .to_response()
    /* views::timeline::TimelineTemplate {
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
    .to_response() */
}
