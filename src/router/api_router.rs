use axum::Router;
use axum::routing::get;
use crate::handler::misttrack_handler;

pub fn get_api_routes() -> Router {
    Router::new()
        .route("/labels", get(misttrack_handler::get_labels))
        .route("/status", get(misttrack_handler::get_status))
        .route("/overview", get(misttrack_handler::get_overview))
        .route("/risk_score", get(misttrack_handler::get_risk_score))
        .route("/transactions_investigation", get(misttrack_handler::get_transactions_investigation))


}
