use crate::schema::tasks;
use diesel::{
    prelude::{Insertable, Queryable},
    Selectable,
};
use serde::{Deserialize, Serialize};

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
#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = tasks)]
pub struct NewTask<'a> {
    pub user_id: i32,
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub completed: Option<bool>,
}
