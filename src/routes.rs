use actix_web::{get, web, HttpResponse, Responder};
use log::{error, info};
use mongodb::Client;

use crate::{models::SubmittedEmailModel, validator::email_validator, DB_NAME};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[get("/add-email/{email}")]
async fn add_email(email: web::Path<String>, client: web::Data<Client>) -> impl Responder {
    info!("Adding new email: {}", email);

    if let Err(e) = email_validator(email.clone()) {
        return e;
    }

    let now = chrono::Utc::now();
    let now_str = now.to_rfc3339();

    let collection = client
        .database(DB_NAME.as_str())
        .collection::<SubmittedEmailModel>("emails");

    // Add the email to the database
    match collection
        .insert_one(
            SubmittedEmailModel {
                email: email.to_string(),
                created_at: now_str,
            },
            None,
        )
        .await
    {
        Ok(_) => {
            info!("Successfully added email to database");
            HttpResponse::Ok().body("Added email")
        }
        Err(e) => {
            error!("An error occurred whilst adding email to database: {} ", e);
            HttpResponse::InternalServerError().body(e.kind.to_string())
        }
    }
}
