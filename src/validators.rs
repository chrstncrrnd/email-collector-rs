use actix_web::HttpResponse;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref EMAIL_REGEX: Regex = Regex::new(
        r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#
    ).unwrap();
}

// This should be more than enough
const MAX_EMAIL_LENGTH_CHARS: usize = 1024;

pub fn email_validator(email: String) -> Result<bool, HttpResponse> {
    // Make sure its not too long
    if email.chars().count() > MAX_EMAIL_LENGTH_CHARS {
        return Err(HttpResponse::PayloadTooLarge().body("Email should not exceed 1024 characters"));
    }
    // Make sure its an actual email
    if !EMAIL_REGEX.is_match(&email) {
        return Err(HttpResponse::NotAcceptable().body("Invalid email"));
    }
    Ok(true)
}

pub fn string_appropriate_size(s: String) -> bool {
    let length = s.chars().count();
    length < 50_000 && length > 0
}
