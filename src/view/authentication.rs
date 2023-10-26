use maud::{html, Markup, PreEscaped, DOCTYPE};
use validator::{ValidationErrors, ValidationErrorsKind};

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
            }
            body {
                main class="h-[100dvh] bg-blue-50 overflow-auto" {
                    div class="card shadow-md bg-white w-96 m-auto top-20" {
                        (register_form(html! {
                            (input("Username", "username", None, None))
                            (input("Email", "email", None, None))
                            (input("Password", "password", None, None))
                            (input("Password Confirmation", "password_confirmation", None, None))
                        }))
                    }
                }
            }
        }
    };
}

pub fn input(
    label: &str,
    field_name: &str,
    errs: Option<&ValidationErrors>,
    value: Option<&str>,
) -> Markup {
    let (error_html, error_class) = if let Some(e) = errs {
        (error(field_name, e), "input-error")
    } else {
        (PreEscaped("".to_string()), "")
    };

    return html! {
        div class="form-control" {
            label class="label" for=(field_name) {
                span class="capitalize" { (label)": " span class="text-red-500" { "*" } }
            }
            input id=(field_name)
                    type="input"
                    class={ "input input-bordered bg-white "(error_class) }
                    name=(field_name)
                    value=(value.unwrap_or(""))
                    required;
            (error_html)
        }
    };
}

pub fn register_form(inputs: Markup) -> Markup {
    return html! {
        form class="card-body" hx-post="/register" hx-swap="outerHTML" novalidate {
            h1 class="card-title text-center text-2xl" { "Đăng Ký" }
            (inputs)
            div class="flex justify-end items-center gap-4 mt-4" {
                a href="/login" class="underline" { "Đã có tài khoản?" }
                button type="submit" class="btn btn-primary text-white" {
                    span class="loading loading-spinner loading-sm htmx-indicator" {}
                    "Lưu"
                }
            }
        }
    };
}

fn error<'a>(field: &'a str, errors: &ValidationErrors) -> Markup {
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
