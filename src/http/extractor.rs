use std::collections::HashMap;

use super::error::{ApplicationError, ErrorBag, RenderErrorsAsHtml};
use async_trait::async_trait;
use axum::{
    extract::{rejection::FormRejection, FromRequest},
    http::Request,
    Form,
};
use serde::de::DeserializeOwned;
use validator::{Validate, ValidationErrorsKind};

#[derive(Debug)]
pub(super) struct ValidatedForm<T>(pub(super) T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for ValidatedForm<T>
where
    S: Send + Sync,
    B: Send + 'static,
    T: DeserializeOwned + Validate + RenderErrorsAsHtml,
    Form<T>: FromRequest<S, B, Rejection = FormRejection>,
{
    type Rejection = ApplicationError;

    async fn from_request(request: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        return match Form::<T>::from_request(request, state).await {
            Err(err) => Err(ApplicationError::AxumFormRejection(err)),
            Ok(Form(value)) => match value.validate() {
                Err(err) => {
                    let mut errors: ErrorBag = HashMap::new();

                    for (name, error) in err.errors() {
                        match &error {
                            &ValidationErrorsKind::Field(error) => errors.insert(
                                name.to_string(),
                                error
                                    .iter()
                                    .map(|e| e.message.as_ref().unwrap().clone().into_owned())
                                    .collect(),
                            ),
                            _ => unimplemented!(),
                        };
                    }

                    Err(ApplicationError::ValidationError(Some(
                        value.render(&errors),
                    )))
                }
                Ok(_) => Ok(ValidatedForm(value)),
            },
        };
    }
}
