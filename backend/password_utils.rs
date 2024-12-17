use bcrypt::{hash, verify, DEFAULT_COST};

pub fn hash_password(password: &str) -> String {
    hash(password, DEFAULT_COST).unwrap()
}

pub fn verify_password(password: &str, hashed_password: &str) -> bool {
    verify(password, hashed_password).unwrap()
}use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub email: String,
    pub password: String,
}