use super::{utils::deserialize_empty_string_as_none, AppContext};
use crate::view::input::Input;
use crate::view::input::OnChangeValidation;
use axum::{
    extract::{Form, State},
    routing::post,
    Router,
};
use maud::{html, Markup};
use serde::Deserialize;
use validator::validate_length;

pub fn router() -> Router<AppContext> {
    return Router::new().route("/check-username", post(check_username));
}

#[derive(Deserialize)]
struct CheckUsernameRequest {
    #[serde(deserialize_with = "deserialize_empty_string_as_none")]
    username: Option<String>,
}

async fn check_username(
    State(AppContext { db }): State<AppContext>,
    Form(request): Form<CheckUsernameRequest>,
) -> Markup {
    let mut username_input =
        Input::new("Username", "username").validate_on_change(OnChangeValidation::Username);

    if request.username.is_none() {
        return html! { (username_input.errors(Some(&vec!["This field is required.".to_string()]))) };
    }

    let username = request.username.unwrap();

    username_input = username_input.value(&username);

    if !validate_length(&username, Some(5), Some(12), None) {
        return html! { (username_input.errors(Some(&vec!["The length must be in range 5-12".to_string()]))) };
    }

    let result = sqlx::query!(
        "select count(*) as count from users where username = ?",
        username
    )
    .fetch_one(&db)
    .await
    .unwrap();

    return if result.count >= 1 {
        html! { (username_input.errors(Some(&vec!["Already exists.".to_string()]))) }
    } else {
        html! { (username_input) }
    };
}
