use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SubmittedEmailModel {
    pub email: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct SubmittedMessageModel {
    pub email: String,
    pub subject: String,
    pub content: String,
    pub created_at: String,
}
