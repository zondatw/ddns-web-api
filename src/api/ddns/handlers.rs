use actix_web::{web, Responder};
use std::process::{Command, Stdio};

use super::serializers::{DDNSRequestSerializer, DDNSResponseSerializer};
use crate::api::constants::DNSState;
use crate::core::serializers::ErrorResponder;

pub async fn handler_ddns_set(
    data: web::Data<DNSState>,
    req: web::Json<DDNSRequestSerializer>,
) -> Result<impl Responder, ErrorResponder> {
    let dns_key = &data.dns_key;
    let dns_server = &data.dns_server;
    let dns_base_domain = &data.dns_base_domain;
    let subdomain = &req.subdomain;
    let ip = &req.ip;
    println!(
        "DDNS subdomain: {}, ip: {}, dns_key: {}, dns_server: {}",
        subdomain, ip, dns_key, dns_server
    );

    let mut echo_output_child = Command::new("echo")
        .arg(format!("server {dns_server}\nupdate add {subdomain}.{dns_base_domain} 86400 A {ip}\nsend\nquit\n"))
        .stdout(Stdio::piped())
        .spawn().unwrap();

    if let Some(echo_output) = echo_output_child.stdout.take() {
        let grep_output_child = Command::new("nsupdate")
            .arg("-y")
            .arg(dns_key.clone())
            .stdin(echo_output)
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        echo_output_child.wait().unwrap();
        let head_stdout = grep_output_child.wait_with_output().unwrap();

        println!("status: {}", head_stdout.status);
        println!("stdout: {}", String::from_utf8_lossy(&head_stdout.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&head_stdout.stderr));

        if !head_stdout.status.success() {
            return Err(ErrorResponder::BadClientData);
        }
    } else {
        return Err(ErrorResponder::BadClientData);
    }

    Ok(DDNSResponseSerializer {
        subdomain: req.subdomain.clone(),
        ip: req.ip.clone(),
    })
}
