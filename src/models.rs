use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SubmittedEmailModel {
    pub email: String,
    pub created_at: String,
}
