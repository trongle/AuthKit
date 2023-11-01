use super::{utils::deserialize_empty_string_as_none, AppContext};
use crate::view::input::{Input, InputKind, OnKeyUpValidation};
use axum::{
    extract::{Form, State},
    routing::post,
    Router,
};
use maud::{html, Markup};
use serde::Deserialize;
use validator::validate_email;

pub fn router() -> Router<AppContext> {
    return Router::new().route("/check-email", post(check_email));
}

#[derive(Deserialize)]
struct CheckEmailRequest {
    #[serde(deserialize_with = "deserialize_empty_string_as_none")]
    email: Option<String>,
}

async fn check_email(
    State(AppContext { db }): State<AppContext>,
    Form(request): Form<CheckEmailRequest>,
) -> Markup {
    let mut email_input = Input::new("Email", "email")
        .kind(InputKind::Email)
        .custom_validation(OnKeyUpValidation::Email);

    if request.email.is_none() {
        return html! {
            (email_input.errors(Some(&vec!["This field is required.".to_string()])))
        };
    }

    let email = request.email.unwrap();

    email_input = email_input.value(&email);

    if !validate_email(&email) {
        return html! {
            (email_input.errors(Some(&vec!["Invalid email.".to_string()])))
        };
    }

    let result = sqlx::query!("select count(*) as count from users where email = ?", email)
        .fetch_one(&db)
        .await
        .unwrap();

    return if result.count >= 1 {
        html! {
            (email_input.errors(Some(&vec!["Email already exists.".to_string()])))
        }
    } else {
        html! { (email_input) }
    };
}
