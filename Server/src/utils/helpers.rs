use actix_web::HttpRequest;
use anyhow::Context;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::config::settings::Config;
use crate::db::paste_db_operations::PasteDbOperations;
use crate::db::scylla_db_operations::ScyllaDbOperations;


pub fn number_text_to_uuid(number: String) -> Uuid {
    let id: u128 =
        number.parse()
        .context("Failed to parse ID as u128").expect("Can't Paste text to u128");
    Uuid::from_u128(id)
}
pub async fn extract_user_id(req: &HttpRequest, db: &ScyllaDbOperations, config: &Config) -> Option<Uuid> {
    if let Some(token) = extract_user_token(req, config).await {
        if let Ok(Some(user_id)) = db.get_userid_by_token(token).await {
            return Some(user_id);
        }
    }
    None
}

pub async fn extract_user_token<'a>(req: &'a HttpRequest, config: &Config) -> Option<&'a str> {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_token) = auth_header.to_str() {
            if auth_token.len() == config.token_size as usize {
                return Some(auth_token);
            }
        }
    }
    None
}
pub fn time_difference_in_seconds(date: Option<DateTime<Utc>>) -> Option<i64> {
    match date {
        Some(date) => {
            let now = Utc::now();
            let duration = date.signed_duration_since(now);
            if duration.num_seconds() >= 0 {
                Some(duration.num_seconds())
            } else {
                Some(0)
            }
        }
        None => None,
    }
}