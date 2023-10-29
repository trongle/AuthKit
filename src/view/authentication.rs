use maud::{html, Markup, DOCTYPE};

use super::input::{Input, InputKind};

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

pub fn login_page(successfully_registered: Option<&str>) -> Markup {
    return html! {
        (layout("Login", html! {
            @if let Some(message) = successfully_registered {
                div class="alert alert-success" {
                    svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6" {
                        path stroke-linecap="round" stroke-linejoin="round" d="M9 12.75L11.25 15 15 9.75M21 12a9 9 0 11-18 0 9 9 0 0118 0z";
                    }
                    (message)
                }
            }
            (login_form(html! {
                (Input::new("Username", "username"))
                (Input::new("Password", "password").kind(InputKind::Password))
            }))
        }))
    };
}

pub fn login_form(inputs: Markup) -> Markup {
    return html! {
        form class="card-body" hx-post="/login" hx-swap="outerHTML" novalidate {
            h1 class="card-title text-center text-2xl" { "Login" }
            (inputs)
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
