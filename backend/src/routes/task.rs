use axum::{routing::get, Router};

use crate::{
    db::DbPool,
    service::task::{create_task, get_tasks},
};

pub fn task_routes(pool: DbPool) -> Router {
    Router::new()
        .route("/tasks", get(get_tasks).post(create_task))
        .with_state(pool)
}
