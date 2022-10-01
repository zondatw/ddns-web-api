use actix_web::{web, Responder};

use super::serializers::{DDNSRequestSerializer, DDNSResponseSerializer};
use crate::core::constants::DNSState;

pub async fn handler_ddns_set(
    data: web::Data<DNSState>,
    req: web::Json<DDNSRequestSerializer>,
) -> impl Responder {
    let dns_key = &data.dns_key;
    println!("DDNS subdomain: {}, ip: {}", req.subdomain, req.ip);

    DDNSResponseSerializer {
        dns_key: dns_key.clone(),
    }
}
