mod controller;
mod router;
mod usecase;
mod views;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(router::router))
        .bind(("localhost", 8000))?
        .run()
        .await
}
