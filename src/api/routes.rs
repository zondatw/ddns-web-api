
use actix_web::{web};

use super::ddns::handlers::handler_ddns_set;

pub fn ddns_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/ddns")
            .route(web::post().to(handler_ddns_set))
    );
}