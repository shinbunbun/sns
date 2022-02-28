use actix_web::{http::header::ContentType, HttpResponse};

pub trait TemplateToResponse {
    fn to_response(&self) -> HttpResponse;
}

impl<T: askama::Template> TemplateToResponse for T {
    fn to_response(&self) -> HttpResponse {
        match self.render() {
            Ok(html_string) => HttpResponse::Ok()
                .content_type(ContentType::html())
                .body(html_string),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        }
    }
}
