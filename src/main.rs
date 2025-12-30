use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

// or #[tokio::main] will also work
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Panic if the app can't read the configuration
    let configuration = get_configuration().expect("failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("failed to connect to Postgres.");
    let address = format!("0.0.0.0:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    run(listener, connection_pool)?.await
}
