use axum::{extract::rejection::FormRejection, response::IntoResponse};
use maud::{Markup, PreEscaped};

#[derive(Debug)]
pub enum ApplicationError<T> {
    ValidationError(validator::ValidationErrors, T),
    AxumFormRejection(FormRejection),
}

pub(super) trait RenderErrorsAsHtml {
    fn render(&self, errs: validator::ValidationErrors) -> Markup;
}

impl<T: RenderErrorsAsHtml> IntoResponse for ApplicationError<T> {
    fn into_response(self) -> axum::response::Response {
        return match self {
            ApplicationError::ValidationError(errs, html_renderer) => html_renderer.render(errs),
            ApplicationError::AxumFormRejection(_) => PreEscaped("".to_string()),
        }
        .into_response();
    }
}
