use maud::{html, Markup, DOCTYPE};

use super::input::{Input, InputKind};

pub async fn register_page() -> Markup {
    return html! {
        (DOCTYPE)
        html data-theme="light" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width";
                title { "Register" }
                link rel="stylesheet" href="/public/css/app.css";
                script src="https://unpkg.com/htmx.org@1.9.6" {}
                script src="https://unpkg.com/htmx.org/dist/ext/morphdom-swap.js" {}
                script src="https://cdn.jsdelivr.net/npm/morphdom@2.6.1/dist/morphdom-umd.js" {}
            }
            body hx-ext="morphdom-swap" {
                main class="h-[100dvh] bg-blue-50 overflow-auto" {
                    div class="card shadow-md bg-white w-96 m-auto top-20" {
                        (register_form(html! {
                            (Input::new("Username", "username"))
                            (Input::new("Email", "email").kind(InputKind::Email))
                            (Input::new("Password", "password").kind(InputKind::Password))
                            (Input::new("Password Confirmation", "password_confirmation").kind(InputKind::Password))
                        }))
                    }
                }
            }
        }
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
