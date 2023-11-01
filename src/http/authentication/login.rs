use super::AppContext;
use crate::{
    http::{
        error::{ApplicationError, ErrorBag, RenderErrorsAsHtml},
        extractor::ValidatedForm,
        utils::deserialize_empty_string_as_none,
    },
    view::authentication::{login_form, login_page},
};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{
    body::BoxBody, extract::State, http::HeaderMap, response::IntoResponse, routing::get, Router,
};
use cookie::Cookie;
use maud::{Markup, PreEscaped};
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
pub struct LoginAttempRequest {
    #[validate(required(message = "This field is required."))]
    #[serde(deserialize_with = "deserialize_empty_string_as_none")]
    pub username: Option<String>,

    #[validate(required(message = "This field is required."))]
    #[serde(deserialize_with = "deserialize_empty_string_as_none")]
    pub password: Option<String>,
}

impl RenderErrorsAsHtml for LoginAttempRequest {
    fn render(&self, errors: &ErrorBag) -> Markup {
        return login_form(Some(self), Some(errors));
    }
}

async fn store(
    State(AppContext { db }): State<AppContext>,
    ValidatedForm(request): ValidatedForm<LoginAttempRequest>,
) -> Result<impl IntoResponse, ApplicationError> {
    // Find the user by username.
    let result = sqlx::query!(
        "select password from users where username = ?",
        request.username
    )
    .fetch_optional(&db)
    .await
    .map_err(|e| ApplicationError::ServerError(e.to_string()))?;

    // If the user exists, then we will verify
    // the password from the request with hashed
    // one stored in the database.
    let result = if let Some(record) = result {
        let password_hash = PasswordHash::new(&record.password)
            .map_err(|e| ApplicationError::ServerError(e.to_string()))?;

        let result = Argon2::default().verify_password(
            request.password.as_ref().unwrap().as_bytes(),
            &password_hash,
        );

        Ok(result)
    } else {
        Err("")
    };

    // If everything ok, then we will create
    // a logged in session for the user. And
    // redirect the user to the home page.
    if result.is_ok() {
        return Ok(PreEscaped("".to_string()));
    }

    let mut errors = ErrorBag::new();
    errors.insert(
        "invalid_credentials".to_string(),
        vec!["Invalid username or password".to_string()],
    );

    return Ok(login_form(Some(&request), Some(&errors)));
}
