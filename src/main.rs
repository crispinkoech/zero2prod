use std::net::TcpListener;

use zero2prod::config::get_config;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app_config = get_config().expect("Failed to read config file");
    let address = format!("{}:{}", &app_config.app_host, &app_config.app_port);
    let listener = TcpListener::bind(&address).expect("Failed to bind address");
    run(listener)?.await
}
