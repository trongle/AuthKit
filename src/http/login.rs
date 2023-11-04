use super::AppContext;
use crate::view::authentication::login_page;
use axum::{http::HeaderMap, response::IntoResponse, routing::get, Router};
use cookie::Cookie;
use time;

pub fn router() -> Router<AppContext> {
    return Router::new().route("/login", get(get_login_page));
}

async fn get_login_page(headers: HeaderMap) -> impl IntoResponse {
    let successfully_registered = headers
        .get_all("cookie")
        .into_iter()
        .filter_map(|v| v.to_str().ok())
        .flat_map(|v| v.split("; "))
        .filter_map(|v| Cookie::parse_encoded(v).ok())
        .find(|v| v.name() == "successfully_registered")
        .map(|v| v.value().to_string());

    return (
        [(
            "Set-Cookie",
            Cookie::build("successfully_registered")
                .expires(time::OffsetDateTime::now_utc() - time::Duration::days(365))
                .build()
                .encoded()
                .to_string(),
        )],
        login_page(successfully_registered.as_deref()),
    );
}
