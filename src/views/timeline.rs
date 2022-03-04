use askama::Template;
use sea_orm::prelude::DateTimeWithTimeZone;

#[derive(Template)]
#[template(path = "timeline.html")]
pub struct TimelineTemplate {
    pub user_name: String,
    pub posts: Vec<Post>,
}

pub struct Post {
    pub message_id: String,
    pub user_name: String,
    pub message_text: String,
    pub created_at: DateTimeWithTimeZone,
    pub likes: i64,
    pub is_like: i64,
}
