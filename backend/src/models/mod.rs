use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::users;

// pub enum DbError {
//     #[error("Database error: {0}")]
//     DbError(#[from] diesel::result::Error),
// }

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]

pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name=users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
}
