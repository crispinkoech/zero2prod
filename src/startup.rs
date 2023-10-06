use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;

use super::routes::*;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let app_factory = move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
            .app_data(db_pool.clone())
    };

    let server = HttpServer::new(app_factory).listen(listener)?.run();
    Ok(server)
}
