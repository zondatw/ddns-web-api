use actix_web::{ body::BoxBody, http::header::ContentType, post, web, HttpRequest, HttpResponse, Responder};
use serde::Serialize;

use crate::constants::DNSState;

#[derive(Serialize)]
struct DDNSSerializer {
    dns_key: String,
}

impl Responder for DDNSSerializer {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[post("/ddns")]
pub async fn handler_ddns_set(data: web::Data<DNSState>) -> impl Responder {
    let dns_key = &data.dns_key;

    DDNSSerializer {
        dns_key: dns_key.clone()
    }
}