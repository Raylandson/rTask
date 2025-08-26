mod db;
mod models;
mod schema;

use crate::{
    db::{create_pool, DbPool},
    models::*,
};
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use diesel::prelude::*;
use serde::Deserialize;
use std::env;
// use diesel_demo::*;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("database url must be set");
    let pool = create_pool(&database_url);

    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/users", get(get_users))
        .route("/users", post(create_user))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
    println!("Hello, world!");
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
    password: String,
}

async fn create_user(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<models::User>, (StatusCode, String)> {
    use crate::schema::users::dsl::*;
    let mut conn = pool.get().expect("error gettin connection");
    let new_user = NewUser {
        username: &payload.username,
        password: &payload.password,
    };

    let result = diesel::insert_into(users)
        .values(new_user)
        .returning(User::as_returning())
        .get_result(&mut conn);
    match result {
        Ok(res) => Ok(Json(res)),
        Err(diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            _,
        )) => Err((StatusCode::CONFLICT, "Username already used".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {e}"))),
    }
}

#[axum::debug_handler]
async fn get_users(State(pool): State<DbPool>) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    use crate::schema::users::dsl::*;
    let mut conn = pool.get().expect("error gettin connection");

    let results = users
        .limit(4)
        .select(User::as_select())
        .load::<User>(&mut conn);
    match results {
        Ok(res) => Ok(Json(res)),
        Err(diesel::result::Error::DatabaseError(_, _)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, format!("DB error")))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("server error: {e}"),
        )),
    }
}
