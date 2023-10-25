use axum::{
    async_trait,
    extract::{rejection::FormRejection, FromRequest},
    http::Request,
    response::IntoResponse,
    routing::get,
    Form, Router,
};
use maud::{html, Markup, PreEscaped, DOCTYPE};
use serde::{de::DeserializeOwned, Deserialize, Deserializer};
use tower_http::services::ServeDir;
use validator::{Validate, ValidationErrors, ValidationErrorsKind};

#[derive(Deserialize, Debug, Validate)]
struct RegisterRequest {
    #[serde(deserialize_with = "deserialize_option_string")]
    #[validate(
        required(message = "This field is required and the length is in range 5-12"),
        length(
            min = 5,
            max = 12,
            message = "This field is required and the length is in range 5-12"
        )
    )]
    username: Option<String>,

    #[serde(deserialize_with = "deserialize_option_string")]
    #[validate(
        required(message = "This field is required."),
        email(message = "Invalid email.")
    )]
    email: Option<String>,

    #[serde(deserialize_with = "deserialize_option_string")]
    #[validate(required(message = "This field is required."))]
    password: Option<String>,

    #[serde(deserialize_with = "deserialize_option_string")]
    #[validate(
        required(message = "This field is required."),
        must_match(other = "password", message = "Does not match with password field.")
    )]
    password_confirmation: Option<String>,
}

fn deserialize_option_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<String> = Option::deserialize(deserializer)?;

    return match value {
        Some(v) if !v.is_empty() => Ok(Some(v)),
        _ => Ok(None),
    };
}

#[derive(Debug)]
struct ValidatedForm<T>(T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for ValidatedForm<T>
where
    S: Send + Sync,
    B: Send + 'static,
    T: DeserializeOwned + Validate,
    Form<T>: FromRequest<S, B, Rejection = FormRejection>,
{
    type Rejection = ServerError;

    async fn from_request(request: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        return match Form::<T>::from_request(request, state).await {
            Err(err) => Err(ServerError::AxumFormRejection(err)),
            Ok(Form(value)) => match value.validate() {
                Err(err) => Err(ServerError::ValidationError(err)),
                Ok(_) => Ok(ValidatedForm(value)),
            },
        };
    }
}

#[derive(Debug)]
enum ServerError {
    ValidationError(validator::ValidationErrors),
    AxumFormRejection(FormRejection),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
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

        return match self {
            ServerError::ValidationError(errs) => {
                html! {
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
                                    //value=(value.username.unwrap_or("".to_string()))
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
                                    //value=(value.email.unwrap_or("".to_string()))
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
                                    //value=(value.password.unwrap_or("".to_string()))
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
                                    //value=(value.password_confirmation.unwrap_or("".to_string()))
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
                }
            }
            ServerError::AxumFormRejection(_) => PreEscaped("".to_string()),
        }
        .into_response();
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest_service("/public", ServeDir::new("public"))
        .route(
            "/register",
            get(|| async {
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
            ).post(|ValidatedForm(request): ValidatedForm<RegisterRequest>| async move {
                println!("{:?}", request);
            })
        );

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn layout(title: &str, body: Markup, script: Option<Markup>) -> Markup {
    return html! {
        (DOCTYPE)
        html data-theme="light" {
            (header(title))
            body class="grid place-items-center h-[100dvh] bg-slate-100" {
                (body)
                (footer())
                (if let Some(s) = script { s } else { PreEscaped("".to_string()) })
            }
        }
    };
}

fn header(title: &str) -> Markup {
    return html! {
        head {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width";
            title {(title)}
            link rel="stylesheet" href="/public/css/app.css";
        }
    };
}

fn footer() -> Markup {
    return html! {
        footer {}
    };
}
