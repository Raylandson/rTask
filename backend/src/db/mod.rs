use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn create_pool(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("failed to create pool")
}
