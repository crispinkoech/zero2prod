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

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let body = "name=crispin%20koech&email=cris_pin%40email.com";
    let response = client
        .post(&format!("{}/subscribe", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16())
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = [
        ("name=le%20name", "email is missing"),
        ("email=le%20name%40email.com", "name is missing"),
        ("", "email and name are missing"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscribe", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "API did not fail with `400 Bad Request` when {}",
            error_message,
        )
    }
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind a random port");
    let address = listener.local_addr().unwrap();
    let server = zero2prod::startup::run(listener).expect("Failed to bind address");
    tokio::spawn(server);

    format!("http://{}:{}", address.ip(), address.port())
}
