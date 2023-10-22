use axum::{routing::get, Router};
use maud::{html, DOCTYPE};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest_service("/public", ServeDir::new("public"))
        .route(
            "/",
            get(|| async {
                return html! {
                    (DOCTYPE)
                    head {
                        link rel="stylesheet" href="/public/css/app.css";
                    }
                    h1 class="underline text-lg" { "Maud + Axum + Tailwindcss + HTMx" }
                };
            }),
        );

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
