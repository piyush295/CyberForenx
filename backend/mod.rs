use actix_web::web;
use crate::controllers::user_controller::{register_user, login_user};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/users")
            .route("/register", web::post().to(register_user))
            .route("/login", web::post().to(login_user)),
    );
}