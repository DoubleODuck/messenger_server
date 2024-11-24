use crate::extractors::Claims;
use actix_web::web;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct Response {
    message: String,
}

pub async fn encode_token(ida: Uuid, secret: web::Data<String>) -> String {
    let id: Uuid = ida;
    let exp: usize = (Utc::now() + Duration::minutes(2)).timestamp() as usize;
    let claims: Claims = Claims { id, exp };
    let token: String = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_str().as_ref()),
    )
    .unwrap();
    token
}
