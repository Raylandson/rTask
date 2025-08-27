use crate::schema::tasks;
use diesel::{prelude::Queryable, Selectable};
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = tasks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Task {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
}

pub struct NewTask<'a> {
    pub user_id: i32,
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub completed: Option<bool>,
}
