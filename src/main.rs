use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use lazy_static::lazy_static;
use mongodb::{
    options::{ClientOptions, Credential},
    Client,
};
use regex::Regex;
use std::env;

use serde::{Deserialize, Serialize};

lazy_static! {
    static ref EMAIL_REGEX: Regex = Regex::new(
        r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#
    ).unwrap();

    static ref DB_NAME: String = env::var("MONGO_DB").unwrap_or("DEV".to_owned());
}

// This should be more than enough
const MAX_EMAIL_LENGTH_CHARS: usize = 1024;

#[derive(Serialize, Deserialize)]
struct Model {
    email: String,
    created_at: String,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[get("/add-email/{email}")]
async fn add_email(email: web::Path<String>, client: web::Data<Client>) -> impl Responder {
    // Make sure its not too long
    if email.chars().count() > MAX_EMAIL_LENGTH_CHARS {
        HttpResponse::PayloadTooLarge().body("Email should not exceed 1024 characters");
    }
    // Make sure its an actual email
    if !EMAIL_REGEX.is_match(&email) {
        HttpResponse::NotAcceptable().body("Invalid email");
    }

    let now = chrono::Utc::now();
    let now_str = now.to_rfc3339();

    match client
        .database(DB_NAME.as_str())
        .collection::<Model>("emails")
        .insert_one(
            Model {
                email: email.to_string(),
                created_at: now_str,
            },
            None,
        )
        .await
    {
        Ok(_) => HttpResponse::Ok().body("Added email"),
        Err(e) => HttpResponse::InternalServerError().body(e.kind.to_string()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok().unwrap();
    env_logger::init();

    let username = env::var("MONGO_USER").unwrap();
    let password = env::var("MONGO_PASSWORD").unwrap();
    let mongo_url = env::var("MONGO_URL").unwrap();

    let mut client_options = ClientOptions::parse(mongo_url).await.unwrap();

    client_options.credential = Some(
        Credential::builder()
            .username(username)
            .password(password)
            .build(),
    );

    let client = Client::with_options(client_options).unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
