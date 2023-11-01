use super::input::{Input, InputKind};
use crate::ErrorBag;
use crate::LoginRequest;
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
            (register_form(html! {
                (Input::new("Username", "username"))
                (Input::new("Email", "email").kind(InputKind::Email))
                (Input::new("Password", "password").kind(InputKind::Password))
                (Input::new("Password Confirmation", "password_confirmation").kind(InputKind::Password))
            }))
        }))
    };
}

pub fn register_form(inputs: Markup) -> Markup {
    return html! {
        form class="card-body" hx-post="/register" hx-swap="outerHTML" novalidate {
            h1 class="card-title text-center text-2xl" { "Register" }
            (inputs)
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
            @if  successfully_registered {
                    div class="alert alert-success" {
                        "Your account has been created!. Now try to login with the registered infomation."
                    }
            }
            (login_form(None, None))
        }))
    };
}

pub fn login_form(request: Option<&LoginRequest>, errors: Option<&ErrorBag>) -> Markup {
    let mut username_input = Input::new("Username", "username").value(
        request
            .map(|req| req.username.as_deref().unwrap_or(""))
            .unwrap_or(""),
    );
    let mut password_input = Input::new("Password", "password").value(
        request
            .map(|req| req.password.as_deref().unwrap_or(""))
            .unwrap_or(""),
    );

    username_input = if let Some(e) = errors {
        username_input.errors(e.get("username"))
    } else {
        username_input
    };
    password_input = if let Some(e) = errors {
        password_input.errors(e.get("password"))
    } else {
        password_input
    };

    return html! {
        form class="card-body" hx-post="/login" hx-swap="outerHTML" novalidate {
            h1 class="card-title text-center text-2xl" { "Login" }
            (username_input)
            (password_input)
            div class="flex justify-end items-center gap-4 my-4" {
                a href="/register" class="underline" { "Don't have account, yet?" }
                button type="submit" class="btn btn-primary text-white" {
                    span class="loading loading-spinner loading-sm htmx-indicator" {}
                    "Login"
                }
            }
        }
    };
}
