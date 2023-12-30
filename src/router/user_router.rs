use crate::handler::user_handler;

use axum::{
    routing::{get},
    Router
};

pub fn get_user_routes() -> Router {
    Router::new()
        .route("/get", get(user_handler::get_user))


}
