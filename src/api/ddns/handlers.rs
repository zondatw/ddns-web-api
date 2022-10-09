use actix_web::{web, Responder};
use std::process::Command;

use super::serializers::{DDNSRequestSerializer, DDNSResponseSerializer};
use crate::api::constants::DNSState;

pub async fn handler_ddns_set(
    data: web::Data<DNSState>,
    req: web::Json<DDNSRequestSerializer>,
) -> impl Responder {
    let dns_key = &data.dns_key;
    println!("DDNS subdomain: {}, ip: {}, dns_key: {}", req.subdomain, req.ip, dns_key);
    let output = Command::new("echo")
        .arg(req.subdomain.clone())
        .output()
        .expect("set ddns command failed to start");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    DDNSResponseSerializer {
        subdomain: req.subdomain.clone(),
        ip: req.ip.clone(),
    }
}
