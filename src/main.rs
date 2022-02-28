use actix_web::{App, HttpServer};
mod controller;
mod router;
mod views;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(router::router))
        .bind(("localhost", 8000))?
        .run()
        .await
}
