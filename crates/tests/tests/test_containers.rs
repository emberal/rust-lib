use derive_more::{Constructor, From};
use diesel_async::pooled_connection::deadpool::{BuildError, PoolError};
use diesel_async::AsyncPgConnection;
use diesel_async_migrations::EmbeddedMigrations;
use lib::diesel::pool::{create_pool_from_url, PgPool};
use lib::diesel::DieselError;
use testcontainers_modules::postgres::Postgres;
use testcontainers_modules::testcontainers::runners::AsyncRunner;
use testcontainers_modules::testcontainers::{ContainerAsync, TestcontainersError};

/// When the TestContainer is dropped, the container will be removed.
/// # Errors
/// If destructed and the container field is dropped, the container will be stopped
/// and all connections from the pool will result in DatabaseError.
#[derive(Constructor)]
pub struct TestContainer {
    pub container: ContainerAsync<Postgres>,
    pub pool: PgPool,
}

#[derive(Debug, From)]
pub enum ContainerError {
    TestContainers(TestcontainersError),
    BuildError(BuildError),
    PoolError(PoolError),
    DieselError(DieselError),
}

pub async fn create_test_containers_pool<'a>() -> Result<TestContainer, ContainerError> {
    let container = create_postgres_container().await?;
    let connection_string = format!(
        "postgres://postgres:postgres@localhost:{}/postgres",
        container.get_host_port_ipv4(5432).await?
    );
    let pool = create_pool_from_url(connection_string)?;
    run_migrations(pool.get().await?.as_mut()).await?;
    Ok(TestContainer::new(container, pool))
}

pub(crate) async fn run_migrations(
    conn: &mut AsyncPgConnection,
) -> Result<(), diesel::result::Error> {
    static EMBEDDED_MIGRATIONS: EmbeddedMigrations =
        diesel_async_migrations::embed_migrations!("./migrations");
    EMBEDDED_MIGRATIONS.run_pending_migrations(conn).await
}

pub async fn create_postgres_container() -> Result<ContainerAsync<Postgres>, TestcontainersError> {
    Postgres::default().start().await
}
