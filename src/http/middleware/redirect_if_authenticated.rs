use std::{error::Error, future::Future, pin::Pin};

use super::Auth;
use axum::{body::Body, http::Request, response::Response};
use tower::{Layer, Service as TowerService};

#[derive(Clone)]
pub struct RedirectIfAuthenticated {}

impl<S> Layer<S> for RedirectIfAuthenticated {
    type Service = Service<S>;

    fn layer(&self, service: S) -> Self::Service {
        return Service { next: service };
    }
}

impl RedirectIfAuthenticated {
    pub fn new() -> Self {
        return Self {};
    }
}

#[derive(Clone)]
pub struct Service<S> {
    next: S,
}

impl<S> TowerService<Request<Body>> for Service<S>
where
    S: TowerService<Request<Body>, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<S::Response, String>>>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        return self.next.poll_ready(cx);
    }

    fn call(&mut self, request: Request<Body>) -> Self::Future {
        return Box::pin(async move {
            let auth = request.extensions().get::<Auth>().unwrap();

            if auth.get_user().is_some() && ["/login", "/register"].contains(&request.uri().path())
            {
                return Err("".to_string());
            }

            match self.next.call(request).await {
                Ok(Ok(response)) => return Ok(response),
                Ok(Err(error)) => return Err(error),
                Err(e) => return Err("".to_string()),
            }
        });
    }
}
