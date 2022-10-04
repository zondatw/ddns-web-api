use actix_web::{middleware::Logger, App, HttpServer};

mod api;
mod config;
mod core;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    config::base::init();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .configure(core::routes::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
