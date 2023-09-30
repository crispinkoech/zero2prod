use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};
use serde::Deserialize;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let app_factory = || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
    };

    let server = HttpServer::new(app_factory).listen(listener)?.run();
    Ok(server)
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn subscribe(form: web::Form<SubscribeFormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
struct SubscribeFormData {
    email: String,
    name: String,
}
