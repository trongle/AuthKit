use hands_on_maud::http;
use sqlx::{migrate, mysql::MySqlPoolOptions};

#[tokio::main]
async fn main() {
    // Setup Mysql pool connections
    // for the applcation.
    let db = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://localhost:3306/hands_on_maud")
        .await
        .unwrap();

    // Run database migrations so we
    // can ensure the database is correctly
    // startup. If we used sqlx-cli to run
    // the migrations, then this action
    // will not re-run the migrations. So
    // it is safe.
    migrate!().run(&db).await.unwrap();

    http::server(db).await;
}

// fn layout(title: &str, body: Markup, script: Option<Markup>) -> Markup {
//     return html! {
//         (DOCTYPE)
//         html data-theme="light" {
//             (header(title))
//             body class="grid place-items-center h-[100dvh] bg-slate-100" {
//                 (body)
//                 (footer())
//                 (if let Some(s) = script { s } else { PreEscaped("".to_string()) })
//             }
//         }
//     };
// }
//
// fn header(title: &str) -> Markup {
//     return html! {
//         head {
//             meta charset="utf-8";
//             meta name="viewport" content="width=device-width";
//             title {(title)}
//             link rel="stylesheet" href="/public/css/app.css";
//         }
//     };
// }
//
// fn footer() -> Markup {
//     return html! {
//         footer {}
//     };
// }
