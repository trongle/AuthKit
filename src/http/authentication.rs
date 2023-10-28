use super::{
    error::{ApplicationError, ErrorBag, RenderErrorsAsHtml},
    extractor::ValidatedForm,
    utils::deserialize_empty_string_as_none,
    AppContext,
};
use crate::view::authentication::{register_form, register_page, Input, InputKind};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use axum::{extract::State, http::HeaderMap, response::IntoResponse, routing::get, Router};
use maud::{html, Markup, PreEscaped};
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

    let mut headers = HeaderMap::new();
    return match result {
        Ok(_) => {
            headers.insert("HX-Location", "/login".parse().unwrap());

            Ok((headers, PreEscaped("".to_string())))
        }
        Err(err) => match err {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                let (label, field_name, input_type, value) =
                    if db_err.message().contains("users.username") {
                        (
                            "Username",
                            "username",
                            "text",
                            request.username.as_deref().unwrap_or(""),
                        )
                    } else {
                        (
                            "Email",
                            "email",
                            "email",
                            request.email.as_deref().unwrap_or(""),
                        )
                    };

                let html = html! {
                    input id=(field_name)
                        type=(input_type)
                        class={ "input input-bordered bg-white input-error" }
                        name=(field_name)
                        value=(value)
                        required;
                        label class="label text-red-500" for="username" {
                            span { (label)" already exists." }
                    }
                };

                headers.insert("HX-Reswap", "innerHTML".parse().unwrap());
                headers.insert(
                    "HX-Retarget",
                    format!("#control_{}", field_name).parse().unwrap(),
                );

                Ok((headers, html))
            }
            _ => Err(ApplicationError::ServerError(format!(
                "Failed to register: {}",
                err
            ))),
        },
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
