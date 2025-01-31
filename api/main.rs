use actix_web::{get, post, web, App, HttpServer, Responder};
use mongodb::{Client, options::ClientOptions};
use dotenv::dotenv;
use std::env;
use std::sync::Arc;

mod models;
mod controllers;

use controllers::password_controller::{create_entry, get_entries};
use crate::models::password_entry::PasswordEntry; // Tambahkan ini

#[get("/entries")]
async fn handle_get_entries(client: web::Data<Arc<Client>>) -> impl Responder {
    get_entries(client).await
}

#[post("/entry")]
async fn handle_create_entry(entry: web::Json<PasswordEntry>, client: web::Data<Arc<Client>>) -> impl Responder {
    create_entry(entry, client).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");

    let client_options = ClientOptions::parse(&mongodb_uri).await.expect("Invalid MongoDB URI");
    let client = Arc::new(Client::with_options(client_options).expect("Failed to connect to MongoDB"));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(handle_get_entries)
            .service(handle_create_entry)
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
