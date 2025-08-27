use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};

use crate::db::DbPool;
use crate::models::task::Task;
use crate::repository::DbError;
// use crate::schema::tasks;
use crate::schema::tasks::dsl::tasks;

pub fn get_all_tasks(pool: DbPool) -> Result<Vec<Task>, DbError> {
    let mut conn = pool
        .get()
        .map_err(|e| DbError::DbConnError(e.to_string()))?;

    match tasks.select(Task::as_select()).load::<Task>(&mut conn) {
        Ok(result) => Ok(result),
        Err(err) => Err(DbError::DbConnError(err.to_string())),
    }
}
