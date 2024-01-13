use crate::handler::user_handler;

use axum::{
    routing::{get},
    Router
};
use axum::routing::post;

pub fn get_user_routes() -> Router {
    Router::new()
        .route("/get", get(user_handler::get_user))
        .route("/login", post(user_handler::login))
        .route("/verify", get(user_handler::verify))


}
