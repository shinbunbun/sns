use askama::Template;

use crate::usecase::message::Post;

#[derive(Template)]
#[template(path = "timeline.html")]
pub struct TimelineTemplate {
    pub user_name: String,
    pub posts: Vec<Post>,
}
