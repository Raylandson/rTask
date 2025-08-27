pub mod task;
pub mod user;

pub enum DbError {
    InvalidField,
    UniqueViolation,
    DbConnError(String),
}
