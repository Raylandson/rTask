use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;

use crate::{
    db::DbPool,
    models::task::Task,
    repository::{self, task::get_all_tasks, DbError},
};

#[derive(Deserialize)]
pub struct CreateTask {
    pub title: String,
    pub description: Option<String>,
    pub user_id: i32,
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

pub async fn create_task(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateTask>,
) -> Result<Json<Task>, (StatusCode, String)> {
    match repository::task::create_task(payload, pool) {
        Ok(res) => Ok(Json(res)),
        Err(DbError::InvalidField) => Err((StatusCode::BAD_REQUEST, "Invalid fields".to_string())),
        Err(DbError::DbConnError(e)) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        Err(DbError::UniqueViolation) => Err((StatusCode::CONFLICT, "aiai".to_string())),
    }
}
