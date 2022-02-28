use crate::views;
use actix_web::{HttpResponse, Responder};
use askama::Template;

pub async fn index() -> impl Responder {
    let html = views::index::IndexTemplate {
        name: String::from("rust"),
    };
    let response_body = html.render().expect("Create response body error");
    HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body)
}
