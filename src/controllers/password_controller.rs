use actix_web::{web, HttpResponse, Responder};
use mongodb::Client;
use std::sync::Arc;
use futures::stream::TryStreamExt;
use crate::models::password_entry::PasswordEntry;

pub async fn create_entry(entry: web::Json<PasswordEntry>, data: web::Data<Arc<Client>>) -> impl Responder {
    let collection = data.database("password_manager").collection::<PasswordEntry>("entries");
    let new_entry = PasswordEntry {
        id: None,
        name: entry.name.clone(),
        username: entry.username.clone(),
        password: entry.password.clone(),
        image: entry.image.clone(),
    };
    collection.insert_one(new_entry, None).await.unwrap();
    HttpResponse::Ok().json("Entry created")
}

pub async fn get_entries(data: web::Data<Arc<Client>>) -> impl Responder {
    let collection = data.database("password_manager").collection::<PasswordEntry>("entries");
    let cursor = collection.find(None, None).await.unwrap();
    let entries: Vec<_> = cursor.try_collect().await.unwrap();
    HttpResponse::Ok().json(entries)
}
