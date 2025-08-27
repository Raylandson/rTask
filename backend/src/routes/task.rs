use axum::{routing::get, Router};

use crate::{db::DbPool, service::task::get_tasks};

pub fn task_routes(pool: DbPool) -> Router {
    Router::new()
        .route("/tasks", get(get_tasks))
        .with_state(pool)
}
