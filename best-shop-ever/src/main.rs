use actix_web::{middleware::Logger, App, HttpServer};
use domain::item::item_service_config;
extern crate env_logger;

mod domain;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .configure(item_service_config)
            .wrap(Logger::default())
    })
    .bind("localhost:8080")?
    .run()
    .await
}
