use actix_web::{web, App, HttpServer};
use mongodb::{Client, options::ClientOptions};
use std::sync::Arc;
use dotenv::dotenv;
use std::env;

mod models;
mod controllers;

use controllers::password_controller::{create_entry, get_entries};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");

    let client_options = ClientOptions::parse(&mongodb_uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let client = Arc::new(client);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .route("/entries", web::get().to(get_entries))
            .route("/entry", web::post().to(create_entry))
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
