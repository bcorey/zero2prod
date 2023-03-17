//! main.rs
use zero2prod::{startup::run, configuration::get_configuration};
use zero2prod::telemetry::{get_subscriber, init_subscriber};
use std::net::TcpListener;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // panic if config can't be read
    let configuration = get_configuration().expect("Failed to read config.");
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());
    let address = format!(
        "{}:{}", 
        configuration.application.host,
        configuration.application.port,
    );
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}

