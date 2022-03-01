use crate::controller;
use actix_web::web;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(controller::index::index));
    cfg.route("/signup", web::get().to(controller::signup::signup));
}
