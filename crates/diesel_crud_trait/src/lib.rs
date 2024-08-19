mod error;

pub use error::CrudError;

use async_trait::async_trait;
use diesel::{AsChangeset, Insertable};

pub trait DieselCrud<'insertable, 'entity, Table>:
    DieselCrudCreate<'insertable, 'entity, Table>
    + DieselCrudRead
    + DieselCrudUpdate
    + DieselCrudDelete
    + DieselCrudList
where
    'entity: 'insertable,
{
}

/// Insert an entity into the database
/// The entity must implement `Insertable<Table>` for the given table.
///
/// Implementing the trait requires the `async_trait` macro.
/// # Associations
/// - `Create` - The type to insert
/// - `Entity` - The type that will be returned
/// # Parameters
/// - `create` - The entity to insert
/// # Returns
/// A result containing the inserted entity or a `CrudError`
#[async_trait]
pub trait DieselCrudCreate<'insertable, 'entity, Table>
where
    'entity: 'insertable,
{
    type Create: Insertable<Table>;
    type Entity: 'entity;
    async fn create(&self, create: &'insertable Self::Create) -> Result<Self::Entity, CrudError>;
}

/// Gets an entity from the database
///
/// Implementing the trait requires the `async_trait` macro.
/// # Associations
/// - `PK` - The primary key of the entity
/// - `Entity` - The type that will be returned
/// # Parameters
/// - `pk` - The primary key of the entity
/// # Returns
/// A result containing the entity or a `CrudError`.
/// If the entity is not found, the error should be `CrudError::NotFound`.
#[async_trait]
pub trait DieselCrudRead {
    type PK;
    type Entity;
    async fn read(&self, pk: Self::PK) -> Result<Self::Entity, CrudError>;
}

/// Updates an entity in the database
/// The entity must implement `AsChangeset` for the given table.
///
/// Implementing the trait requires the `async_trait` macro.
/// # Associations
/// - `Update` - The type to update
/// # Parameters
/// - `update` - The update to apply
/// # Returns
/// A result containing the number of rows updated or a `CrudError`.
/// If the entity is not found, the error should be `CrudError::NotFound`.
#[async_trait]
pub trait DieselCrudUpdate {
    type Update: AsChangeset;
    async fn update(&self, update: Self::Update) -> Result<usize, CrudError>;
}

/// Deletes an entity from the database
///
/// Implementing the trait requires the `async_trait` macro.
/// # Associations
/// - `PK` - The primary key of the entity
/// # Parameters
/// - `pk` - The primary key of the entity
/// # Returns
/// A result containing the number of rows deleted or a `CrudError`.
/// If the entity is not found, the error should be `CrudError::NotFound`.
#[async_trait]
pub trait DieselCrudDelete {
    type PK;
    async fn delete(&self, pk: &Self::PK) -> Result<usize, CrudError>;
}

/// Lists all entities in the table
///
/// Implementing the trait requires the `async_trait` macro.
/// # Associations
/// - `Entity` - The type that will be returned in a Vec
/// # Returns
/// A result containing a Vec of entities or a `CrudError`.
#[async_trait]
pub trait DieselCrudList {
    type Entity;
    async fn list(&self) -> Result<Vec<Self::Entity>, CrudError>;
}
