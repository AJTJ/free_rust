use crate::{actions::get_dive_sessions_by_user, graphql_schema::DbPool, schema::users};
use actix_web::web;
use async_graphql::{ComplexObject, Context, FieldResult, InputObject, SimpleObject};
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(InputObject)]
pub struct FooInput {}

#[derive(AsChangeset, InputObject)]
#[diesel(table_name = users)]
pub struct FooUpdate {}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct FooCreation {
    // partial default data
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_active: bool,
}

// This one needs to match 1:1
#[derive(Queryable, SimpleObject)]
#[graphql(complex)]
pub struct Foo {
    // default data
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_active: bool,
    pub deleted_at: Option<NaiveDateTime>,
    pub deleted_by: Option<Uuid>,
}

#[derive(InputObject)]
pub struct FooQueryParams {}

// not sure this one is necessary
#[derive(SimpleObject)]
#[graphql(complex)]
pub struct FooOutput {}
