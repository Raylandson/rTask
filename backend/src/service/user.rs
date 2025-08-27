use crate::repository::user::DbError;
use crate::{db::DbPool, models::*};
use crate::{models, repository};
use axum::routing::get;
use axum::Router;
use axum::{extract::State, http::StatusCode, response::Json};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
}

pub fn user_routes(pool: DbPool) -> Router {
    Router::new()
        .route("/users", get(get_users).post(create_user))
        .with_state(pool)
}

pub async fn create_user(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<models::User>, (StatusCode, String)> {
    let result = repository::user::create_user(payload, pool);
    match result {
        Ok(user) => Ok(Json(user)),
        Err(DbError::InvalidField) => Err((StatusCode::BAD_REQUEST, "Invalid fields".to_string())),
        Err(DbError::UniqueViolation) => {
            Err((StatusCode::CONFLICT, "Username already used".to_string()))
        }
        Err(DbError::DbConnError(e)) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}

#[axum::debug_handler]
pub async fn get_users(
    State(pool): State<DbPool>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    match repository::user::get_all_users(pool) {
        Ok(res) => Ok(Json(res)),
        Err(DbError::DbConnError(error)) => Err((StatusCode::INTERNAL_SERVER_ERROR, error)),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal error".to_string(),
        )),
    }
}
