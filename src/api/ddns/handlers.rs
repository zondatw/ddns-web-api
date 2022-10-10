use actix_web::{web, Responder};
use std::process::Command;

use super::serializers::{DDNSRequestSerializer, DDNSResponseSerializer};
use crate::api::constants::DNSState;
use crate::core::serializers::ErrorResponder;

pub async fn handler_ddns_set(
    data: web::Data<DNSState>,
    req: web::Json<DDNSRequestSerializer>,
) -> Result<impl Responder, ErrorResponder> {
    let dns_key = &data.dns_key;
    println!(
        "DDNS subdomain: {}, ip: {}, dns_key: {}",
        req.subdomain, req.ip, dns_key
    );
    let output = Command::new("echo")
        .arg(req.subdomain.clone())
        .output()
        .expect("set ddns command failed to start");

    return Err(ErrorResponder::BadClientData);
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    Ok(DDNSResponseSerializer {
        subdomain: req.subdomain.clone(),
        ip: req.ip.clone(),
    })
}
