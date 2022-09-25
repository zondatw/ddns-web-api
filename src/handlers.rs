use actix_web::{ body::BoxBody, http::header::ContentType, post, web, HttpRequest, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use log;

use crate::constants::DNSState;

#[derive(Deserialize)]
struct DDNSRequestSerializer {
    subdomain: String,
    ip: String,
}

#[derive(Serialize)]
struct DDNSResponseSerializer {
    dns_key: String,
}

impl Responder for DDNSResponseSerializer {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[post("/ddns")]
pub async fn handler_ddns_set(data: web::Data<DNSState>, req: web::Json<DDNSRequestSerializer>) -> impl Responder {
    let dns_key = &data.dns_key;
    println!("DDNS subdomain: {}, ip: {}", req.subdomain, req.ip);

    DDNSResponseSerializer {
        dns_key: dns_key.clone()
    }
}