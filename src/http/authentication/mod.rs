use super::AppContext;
use axum::Router;

mod login;
mod register;

pub use login::LoginAttempRequest;
pub use register::RegisterRequest;

pub fn router() -> Router<AppContext> {
    return Router::new()
        .merge(register::router())
        .merge(login::router());
}
