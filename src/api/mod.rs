use anyhow::Result;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;

pub mod routes;

/// The application state.
///
/// # Fields
///
/// * `pool` - The connection pool for the database.
#[derive(Debug)]
pub struct AppState {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl AppState {
    /// Create a new `AppState`.
    ///
    /// # Arguments
    ///
    /// * `pool` - The connection pool for the database.
    ///
    /// # Returns
    ///
    /// * `AppState` - The application state.
    pub const fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }

    /// Get a connection from the pool.
    ///
    /// # Returns
    ///
    /// * `Result<AsyncPgConnection>` - The connection to the database.
    ///
    /// # Errors
    ///
    /// * If there is an error getting a connection from the pool.
    pub fn connection(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>> {
        Ok(self.pool.get()?)
    }
}
