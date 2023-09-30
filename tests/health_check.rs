use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let app_address = spawn_app();
    println!("{}", &app_address);

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", app_address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind a random port");
    let address = listener.local_addr().unwrap();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    tokio::spawn(server);

    format!("http://{}:{}", address.ip(), address.port())
}