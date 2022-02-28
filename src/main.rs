use actix_web::{App, HttpServer};
use rust_sns::router;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(router::router))
        .bind(("localhost", 8000))?
        .run()
        .await
}
