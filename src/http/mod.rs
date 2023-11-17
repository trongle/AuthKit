use axum::{routing::get, Router};
use axum_session::{Key, Session, SessionConfig, SessionLayer, SessionRedisPool, SessionStore};
use redis_pool::RedisPool;
use sqlx::MySqlPool;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;

mod authentication;
mod check_email;
mod check_username;
mod error;
mod extractor;
mod middleware;
mod utils;

pub use authentication::{LoginAttempRequest, RegisterRequest};
pub use error::ErrorBag;

#[derive(Clone)]
pub struct AppContext {
    db: MySqlPool,
}

pub async fn server(db: MySqlPool) {
    let app_context = AppContext { db };

    // Setup redis pool connections.
    let redis_url = "redis://default@localhost:6379";
    let redis_client = redis::Client::open(redis_url).unwrap();
    let redis_pool = RedisPool::from(redis_client);

    // Setup session store.
    let session_config = SessionConfig::default()
        .with_secure(true)
        .with_cookie_same_site(axum_session::SameSite::Lax)
        .with_key(Key::generate());
    let session_store =
        SessionStore::<SessionRedisPool>::new(Some(redis_pool.clone().into()), session_config)
            .await
            .expect("Failed to create session store");

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(
            router_web()
                .with_state(app_context)
                .layer(ServiceBuilder::new().layer(SessionLayer::new(session_store)))
                .into_make_service(),
        )
        .await
        .unwrap();
}

fn router_web() -> Router<AppContext> {
    return Router::new()
        .nest_service("/public", ServeDir::new("public"))
        .route("/home", get(get_home))
        .merge(authentication::router())
        .merge(check_email::router())
        .merge(check_username::router());
}

async fn get_home(session: Session<SessionRedisPool>) -> String {
    let username: String = session.get("username").unwrap();

    return format!("Hello, {}!", username);
}
