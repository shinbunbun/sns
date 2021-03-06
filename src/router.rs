use crate::controller;
use actix_web::web;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(controller::index::index));
    cfg.route("/", web::post().to(controller::index::index_post));
    cfg.route("/timeline", web::get().to(controller::timeline::timeline));
    cfg.route("/signup", web::get().to(controller::signup::signup));
    cfg.route("/signup", web::post().to(controller::signup::signup_post));
    cfg.route("/post", web::post().to(controller::message::message));
    cfg.route("/like", web::post().to(controller::like::like_post));
    cfg.route("/like", web::delete().to(controller::like::like_delete));
    cfg.route("/logout", web::post().to(controller::logout::logout_post));
}
