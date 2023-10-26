use axum::Router;
use tower_http::services::ServeDir;

mod authentication;
mod error;
mod extractor;
mod utils;

pub async fn server() {
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router_web().into_make_service())
        .await
        .unwrap();
}

fn router_web() -> Router {
    return Router::new()
        .nest_service("/public", ServeDir::new("public"))
        .merge(authentication::router());
}
