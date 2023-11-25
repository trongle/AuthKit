use crate::AppContext;
use axum::{body::Body, http::Request, response::Response};
use axum_session::{Session, SessionRedisPool};
use sqlx::MySqlPool;
use tower::{Layer, Service as TowerService};

#[derive(Clone, Debug)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
}

#[derive(Clone)]
pub struct Auth {
    user: Option<User>,
    session: Session<SessionRedisPool>,
    db: MySqlPool,
}

#[derive(Clone)]
pub struct AuthLayer {
    app: AppContext,
}

impl<Next> Layer<Next> for AuthLayer {
    type Service = Service<Next>;

    fn layer(&self, next: Next) -> Self::Service {
        return Service {
            next,
            user: None,
            app: self.app.clone(),
        };
    }
}

impl AuthLayer {
    pub fn new(app: AppContext) -> Self {
        return Self { app };
    }
}

impl Auth {
    pub async fn get_user(&mut self) -> Option<User> {
        if let Some(user) = self.user.as_ref() {
            println!("from cache");
            return Some(user.clone());
        }

        if let Some(user_id) = self.session.get::<i32>("user_id") {
            let result = sqlx::query!(
                "select id, username, email from users where id = ?",
                user_id
            )
            .fetch_optional(&self.db)
            .await;

            if let Ok(record) = result {
                let user = User {
                    id: record.as_ref().unwrap().id,
                    username: record.as_ref().unwrap().username.clone(),
                    email: record.as_ref().unwrap().email.clone(),
                };

                self.user = Some(user);

                println!("User: {:?}", self.user);

                return self.user.clone();
            } else if let Err(e) = result {
                panic!("Error getting user: {}", e);
            }
        }

        return None;
    }
}

#[derive(Clone)]
pub struct Service<Next> {
    next: Next,
    pub user: Option<User>,
    app: AppContext,
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

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        let auth = Auth {
            session: req
                .extensions()
                .get::<Session<SessionRedisPool>>()
                .unwrap()
                .clone(),
            db: self.app.db.clone(),
            user: None,
        };

        req.extensions_mut().insert(auth);

        return self.next.call(req);
    }
}
