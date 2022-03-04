use crate::controller;
use actix_web::web;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(controller::index::index));
    cfg.route("/", web::post().to(controller::index::index_post));
    cfg.route("/signup", web::get().to(controller::signup::signup));
    cfg.route("/signup", web::post().to(controller::signup::signup_post));
}
