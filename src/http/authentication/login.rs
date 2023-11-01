use super::AppContext;
use crate::{
    http::{
        error::{ErrorBag, RenderErrorsAsHtml},
        extractor::ValidatedForm,
        utils::deserialize_empty_string_as_none,
    },
    view::{
        authentication::{login_form, login_page},
        input::{Input, InputKind},
    },
};
use axum::{body::BoxBody, http::HeaderMap, response::IntoResponse, routing::get, Router};
use cookie::Cookie;
use maud::{html, Markup};
use serde::Deserialize;
use time::{self, Duration};
use validator::Validate;

pub fn router() -> Router<AppContext> {
    return Router::new().route("/login", get(get_login_page).post(store));
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
                .max_age(Duration::ZERO)
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

#[derive(Deserialize, Validate, Debug)]
pub struct LoginRequest {
    #[validate(required(message = "This field is required."))]
    #[serde(deserialize_with = "deserialize_empty_string_as_none")]
    pub username: Option<String>,

    #[validate(required(message = "This field is required."))]
    #[serde(deserialize_with = "deserialize_empty_string_as_none")]
    pub password: Option<String>,
}

impl RenderErrorsAsHtml for LoginRequest {
    fn render(&self, errors: &ErrorBag) -> Markup {
        return login_form(Some(self), Some(errors));
    }
}

async fn store(ValidatedForm(request): ValidatedForm<LoginRequest>) {
    println!("{:?}", request);
}
