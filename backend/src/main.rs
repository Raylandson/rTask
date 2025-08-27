mod db;
mod models;
mod repository;
mod routes;
mod schema;
mod service;

use crate::db::{create_pool, DbPool};
use crate::routes::task::task_routes;
use crate::routes::user::user_routes;
use axum::{routing::get, Router};
use std::env;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_url: String = env::var("DATABASE_URL").expect("database url must be set");
    let pool: DbPool = create_pool(&database_url);

    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .merge(user_routes(pool.clone()))
        .merge(task_routes(pool.clone()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
    println!("Hello, world!");
}
