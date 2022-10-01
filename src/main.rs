use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;

mod core;
mod api;

use crate::core::constants::{DNSState};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let dns_key = std::env::var("DNS_KEY").expect("DNS_KEY must be set.");

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(web::Data::new(DNSState {
                dns_key: dns_key.clone(),
            }))
            .configure(api::routes::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
