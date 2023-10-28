use maud::{html, Markup, PreEscaped, DOCTYPE};

fn layout(title: &str, body: Markup, script: Option<Markup>) -> Markup {
    return html! {
        (DOCTYPE)
        html data-theme="light" {
            (header(title))
            body class="grid place-items-center h-[100dvh] bg-blue-100" {
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
