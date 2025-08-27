use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};

use crate::repository::DbError;
use crate::schema::users::dsl::users;
use crate::{
    db::DbPool,
    models::user::{NewUser, User},
    service::user::CreateUser,
};

pub fn create_user(user_info: CreateUser, pool: DbPool) -> Result<User, DbError> {
    let mut conn = pool.get().expect("error gettin connection");
    let new_user = NewUser {
        username: &user_info.username,
        password: &user_info.password,
    };

    let result = diesel::insert_into(users)
        .values(new_user)
        .returning(User::as_returning())
        .get_result(&mut conn);
    match result {
        Ok(res) => Ok(res),
        Err(diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            _,
        )) => Err(DbError::UniqueViolation),
        Err(e) => Err(DbError::DbConnError(e.to_string())),
    }
}

pub fn get_all_users(pool: DbPool) -> Result<Vec<User>, DbError> {
    let mut conn = pool
        .get()
        .map_err(|_| DbError::DbConnError("Error in db connection".to_string()))?;

    let result = users.select(User::as_select()).load::<User>(&mut conn);

    match result {
        Ok(users_vec) => Ok(users_vec),
        Err(e) => Err(DbError::DbConnError(e.to_string())),
    }
}
