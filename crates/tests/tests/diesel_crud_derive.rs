use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use lib::diesel_crud_derive::{
    DieselCrudCreate, DieselCrudDelete, DieselCrudList, DieselCrudRead, DieselCrudUpdate,
};
use lib::diesel_crud_trait::DieselCrudCreate;
use test_containers::create_test_containers_pool;

#[cfg(test)]
pub mod test_containers;

diesel::table! {
    user (email) {
        #[max_length = 255]
        email -> Varchar,
    }
}

#[derive(
    Debug,
    PartialEq,
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
    DieselCrudCreate,
    DieselCrudDelete,
    DieselCrudList,
    DieselCrudRead,
    DieselCrudUpdate,
)]
#[diesel_crud(insert = InsertUser)]
#[diesel(table_name = user)]
struct User {
    #[diesel_crud(pk)]
    email: String,
}

#[derive(Clone, Insertable)]
#[diesel(table_name = user)]
struct InsertUser {
    email: String,
}

#[tokio::test]
async fn test_insert_user() {
    let container = create_test_containers_pool().await.unwrap();
    let mut conn = container.pool.get().await.unwrap();
    let user = User::insert(
        InsertUser {
            email: "test".to_string(),
        },
        &mut conn,
    )
    .await;
    assert_eq!(
        user,
        Ok(User {
            email: "test".to_string()
        })
    );
}
