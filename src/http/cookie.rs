use axum::{http::HeaderMap, response::IntoResponse, routing::get, Router};
use cookie::Cookie;

use super::AppContext;

pub fn router() -> Router<AppContext> {
    return Router::new().route("/cookie", get(get_cookie));
}

async fn get_cookie(headers: HeaderMap) -> impl IntoResponse {
    let cookies = headers
        .get_all("cookie")
        .into_iter()
        .filter_map(|c| c.to_str().ok())
        .flat_map(|c| c.split("; "))
        .filter_map(|c| Cookie::parse_encoded(c).ok())
        .for_each(|c| println!("{}", c.value()));

    let mut response_header = HeaderMap::new();

    response_header.insert(
        "Set-Cookie",
        Cookie::new("cookie", "cookie")
            .encoded()
            .to_string()
            .parse()
            .unwrap(),
    );

    return (response_header, "cookie cookie");
}
