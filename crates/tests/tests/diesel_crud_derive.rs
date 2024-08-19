#![allow(unused)]
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use lib::diesel_crud_derive::{
    DieselCrud, DieselCrudCreate, DieselCrudDelete, DieselCrudList, DieselCrudRead,
    DieselCrudUpdate,
};
use lib::diesel_crud_trait::DieselCrudCreate;

diesel::table! {
    user (email) {
        #[max_length = 255]
        email -> Varchar,
    }
}

#[derive(Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = user)]
struct User {
    email: String,
}

#[derive(Insertable)]
#[diesel(table_name = user)]
struct InsertUser {
    email: String,
}

#[derive(DieselCrud)]
#[diesel_crud(table = user, entity = User, pk = String, pk_field = email, create = InsertUser, update = User)]
struct TestService {
    pool: Pool<AsyncPgConnection>,
}

#[derive(DieselCrudCreate, DieselCrudRead, DieselCrudUpdate, DieselCrudDelete, DieselCrudList)]
#[diesel_crud(table = user, entity = User, pk = String, pk_field = email, create = InsertUser, update = User)]
struct TestServiceSeparate {
    pool: Pool<AsyncPgConnection>,
}

#[derive(DieselCrudCreate)]
#[diesel_crud(table = user, entity = User, create = InsertUser)]
struct TestServiceCreate {
    pool: Pool<AsyncPgConnection>,
}

#[derive(DieselCrudRead)]
#[diesel_crud(table = user, entity = User, pk = String)]
struct TestServiceRead {
    pool: Pool<AsyncPgConnection>,
}

#[derive(DieselCrudUpdate)]
#[diesel_crud(table = user, update = User)]
struct TestServiceUpdate {
    pool: Pool<AsyncPgConnection>,
}

#[derive(DieselCrudDelete)]
#[diesel_crud(table = user, pk = String, pk_field = email)]
struct TestServiceDelete {
    pool: Pool<AsyncPgConnection>,
}

#[derive(DieselCrudList)]
#[diesel_crud(table = user, entity = User)]
struct TestServiceList {
    pool: Pool<AsyncPgConnection>,
}

#[test]
fn test_insert_user() {
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new("");
    let pool = Pool::builder(config).max_size(10).build().unwrap();

    let service = TestServiceCreate { pool };
    service.create(&InsertUser {
        email: "test".to_string(),
    });
}
