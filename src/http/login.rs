use super::AppContext;
use crate::view::authentication::login_page;
use axum::{body::BoxBody, http::HeaderMap, response::IntoResponse, routing::get, Router};
use cookie::Cookie;
use maud::Markup;
use time;

pub fn router() -> Router<AppContext> {
    return Router::new().route("/login", get(get_login_page));
}

struct LoginResponse<'a> {
    deleting_cookie: Cookie<'a>,
    content: Markup,
}

impl<'a> IntoResponse for LoginResponse<'a> {
    fn into_response(self) -> axum::http::Response<BoxBody> {
        let mut response_header = HeaderMap::new();

        response_header.insert(
            "Set-Cookie",
            self.deleting_cookie.encoded().to_string().parse().unwrap(),
        );

        return (response_header, self.content).into_response();
    }
}

impl<'a> LoginResponse<'a> {
    fn new(content: Markup) -> Self {
        return Self {
            deleting_cookie: Cookie::build("successfully_registered")
                .expires(time::OffsetDateTime::now_utc() - time::Duration::days(365))
                .build(),
            content,
        };
    }
}

async fn get_login_page(headers: HeaderMap) -> impl IntoResponse {
    let successfully_registered = headers
        .get_all("cookie")
        .into_iter()
        .filter_map(|v| v.to_str().ok())
        .flat_map(|v| v.split("; "))
        .filter_map(|v| Cookie::parse_encoded(v).ok())
        .find(|v| v.name() == "successfully_registered")
        .map(|v| v.value().parse::<bool>().unwrap());

    return LoginResponse::new(login_page(successfully_registered.unwrap_or(false)));
}
