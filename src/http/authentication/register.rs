use crate::http::error::{ApplicationError, ErrorBag, RenderErrorsAsHtml};
use crate::http::extractor::ValidatedForm;
use crate::http::{utils::deserialize_empty_string_as_none, AppContext};
use crate::view::authentication::{register_form, register_page};
use crate::view::input::{Input, InputKind};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use axum::http::HeaderMap;
use axum::response::Response;
use axum::{extract::State, response::IntoResponse, routing::get, Router};
use cookie::Cookie;
use maud::{html, Markup};
use serde::Deserialize;
use validator::Validate;

pub fn router() -> Router<AppContext> {
    return Router::new().route("/register", get(register_page).post(store));
}

#[derive(Deserialize, Debug, Validate)]
struct RegisterRequest {
    #[serde(deserialize_with = "deserialize_empty_string_as_none")]
    #[validate(
        required(message = "This field is required and the length must be in range 5-12"),
        length(
            min = 5,
            max = 12,
            message = "This field is required and the length must be in range 5-12"
        )
    )]
    username: Option<String>,

    #[serde(deserialize_with = "deserialize_empty_string_as_none")]
    #[validate(
        required(message = "This field is required."),
        email(message = "Invalid email.")
    )]
    email: Option<String>,

    #[serde(deserialize_with = "deserialize_empty_string_as_none")]
    #[validate(required(message = "This field is required."))]
    password: Option<String>,

    #[serde(deserialize_with = "deserialize_empty_string_as_none")]
    #[validate(
        required(message = "This field is required."),
        must_match(other = "password", message = "Does not match with password field.")
    )]
    password_confirmation: Option<String>,
}

impl RenderErrorsAsHtml for RegisterRequest {
    fn render(&self, errors: &ErrorBag) -> Markup {
        return register_form(html! {
            (Input::new("Username", "username")
                .value(self.username.as_deref().unwrap_or(""))
                .errors(errors.get(&"username".to_string())))
            (Input::new("Email", "email")
                .kind(InputKind::Email)
                .value(self.email.as_deref().unwrap_or(""))
                .errors(errors.get(&"email".to_string())))
            (Input::new("Password", "password")
                .kind(InputKind::Password)
                .value(self.password.as_deref().unwrap_or(""))
                .errors(errors.get(&"password".to_string())))
            (Input::new("Password Confirmation", "password_confirmation")
                .kind(InputKind::Password)
                .value(self.password_confirmation.as_deref().unwrap_or(""))
                .errors(errors.get(&"password_confirmation".to_string())))
        });
    }
}

struct SuccessfulRegistrationResponse {
    redirect_to: String,
}

impl SuccessfulRegistrationResponse {
    fn new(redirect_to: String) -> Self {
        return Self { redirect_to };
    }
}

impl IntoResponse for SuccessfulRegistrationResponse {
    fn into_response(self) -> Response {
        let mut headers = HeaderMap::new();

        headers.insert("HX-Location", self.redirect_to.parse().unwrap());
        headers.insert(
            "Set-Cookie",
            Cookie::build(("successfully_registered", "true"))
                .secure(true)
                .same_site(cookie::SameSite::Strict)
                .http_only(true)
                .build()
                .encoded()
                .to_string()
                .parse()
                .unwrap(),
        );

        return (headers).into_response();
    }
}

async fn store(
    State(AppContext { db }): State<AppContext>,
    ValidatedForm(request): ValidatedForm<RegisterRequest>,
) -> Result<impl IntoResponse, ApplicationError> {
    let password_hash = hash_password(request.password.as_ref().unwrap())?;

    let result = sqlx::query!(
        "insert into users (username, email, password) values (?, ?, ?)",
        request.username,
        request.email,
        password_hash
    )
    .execute(&db)
    .await;

    return match result {
        Ok(_) => Ok(SuccessfulRegistrationResponse::new("/login".to_string())),
        Err(err) => Err(ApplicationError::ServerError(err.to_string())),
    };
}

fn hash_password(password: &str) -> Result<String, ApplicationError> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    return Ok(argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| ApplicationError::ServerError(format!("Failed to hash password: {}", e)))?
        .to_string());
}
