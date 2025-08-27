use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;

use crate::{db::DbPool, models::task::Task, repository::task::get_all_tasks};

#[derive(Deserialize)]
pub struct CreateTask {
    pub title: String,
    pub description: Option<String>,
    pub user_id: u32,
    pub completed: Option<bool>,
}

pub async fn get_tasks(
    State(pool): State<DbPool>,
) -> Result<Json<Vec<Task>>, (StatusCode, String)> {
    match get_all_tasks(pool) {
        Ok(result) => Ok(Json(result)),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "db internal error".to_string(),
        )),
    }
}
