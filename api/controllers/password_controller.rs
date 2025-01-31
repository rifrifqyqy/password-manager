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

    match collection.insert_one(new_entry, None).await {
        Ok(_) => {
            println!("Entry berhasil dibuat");
            HttpResponse::Ok().json("Entry created")
        }
        Err(err) => {
            eprintln!("Error saat menyimpan entry: {}", err);
            HttpResponse::InternalServerError().json("Gagal membuat entry")
        }
    }
}

pub async fn get_entries(data: web::Data<Arc<Client>>) -> impl Responder {
    let collection = data.database("password_manager").collection::<PasswordEntry>("entries");

    match collection.find(None, None).await {
        Ok(cursor) => {
            match cursor.try_collect::<Vec<PasswordEntry>>().await {
                Ok(entries) => HttpResponse::Ok().json(entries),
                Err(err) => {
                    eprintln!("Error saat mengambil entries: {}", err);
                    HttpResponse::InternalServerError().json("Gagal mengambil data")
                }
            }
        }
        Err(err) => {
            eprintln!("Error saat query database: {}", err);
            HttpResponse::InternalServerError().json("Gagal mengambil data dari database")
        }
    }
}
