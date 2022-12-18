use std::net::TcpListener;
use sqlx::{Executor, PgConnection, PgPool, Connection};
use secrecy::ExposeSecret;
use zero2prod::telemetry::{init_subscriber, get_subscriber};
use once_cell::sync::Lazy;

use uuid::Uuid;
use zero2prod::configuration::{DatabaseSettings, get_configuration};
use zero2prod::email_client::EmailClient;
use zero2prod::startup::run;



// Ensure that the `tracing` stack is only initialised once using `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
// We cannot assign the output of `get_subscriber` to a variable based on the value of `TEST_LOG`
// because the sink is part of the type returned by `get_subscriber`, therefore they are not the
// same type. We could work around it, but this is the most straight-forward way of moving forward.
    // If we're testing write to std out otherwise use the given sink (writer)
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    };
});


pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);
    let mut configuration = get_configuration().expect("Failed to read configuration");
    // Override the database_name for tests to ensure a unique db is create for the test run. The ensures isolation
    // of the main database.
    configuration.database.database_name = Uuid::new_v4().to_string();

    let sender_email = configuration.email_client.sender()
        .expect("Invalid sender email address.");
    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender_email,
        configuration.email_client.authorization_token,
    );

    let connection_pool = configure_database(&configuration.database).await;
    let server = run(listener, connection_pool.clone(), email_client).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp {
        address,
        db_pool: connection_pool,
    }
}

pub async fn configure_database(database_configuration: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect(&database_configuration.connection_string_without_db().expose_secret())
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, database_configuration.database_name).as_str())
        .await
        .expect("Failed to create the database");

    let connection_pool = PgPool::connect(&database_configuration.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}
