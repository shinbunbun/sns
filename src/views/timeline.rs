use askama::Template;
use chrono::{DateTime, Local};

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
    pub created_at: DateTime<Local>,
    pub likes: u64,
    pub is_like: bool,
}
