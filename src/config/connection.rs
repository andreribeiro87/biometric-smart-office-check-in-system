use diesel::r2d2::{ConnectionManager, Pool};
use diesel::pg::PgConnection;
pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub fn init_pool(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .max_size(5)
        .build(manager)
        .expect("Failed to create pool")
}
