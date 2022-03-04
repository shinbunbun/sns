mod app_context;
mod controller;
mod router;
mod usecase;
mod views;
use actix_session::CookieSession;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // AppContextインスタンス化
    let app_context = app_context::AppContext::new().await;

    HttpServer::new(move || {
        App::new()
            // アプリケーションにAppContextを注入
            .app_data(web::Data::new(app_context.clone()))
            .configure(router::router)
            .wrap(CookieSession::signed(&[0; 32]).http_only(true))
    })
    .bind(("localhost", 8000))?
    .run()
    .await
}
