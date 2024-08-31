use deadpool_diesel::postgres::BuildError;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;

/// A type alias for the asynchronous PostgreSQL connection pool.
pub type PgPool = Pool<AsyncPgConnection>;

/// Create a deadpool connection pool from the given URL.
/// Using the default pool size and other settings.
pub fn create_pool_from_url(url: impl Into<String>) -> Result<PgPool, BuildError> {
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url);
    Pool::builder(config).build()
}

/// Create a deadpool connection pool from the given URL.
/// Using the given pool size and other default settings.
pub fn create_pool_from_url_with_size(
    url: impl Into<String>,
    size: usize,
) -> Result<PgPool, BuildError> {
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url);
    Pool::builder(config).max_size(size).build()
}
