use actix_web::web;

use super::handlers::handler_ddns_set;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/ddns").route(web::post().to(handler_ddns_set)));
}
