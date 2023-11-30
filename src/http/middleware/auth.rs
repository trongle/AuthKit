use crate::AppContext;
use axum::{body::Body, extract::State, http::Request, middleware::Next, response::Response};
use axum_session::{Session, SessionRedisPool};
use sqlx::MySqlPool;

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

impl Auth {
    pub fn get_user(&self) -> Option<&User> {
        return self.user.as_ref();
    }

    pub async fn set_user(&mut self) {
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
            } else if let Err(e) = result {
                panic!("Error getting user: {}", e);
            }
        }
    }
}

pub async fn auth(
    State(AppContext { db }): State<AppContext>,
    mut request: Request<Body>,
    next: Next<Body>,
) -> Response {
    let mut auth = Auth {
        session: request
            .extensions()
            .get::<Session<SessionRedisPool>>()
            .unwrap()
            .clone(),
        db,
        user: None,
    };

    auth.set_user().await;

    request.extensions_mut().insert(auth);

    return next.run(request).await;
}
