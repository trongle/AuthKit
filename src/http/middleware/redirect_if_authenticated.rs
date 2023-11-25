use axum::{body::Body, http::Request, response::Response};
use axum_session::SessionRedisPool;
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
    type Future = S::Future;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        return self.next.poll_ready(cx);
    }

    fn call(&mut self, request: Request<Body>) -> Self::Future {
        println!("RedirectIfAuthenticated");

        if request.uri().path() != "/login" && request.uri().path() != "/register" {
            return self.next.call(request);
        }

        // Box::pin(async move {
        // if let Some(session) = request
        //     .extensions()
        //     .get::<axum_session::Session<SessionRedisPool>>()
        // {
        //     if session.get::<i32>("user_id").is_some() {
        //         return Response::builder()
        //             .status(302)
        //             .header("Location", "/home")
        //             .body(Body::empty())
        //             .unwrap();
        //     }
        // }
        // });
        return self.next.call(request);
    }
}
