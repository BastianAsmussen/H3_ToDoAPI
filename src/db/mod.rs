pub mod models;
pub mod schema;

use anyhow::Result;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

/// Create a new connection pool for the application.
///
/// # Returns
///
/// * `Result<Pool<ConnectionManager<PgConnection>>>` - The connection pool if successful.
///
/// # Errors
///
/// * If the `DATABASE_URL` environment variable is not set.
/// * If the connection pool cannot be created.
pub fn new_connection_pool() -> Result<Pool<ConnectionManager<PgConnection>>> {
    let database_url = dotenvy::var("DATABASE_URL")?;

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::new(manager)?;

    Ok(pool)
}
