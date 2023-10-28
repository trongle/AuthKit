use axum::Router;
use sqlx::MySqlPool;
use tower_http::services::ServeDir;

mod authentication;
mod error;
mod extractor;
mod utils;

#[derive(Clone)]
pub struct AppContext {
    db: MySqlPool,
}

pub async fn server(db: MySqlPool) {
    let app_context = AppContext { db };

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router_web().with_state(app_context).into_make_service())
        .await
        .unwrap();
}

fn router_web() -> Router<AppContext> {
    return Router::new()
        .nest_service("/public", ServeDir::new("public"))
        .merge(authentication::router());
}