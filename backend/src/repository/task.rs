use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};

use crate::db::DbPool;
use crate::models::task::{NewTask, Task};
use crate::repository::DbError;
use crate::schema::tasks::dsl::tasks;
use crate::service::task::CreateTask;

pub fn get_all_tasks(pool: DbPool) -> Result<Vec<Task>, DbError> {
    let mut conn = pool
        .get()
        .map_err(|e| DbError::DbConnError(e.to_string()))?;

    match tasks.select(Task::as_select()).load::<Task>(&mut conn) {
        Ok(result) => Ok(result),
        Err(err) => Err(DbError::DbConnError(err.to_string())),
    }
}

pub fn create_task(task_info: CreateTask, pool: DbPool) -> Result<Task, DbError> {
    let mut conn = pool
        .get()
        .map_err(|e| DbError::DbConnError(e.to_string()))?;

    let new_task = NewTask {
        user_id: task_info.user_id,
        title: &task_info.title,
        description: task_info.description.as_deref(),
        completed: task_info.completed,
    };

    let result = diesel::insert_into(tasks)
        .values(new_task)
        .returning(Task::as_returning())
        .get_result(&mut conn);

    match result {
        Ok(res) => Ok(res),
        Err(diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            _,
        )) => Err(DbError::UniqueViolation),
        Err(diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::NotNullViolation,
            _,
        )) => Err(DbError::InvalidField),
        Err(e) => Err(DbError::DbConnError(e.to_string())),
    }
}
