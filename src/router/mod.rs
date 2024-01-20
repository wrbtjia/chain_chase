mod user_router;
mod api_router;

use axum::{Json, Router, routing::{get}};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde_json::json;
use crate::router::user_router::get_user_routes;
use crate::router::api_router::get_api_routes;

pub fn get_routes() -> Router {
    Router::new()
        .route("/", get(index))
        .nest("/user",get_user_routes())
        .nest("/api",get_api_routes())
        .fallback(handler_404)


}

async fn index() -> &'static str { "index" }

async fn handler_404() -> impl IntoResponse {
    let json = json!({
        "code": "404",
        "msg": "路径不存在"
    });

    (StatusCode::NOT_FOUND, Json(json))
}
