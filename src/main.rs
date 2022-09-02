use std::net::TcpListener;
use sqlx::{PgPool};
use tracing::info;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};
use zero2prod::configuration::get_configuration;
use secrecy::ExposeSecret;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // create the default subscriber then use LogTracer to convert logs to tracing and set the global
    // default subscriber
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);


    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres.");

    // information only
    let address = format!("127.0.0.1:{}", configuration.application_port);
    info!("Listening on address: {}", &address);

    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
