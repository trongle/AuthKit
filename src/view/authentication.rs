use super::input::OnChangeValidation;
use super::input::{Input, InputKind};
use crate::LoginAttempRequest;
use crate::{ErrorBag, RegisterRequest};
use maud::{html, Markup, DOCTYPE};

pub fn layout(title: &str, body: Markup) -> Markup {
    return html! {
        (DOCTYPE)
        html data-theme="light" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width";
                title { (title) }
                link rel="stylesheet" href="/public/css/app.css";
                script src="https://unpkg.com/htmx.org@1.9.6" {}
                script src="https://unpkg.com/htmx.org/dist/ext/morphdom-swap.js" {}
                script src="https://cdn.jsdelivr.net/npm/morphdom@2.6.1/dist/morphdom-umd.js" {}
            }
            body hx-ext="morphdom-swap" hx-boost="true" {
                main class="h-[100dvh] bg-blue-50 overflow-auto" {
                    div class="card shadow-md bg-white w-96 m-auto top-20" {
                        (body)
                    }
                }
            }
        }
    };
}

pub async fn register_page() -> Markup {
    return html! {
        (layout("Register", html! {
            (register_form(None, None))
        }))
    };
}

pub fn register_form(request: Option<&RegisterRequest>, errors: Option<&ErrorBag>) -> Markup {
    let mut username_input =
        Input::new("Username", "username").validate_on_change(OnChangeValidation::Username);
    let mut email_input = Input::new("Email", "email")
        .kind(InputKind::Email)
        .validate_on_change(OnChangeValidation::Email);
    let mut password_input = Input::new("Password", "password").kind(InputKind::Password);
    let mut password_confirmation_input =
        Input::new("Password Confirmation", "password_confirmation").kind(InputKind::Password);

    if let Some(e) = errors {
        username_input = username_input.errors(e.get("username"));
        email_input = email_input.errors(e.get("email"));
        password_input = password_input.errors(e.get("password"));
        password_confirmation_input =
            password_confirmation_input.errors(e.get("password_confirmation"));
    }

    if let Some(req) = request {
        username_input = username_input.value(req.username.as_deref().unwrap_or(""));
        email_input = email_input.value(req.email.as_deref().unwrap_or(""));
        password_input = password_input.value(req.password.as_deref().unwrap_or(""));
        password_confirmation_input =
            password_confirmation_input.value(req.password_confirmation.as_deref().unwrap_or(""));
    }

    return html! {
        form class="card-body" hx-post="/register" hx-swap="outerHTML" novalidate {
            h1 class="card-title text-center text-2xl" { "Register" }
            (username_input)
            (email_input)
            (password_input)
            (password_confirmation_input)
            div class="flex justify-end items-center gap-4 mt-4" {
                a href="/login" class="underline" { "Already has account?" }
                button type="submit" class="btn btn-primary text-white" {
                    span class="loading loading-spinner loading-sm htmx-indicator" {}
                    "Register"
                }
            }
        }
    };
}

pub fn login_page(successfully_registered: bool) -> Markup {
    return html! {
        (layout("Login", html! {
            div {
                @if successfully_registered {
                    div class="alert alert-success" {
                        "Your account has been created!. Now try to login with the registered information."
                    }
                }
                (login_form(None, None))
            }
        }))
    };
}

pub fn login_form(request: Option<&LoginAttempRequest>, errors: Option<&ErrorBag>) -> Markup {
    let mut username_input = Input::new("Username", "username").value(
        request
            .map(|req| req.username.as_deref().unwrap_or(""))
            .unwrap_or(""),
    );
    let mut password_input = Input::new("Password", "password")
        .kind(InputKind::Password)
        .value(
            request
                .map(|req| req.password.as_deref().unwrap_or(""))
                .unwrap_or(""),
        );
    let mut invalid_credentials = None;

    if let Some(e) = errors {
        username_input = username_input.errors(e.get("username"));
        password_input = password_input.errors(e.get("password"));
        invalid_credentials = e.get("invalid_credentials");
    }

    return html! {
        @if let Some(e) = invalid_credentials {
            div class="alert alert-error" {
                (e[0])
            }
        }
        form class="card-body" hx-post="/login" hx-swap="innerHTML" hx-target="closest div" novalidate {
            h1 class="card-title text-center text-2xl" { "Login" }
            (username_input)
            (password_input)
            div class="flex justify-end items-center gap-4 my-4" {
                a href="/register" class="underline" hx-target="body" { "Don't have account, yet?" }
                button type="submit" class="btn btn-primary text-white" {
                    span class="loading loading-spinner loading-sm htmx-indicator" {}
                    "Login"
                }
            }
        }
    };
}
