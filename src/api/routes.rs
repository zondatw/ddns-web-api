use actix_web::web;

use super::constants::DNSState;
use super::ddns;
use crate::config;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.app_data(web::Data::new(DNSState {
        dns_key: (*config::base::dns_key().lock().unwrap()).clone(),
        dns_server: (*config::base::dns_server().lock().unwrap()).clone(),
        dns_base_domain: (*config::base::dns_base_domain().lock().unwrap()).clone(),
    }))
    .service(web::scope("/api").configure(ddns::routes::config));
}
