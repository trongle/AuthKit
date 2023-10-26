use super::{
    error::RenderErrorsAsHtml, extractor::ValidatedForm, utils::deserialize_empty_string_as_none,
};
use crate::view::authentication::{input, register_form, register_page};
use axum::{routing::get, Router};
use maud::{html, Markup};
use serde::Deserialize;
use validator::{Validate, ValidationErrors};

#[derive(Deserialize, Debug, Validate)]
struct RegisterRequest {
    #[serde(deserialize_with = "deserialize_empty_string_as_none")]
    #[validate(
        required(message = "This field is required and the length is in range 5-12"),
        length(
            min = 5,
            max = 12,
            message = "This field is required and the length is in range 5-12"
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
    fn render(&self, errs: ValidationErrors) -> Markup {
        let errs = Some(&errs);

        return register_form(html! {
            (input("username", errs, self.username.as_deref()))
            (input("email", errs, self.email.as_deref()))
            (input("password", errs, self.password.as_deref()))
            (input("password_confirmation", errs, self.password_confirmation.as_deref()))
        });
    }
}

pub fn router() -> Router {
    return Router::new().route("/register", get(register_page).post(store));
}

async fn store(ValidatedForm(request): ValidatedForm<RegisterRequest>) {}
