use actix_web::web;

use crate::api;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(api::routes::config);
}