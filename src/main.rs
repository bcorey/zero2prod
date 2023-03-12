//! main.rs

use zero2prod::{startup::run, configuration::get_configuration};
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // panic if config can't be read
    let configuration = get_configuration().expect("Failed to read config.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
