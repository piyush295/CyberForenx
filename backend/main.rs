use actix_web::{web, App, HttpServer};
use mongodb::{options::ClientOptions, Client};
use std::sync::Mutex;

mod controllers;
mod models;
mod routes;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // MongoDB client setup
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("cyberforenx");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Mutex::new(db.clone())))
            .configure(routes::init)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}