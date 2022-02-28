use actix_web::{HttpResponse, Responder};
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]

struct IndexTemplate<'a> {
    name: &'a str,
}

pub async fn index() -> impl Responder {
    let html = IndexTemplate { name: "rust" };
    let response_body = html.render().expect("Create response body error");
    HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body)
}
