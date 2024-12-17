use actix_web::{web, HttpResponse};
use mongodb::Database;
use serde::Deserialize;
use crate::services::user_service::{create_user, validate_user_password};

#[derive(Deserialize)]
pub struct RegisterUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

pub async fn register_user(db: web::Data<Mutex<Database>>, user: web::Json<RegisterUser>) -> HttpResponse {
    let user = user.into_inner();
    let new_user = User {
        id: None,
        name: user.name,
        email: user.email,
        password: user.password,
    };

    match create_user(&db.lock().unwrap(), new_user).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn login_user(db: web::Data<Mutex<Database>>, user: web::Json<LoginUser>) -> HttpResponse {
    let user = user.into_inner();
    if validate_user_password(&db.lock().unwrap(), &user.email, &user.password).await {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::Unauthorized().finish()
    }
}