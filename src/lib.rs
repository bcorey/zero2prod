//! src/lib.rs
pub mod configuration;
pub mod routes;
pub mod startup;

use actix_web::{web, App, HttpServer, HttpResponse};
use actix_web::dev::Server;
use std::net::TcpListener;



#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

