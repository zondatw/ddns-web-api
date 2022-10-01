
use actix_web::{web};

use super::ddns;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(ddns::routes::config);
}