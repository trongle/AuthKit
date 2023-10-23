use axum::{routing::get, Router};
use maud::{html, Markup, PreEscaped, DOCTYPE};
use tower_http::services::ServeDir;

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
                            }
                            body class="grid place-items-center h-[100dvh] bg-blue-50" {
                                form class="card shadow-md bg-white w-96 -translate-y-1/2"{
                                    div class="card-body gap-4" {
                                        h1 class="card-title text-center text-2xl" { "Đăng Ký" }
                                        div class="form-control" {
                                            label class="label" for="username" {
                                                span class="font-bold" { "Username: " span class="text-red-500" { "*" } }
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
                                                span class="font-bold" { "Email: " span class="text-red-500" { "*" } }
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
                                                span class="font-bold" { "Password: " span class="text-red-500" { "*" } }
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
                                                span class="font-bold" { "Confirm Password: " span class="text-red-500" { "*" } }
                                            }
                                            input id="password_confirmation" 
                                                    type="password" 
                                                    class="input input-bordered bg-white" 
                                                    name="password_confirmation" 
                                                    value="" 
                                                    required;
                                        }
                                        div class="flex justify-end items-center gap-4" {
                                            a href="/login" class="underline" { "Đã có tài khoản?" }
                                            button class="btn btn-primary text-white" { "Lưu" }
                                        }
                                    }
                                }
                            }
                        }
                    };
                }
            )
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
