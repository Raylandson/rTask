use axum::{routing::get, Router};

use crate::{
    db::DbPool,
    service::user::{create_user, get_users},
};

pub fn user_routes(pool: DbPool) -> Router {
    Router::new()
        .route("/users", get(get_users).post(create_user))
        .with_state(pool)
}
