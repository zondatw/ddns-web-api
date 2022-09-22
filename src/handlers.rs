use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use crate::constants::AppState;

#[post("/ddns")]
pub async fn handler_ddns_set(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello ddns {app_name}!")
}