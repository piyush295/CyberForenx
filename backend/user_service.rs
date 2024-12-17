use crate::models::user_model::User;
use crate::utils::password_utils::{hash_password, verify_password};
use mongodb::bson::doc;
use mongodb::Database;

pub async fn create_user(db: &Database, user: User) -> mongodb::error::Result<()> {
    let collection = db.collection("users");
    let mut user = user;
    user.password = hash_password(&user.password);
    collection.insert_one(user, None).await?;
    Ok(())
}

pub async fn find_user_by_email(db: &Database, email: &str) -> mongodb::error::Result<Option<User>> {
    let collection = db.collection("users");
    let filter = doc! { "email": email };
    let user = collection.find_one(filter, None).await?;
    Ok(user)
}

pub async fn validate_user_password(db: &Database, email: &str, password: &str) -> bool {
    if let Ok(Some(user)) = find_user_by_email(db, email).await {
        return verify_password(password, &user.password);
    }
    false
}