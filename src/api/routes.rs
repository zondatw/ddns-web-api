
use actix_web::{web};

use super::ddns;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api").configure(
            ddns::routes::config
        )
    );
}