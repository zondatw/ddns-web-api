use actix_web::{middleware::Logger, web, App, HttpServer};

mod api;
mod config;
mod core;

use crate::core::constants::DNSState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    config::base::init();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(web::Data::new(DNSState {
                dns_key: (*config::base::dns_key().lock().unwrap()).clone(),
            }))
            .configure(api::routes::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
