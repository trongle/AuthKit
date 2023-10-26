use super::{
    error::RenderErrorsAsHtml, extractor::ValidatedForm, utils::deserialize_empty_string_as_none,
};
use axum::{routing::get, Router};
use maud::{html, Markup, PreEscaped, DOCTYPE};
use serde::Deserialize;
use validator::{Validate, ValidationErrors, ValidationErrorsKind};

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
    fn render(&self, errs: validator::ValidationErrors) -> Markup {
        struct ErrorHtml<'a>(&'a ValidationErrors, &'a str);

        impl<'a> maud::Render for ErrorHtml<'a> {
            fn render(&self) -> Markup {
                let (errors, field) = (self.0, self.1);

                return if !errors.errors().contains_key(field) {
                    PreEscaped("".to_string())
                } else {
                    html! {
                        label class="label text-red-500" for=(field) {
                            span {
                                (match errors.errors().get(field) {
                                    Some(ValidationErrorsKind::Field(errors)) => errors.first().unwrap().message.as_ref().unwrap(),
                                    _ => ""
                                })
                            }
                        }
                    }
                };
            }
        }

        return html! {
            div class="card-body" {
                h1 class="card-title text-center text-2xl" { "Đăng Ký" }
                div class="form-control" {
                    label class="label" for="username" {
                        span { "Username: " span class="text-red-500" { "*" } }
                    }
                    input id="username"
                            type="input"
                            class={ "input input-bordered bg-white"
                                (if errs.errors().contains_key("username") { " input-error" } else { "" })
                            }
                            name="username"
                            value=(self.username.as_ref().unwrap_or(&"".to_string()))
                            required;
                    (ErrorHtml(&errs, "username"))
                }
                div class="form-control" {
                    label class="label" for="emaill" {
                        span { "Email: " span class="text-red-500" { "*" } }
                    }
                    input id="email"
                            type="email"
                            class={ "input input-bordered bg-white"
                                (if errs.errors().contains_key("email") { " input-error" } else { "" })
                            }
                            name="email"
                            value=(self.email.as_ref().unwrap_or(&"".to_string()))
                            required;
                    (ErrorHtml(&errs, "email"))
                }
                div class="form-control" {
                    label class="label" for="password" {
                        span { "Password: " span class="text-red-500" { "*" } }
                    }
                    input id="password"
                            type="password"
                            class={ "input input-bordered bg-white"
                                (if errs.errors().contains_key("password") { " input-error" } else { "" })
                            }
                            name="password"
                            value=(self.password.as_ref().unwrap_or(&"".to_string()))
                            required;
                    (ErrorHtml(&errs, "password"))
                }
                div class="form-control" {
                    label class="label" for="password_confirmation" {
                        span { "Confirm Password: " span class="text-red-500" { "*" } }
                    }
                    input id="password_confirmation"
                            type="password"
                            class={ "input input-bordered bg-white"
                                (if errs.errors().contains_key("password_confirmation") { " input-error" } else { "" })
                            }
                            name="password_confirmation"
                            value=(self.password_confirmation.as_ref().unwrap_or(&"".to_string()))
                            required;
                    (ErrorHtml(&errs, "password_confirmation"))
                }
                div class="flex justify-end items-center gap-4" {
                    a href="/login" class="underline" { "Đã có tài khoản?" }
                    button type="submit" class="btn btn-primary text-white" {
                        span class="loading loading-spinner loading-sm htmx-indicator" {}
                        "Lưu"
                    }
                }
            }
        };
    }
}

pub fn router() -> Router {
    return Router::new().route("/register", get(register).post(store));
}

async fn register() -> Markup {
    return html! {
        (DOCTYPE)
        html data-theme="light" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width";
                title { "Register" }
                link rel="stylesheet" href="/public/css/app.css";
                script src="https://unpkg.com/htmx.org@1.9.6" {}
            }
            body class="grid place-items-center h-[100dvh] bg-blue-50" {
                form class="card shadow-md bg-white w-96 -translate-y-1/4" hx-post="/register" novalidate {
                    div class="card-body " {
                        h1 class="card-title text-center text-2xl" { "Đăng Ký" }
                        div class="form-control" {
                            label class="label" for="username" {
                                span { "Username: " span class="text-red-500" { "*" } }
                            }
                            input id="username"
                                    type="input"
                                    class="input input-bordered bg-white"
                                    name="username"
                                    value=""
                                    required;
                        }
                        div class="form-control" {
                            label class="label" for="emaill" {
                                span { "Email: " span class="text-red-500" { "*" } }
                            }
                            input id="email"
                                    type="email"
                                    class="input input-bordered bg-white"
                                    name="email"
                                    value=""
                                    required;
                        }
                        div class="form-control" {
                            label class="label" for="password" {
                                span { "Password: " span class="text-red-500" { "*" } }
                            }
                            input id="password"
                                    type="password"
                                    class="input input-bordered bg-white"
                                    name="password"
                                    value=""
                                    required;
                        }
                        div class="form-control" {
                            label class="label" for="password_confirmation" {
                                span { "Confirm Password: " span class="text-red-500" { "*" } }
                            }
                            input id="password_confirmation"
                                    type="password"
                                    class="input input-bordered bg-white"
                                    name="password_confirmation"
                                    value=""
                                    required;
                        }
                        div class="flex justify-end items-center gap-4 mt-4" {
                            a href="/login" class="underline" { "Đã có tài khoản?" }
                            button type="submit" class="btn btn-primary text-white" {
                                span class="loading loading-spinner loading-sm htmx-indicator" {}
                                "Lưu"
                            }
                        }
                    }
                }
            }
        }
    };
}

async fn store(ValidatedForm(request): ValidatedForm<RegisterRequest>) {}
