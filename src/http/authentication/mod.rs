use super::AppContext;
use axum::Router;

mod login;
mod register;

pub fn router() -> Router<AppContext> {
    return Router::new()
        .merge(register::router())
        .merge(login::router());
}
