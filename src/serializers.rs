use actix_web::{ body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct DDNSRequestSerializer {
    pub subdomain: String,
    pub ip: String,
}

#[derive(Serialize)]
pub struct DDNSResponseSerializer {
    pub dns_key: String,
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