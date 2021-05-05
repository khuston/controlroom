use actix_web;
use std::env;
use std::io;
use env_logger;
use actix_web::{HttpServer, App, middleware};

mod processes;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(processes::list)
            .service(processes::get)
            .service(processes::start)
    })
    .bind("127.0.0.1:9090")?
    .run()
    .await
}
