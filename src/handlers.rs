use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use crate::constants::DNSState;

#[post("/ddns")]
pub async fn handler_ddns_set(data: web::Data<DNSState>) -> impl Responder {
    let dns_key = &data.dns_key;
    format!("DDNS dns key {dns_key}!")
}