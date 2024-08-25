use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use diesel_async::{AsyncConnection, AsyncPgConnection};
use dotenvy_macro::dotenv;
use lib::diesel_crud_derive::{
    DieselCrudCreate, DieselCrudDelete, DieselCrudList, DieselCrudRead, DieselCrudUpdate,
};
use lib::diesel_crud_trait::DieselCrudCreate;

diesel::table! {
    user (email) {
        #[max_length = 255]
        email -> Varchar,
    }
}

#[derive(
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
    let database_url = dotenv!("DATABASE_URL");
    let mut conn = AsyncPgConnection::establish(database_url).await.unwrap();
    conn.begin_test_transaction().await.unwrap();
    let _user = User::insert(
        InsertUser {
            email: "test".to_string(),
        },
        &mut conn,
    )
    .await;
}
