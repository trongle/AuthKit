use std::collections::HashMap;

use axum::{extract::rejection::FormRejection, response::IntoResponse};
use maud::{Markup, PreEscaped};

pub type ErrorBag = HashMap<String, Vec<String>>;

#[derive(Debug)]
pub enum ApplicationError {
    ValidationError(Option<Markup>),
    AxumFormRejection(FormRejection),
    ServerError(String),
}

pub(super) trait RenderErrorsAsHtml {
    fn render(&self, errs: &ErrorBag) -> Markup;
}

impl IntoResponse for ApplicationError {
    fn into_response(self) -> axum::response::Response {
        return match self {
            ApplicationError::ValidationError(html) => {
                html.or(Some(PreEscaped("".to_string()))).unwrap()
            }
            ApplicationError::AxumFormRejection(_) => PreEscaped("".to_string()),
            ApplicationError::ServerError(_) => PreEscaped("".to_string()),
        }
        .into_response();
    }
}
