use actix_governor::Governor;
use actix_web::{web, App, HttpServer};
use lazy_static::lazy_static;
use log::info;
use mongodb::{
    bson::doc,
    options::{ClientOptions, Credential},
    Client,
};
use std::env;

use crate::routes::{add_email, index, message};
mod models;
mod routes;
mod validators;

lazy_static! {
    static ref DB_NAME: String = env::var("MONGO_DB").unwrap_or("DEV".to_owned());
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok().unwrap();
    env_logger::init();

    let governor_conf = actix_governor::GovernorConfigBuilder::default()
        .per_second(1)
        .burst_size(2)
        .finish()
        .unwrap();

    info!("Starting server");

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
            .wrap(Governor::new(&governor_conf))
            .app_data(web::Data::new(client.clone()))
            .service(index)
            .service(add_email)
            .service(message)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
