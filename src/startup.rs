use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer};

use super::routes::*;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let app_factory = || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
    };

    let server = HttpServer::new(app_factory).listen(listener)?.run();
    Ok(server)
}
