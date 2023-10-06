use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::config::{get_config, DatabaseConfig};
use zero2prod::startup::run;

#[tokio::test]
async fn health_check_works() {
    let TestApp { url, .. } = spawn_app().await;
    println!("{}", &url);

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", url))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let TestApp { db_pool, url } = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=crispin%20koech&email=cris_pin%40email.com";
    let response = client
        .post(&format!("{}/subscribe", &url))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16());

    let subscription = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&db_pool)
        .await
        .expect("Failed to save subscription");

    assert_eq!(subscription.email, "cris_pin@email.com");
    assert_eq!(subscription.name, "crispin koech");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let TestApp { url, .. } = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = [
        ("name=le%20name", "email is missing"),
        ("email=le%20name%40email.com", "name is missing"),
        ("", "email and name are missing"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscribe", &url))
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

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind a random port");
    let address = listener.local_addr().unwrap();
    let url = format!("http://{}:{}", address.ip(), address.port());

    let mut app_config = get_config().expect("Failed to read config file");
    app_config.database.database_name = Uuid::new_v4().to_string();
    let db_pool = configure_db(&app_config.database).await;
    let server = run(listener, db_pool.clone()).expect("Failed to bind address");
    tokio::spawn(server);

    TestApp { db_pool, url }
}

async fn configure_db(config: &DatabaseConfig) -> PgPool {
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres");
    // Create database
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create a database");
    // Migrate database
    let db_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to postgres");
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Failed to apply database migrations");

    db_pool
}

pub struct TestApp {
    pub db_pool: PgPool,
    pub url: String,
}
