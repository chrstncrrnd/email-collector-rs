use actix_web::{get, post, web, HttpResponse, Responder};
use log::{error, info};
use mongodb::{bson::doc, Client};
use serde::{Deserialize, Serialize};

use crate::{
    models::{SubmittedEmailModel, SubmittedMessageModel},
    validators::{email_validator, string_appropriate_size},
    DB_NAME,
};

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

    // Check if the email exists already in the database
    match collection
        .find_one(
            doc! {
                "email": email.to_string()
            },
            None,
        )
        .await
    {
        Ok(d) => {
            if let Some(_) = d {
                info!("Email {} already in database", email);
                return HttpResponse::Ok()
                    .body("No changes were made, email already exists in database");
            }
        }
        Err(e) => {
            error!("Mongo query failed: {}", e);
            return HttpResponse::InternalServerError()
                .body("Something went wrong with database query");
        }
    }

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

#[derive(Serialize, Deserialize)]
struct Message {
    email: String,
    subject: String,
    content: String,
}

#[post("/message")]
async fn message(data: web::Json<Message>, client: web::Data<Client>) -> impl Responder {
    let msg = data.0;
    info!("New message from: {}", msg.email);
    if let Err(e) = email_validator(msg.email.clone()) {
        return e;
    }

    if !string_appropriate_size(msg.content.clone())
        || !string_appropriate_size(msg.subject.clone())
    {
        return HttpResponse::PayloadTooLarge().body("The message content or subject was too long");
    }

    let now = chrono::Utc::now();
    let now_str = now.to_rfc3339();

    let collection = client
        .database(DB_NAME.as_str())
        .collection::<SubmittedMessageModel>("messages");

    // Add the email to the database
    match collection
        .insert_one(
            SubmittedMessageModel {
                email: msg.email,
                created_at: now_str,
                content: msg.content,
                subject: msg.subject,
            },
            None,
        )
        .await
    {
        Ok(_) => {
            info!("Successfully added message to database");
            HttpResponse::Ok().body("Added email")
        }
        Err(e) => {
            error!(
                "An error occurred whilst adding message to database: {} ",
                e
            );
            HttpResponse::InternalServerError().body(e.kind.to_string())
        }
    }
}
