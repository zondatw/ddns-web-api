use actix_web::{middleware::Logger, App, HttpServer};

mod api;
mod config;
mod core;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    config::base::init();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let server_domain = (*config::base::server_domain().lock().unwrap()).clone();
    let server_port = *config::base::server_port().lock().unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .configure(core::routes::config)
    })
    .bind((server_domain, server_port))?
    .run()
    .await
}
