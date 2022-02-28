use crate::controller::index;
use actix_web::web;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index::index));
}
