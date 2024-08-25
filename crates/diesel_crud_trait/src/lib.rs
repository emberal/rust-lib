mod error;

use async_trait::async_trait;
use diesel::{AsChangeset, Insertable};
use diesel_async::AsyncPgConnection;
pub use error::CrudError;

/// Combines all CRUD operations into a single trait
/// Includes:
/// - Create
/// - Read
/// - Update
/// - Delete
/// - List
pub trait DieselCrud<Table>:
    DieselCrudCreate<Table> + DieselCrudRead + DieselCrudUpdate + DieselCrudDelete + DieselCrudList
{
}

/// Insert an entity into the database
/// The entity must implement `Insertable<Table>` for the given table.
///
/// Implementing the trait requires the `async_trait` macro.
/// # Associations
/// - `Insert` - The type to insert, must implement `Insertable<Table>`
/// # Parameters
/// - `insert` - The entity to insert
/// - `conn` - The database connection
/// # Returns
/// A result containing the inserted entity or a `CrudError`
#[async_trait]
pub trait DieselCrudCreate<Table>
where
    Self: Sized,
{
    type Insert: Insertable<Table>;
    async fn insert(insert: Self::Insert, conn: &mut AsyncPgConnection) -> Result<Self, CrudError>;
    async fn insert_many(
        insert: &[Self::Insert],
        conn: &mut AsyncPgConnection,
    ) -> Result<Vec<Self>, CrudError>;
}

/// Gets an entity from the database
///
/// Implementing the trait requires the `async_trait` macro.
/// # Associations
/// - `PK` - The primary key of the entity
/// # Parameters
/// - `pk` - The primary key of the entity
/// - `conn` - The database connection
/// # Returns
/// A result containing the entity or a `CrudError`.
/// If the entity is not found, the error should be `CrudError::NotFound`.
#[async_trait]
pub trait DieselCrudRead
where
    Self: Sized,
{
    type PK;
    async fn read(pk: Self::PK, conn: &mut AsyncPgConnection) -> Result<Self, CrudError>;
}

/// Updates an entity in the database
/// The entity must implement `AsChangeset` for the given table.
///
/// Implementing the trait requires the `async_trait` macro.
/// # Associations
/// - `Update` - The type to update
/// # Parameters
/// - `update` - The update to apply
/// - `conn` - The database connection
/// # Returns
/// A result containing the old entry of the entity if successful or a `CrudError`.
/// If the entity is not found, the error should be `CrudError::NotFound`.
#[async_trait]
pub trait DieselCrudUpdate
where
    Self: Sized,
{
    type Update: AsChangeset;
    async fn update(update: Self::Update, conn: &mut AsyncPgConnection) -> Result<Self, CrudError>;
}

/// Deletes an entity from the database
///
/// Implementing the trait requires the `async_trait` macro.
/// # Associations
/// - `PK` - The primary key of the entity
/// # Parameters
/// - `pk` - The primary key of the entity
/// - `conn` - The database connection
/// # Returns
/// A result containing the deleted entity or a `CrudError`.
/// If the entity is not found, the error should be `CrudError::NotFound`.
#[async_trait]
pub trait DieselCrudDelete
where
    Self: Sized,
{
    type PK;
    async fn delete(pk: Self::PK, conn: &mut AsyncPgConnection) -> Result<Self, CrudError>;
}

/// Lists all entities in the table
///
/// Implementing the trait requires the `async_trait` macro.
/// # Parameters
/// - `conn` - The database connection
/// # Returns
/// A result containing a Vec of entities or a `CrudError`.
#[async_trait]
pub trait DieselCrudList
where
    Self: Sized,
{
    async fn list(conn: &mut AsyncPgConnection) -> Result<Vec<Self>, CrudError>;
}
