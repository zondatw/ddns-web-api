use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use std::sync::Mutex;
use dotenv::dotenv;

mod handlers;
mod constants;
mod serializers;

use crate::handlers::handler_ddns_set;
use crate::constants::{AppState, DNSState};

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {app_name}!") // <- response with app_name
}

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

async fn counting(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {counter}") // <- response with count
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let dns_key = std::env::var("DNS_KEY").expect("DNS_KEY must be set.");

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Note: web::Data created _outside_ HttpServer::new closure
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });


    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
            .app_data(web::Data::new(DNSState {
                dns_key: dns_key.clone(),
            }))
            .app_data(counter.clone()) // <- register the created data
            .service(index)
            .route("/count", web::get().to(counting))
            .service(echo)
            .service(handler_ddns_set)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}