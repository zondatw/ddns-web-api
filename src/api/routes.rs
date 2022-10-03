use actix_web::web;

use super::ddns;
use crate::config;
use crate::core::constants::DNSState;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.app_data(web::Data::new(DNSState {
        dns_key: (*config::base::dns_key().lock().unwrap()).clone(),
    }))
    .service(web::scope("/api").configure(ddns::routes::config));
}
