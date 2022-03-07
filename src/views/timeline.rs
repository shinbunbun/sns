use askama::Template;

use crate::usecase::message::Message;

#[derive(Template)]
#[template(path = "timeline.html")]
pub struct TimelineTemplate {
    pub user_name: String,
    pub messages: Vec<Message>,
}
