use askama::Template;

#[derive(Template)]
#[template(path = "signup.html")]
pub struct SignUpTemplate {}
