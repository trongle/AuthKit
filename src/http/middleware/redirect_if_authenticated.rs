use axum::{body::Body, http::Request, response::Response};
use tower::Service;

#[derive(Clone)]
struct RedirectIfAuthenticated<S> {
    next: S,
}

impl<S> Service<Request<Body>> for RedirectIfAuthenticated<S>
where
    S: Service<Request<Body>, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        return self.next.poll_ready(cx);
    }

    fn call(&mut self, request: Request<Body>) -> Self::Future {
        return self.next.call(request);
    }
}
