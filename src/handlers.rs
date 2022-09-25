use actix_web::{post, web, Responder};

use crate::constants::DNSState;
use crate::serializers::{DDNSRequestSerializer, DDNSResponseSerializer};

#[post("/ddns")]
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
