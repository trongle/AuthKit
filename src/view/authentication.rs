use std::fmt::Display;

use maud::{html, Markup, PreEscaped, Render, DOCTYPE};

pub struct Input<'a> {
    pub label: &'a str,
    pub field_name: &'a str,
    pub kind: InputKind,
    pub value: Option<&'a str>,
    pub errors: Option<&'a Vec<String>>,
}

pub enum InputKind {
    Text,
    Email,
    Password,
}

impl Display for InputKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return f.write_str(match self {
            InputKind::Text => "text",
            InputKind::Email => "email",
            InputKind::Password => "password",
        });
    }
}

impl<'a> Input<'a> {
    pub fn new(label: &'a str, field_name: &'a str) -> Self {
        return Self {
            label,
            field_name,
            kind: InputKind::Text,
            value: None,
            errors: None,
        };
    }

    pub fn kind(mut self, kind: InputKind) -> Self {
        self.kind = kind;
        return self;
    }

    pub fn errors(mut self, errors: Option<&'a Vec<String>>) -> Self {
        self.errors = errors;
        return self;
    }

    pub fn value(mut self, value: &'a str) -> Self {
        self.value = Some(value);
        return self;
    }
}

impl<'a> Render for Input<'a> {
    fn render(&self) -> Markup {
        return html! {
            div class="form-control" id={ "control_"(self.field_name) } {
                label class="label" for=(self.field_name) {
                    span class="capitalize" { (self.label)": " span class="text-red-500" { "*" } }
                }
                input id=(self.field_name)
                        type=(self.kind.to_string())
                        class={ "input input-bordered bg-white "(if self.errors.is_some() { "input-error" } else { "" }) }
                        name=(self.field_name)
                        value=(self.value.unwrap_or(""))
                        required
                        hx-trigger="keyup changed delay:500ms"
                        hx-swap=[match self.field_name {
                            "email" | "username" => Some("morphdom"),
                            _ => None
                        }]
                        hx-post=[match self.field_name {
                            "email" | "username" => Some(format!("/check-{}", self.field_name)),
                            _ => None
                        }]
                        hx-target=[match self.field_name {
                            "email" | "username" => Some(format!("#control_{}",self.field_name)),
                            _ => None
                        }];
                (self.errors.map_or(PreEscaped("".to_string()), |errors| error(self.field_name, &errors)))
            }
        };
    }
}

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

fn error(field_name: &str, errors: &Vec<String>) -> Markup {
    return html! {
        label class="label text-red-500" for=(field_name) {
            @for error in errors {
                span { (error) }
            }
        }
    };
}
