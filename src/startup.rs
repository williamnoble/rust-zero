use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use sqlx::{PgPool};
use tracing_actix_web::TracingLogger;
use crate::routes::{greet, health_check, subscribe};

pub fn run (listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .route("/{name}", web::get().to(greet))
            .app_data(db_pool.clone())
    })
        // pass in a listener so we can modify this value depending on if we call from main or tests
        .listen(listener)?
        .run();
    Ok(server)
}