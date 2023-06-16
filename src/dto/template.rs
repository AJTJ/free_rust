use crate::{actions::get_dive_sessions_by_user, graphql_schema::DbPool, schema::users};
use actix_web::web;
use async_graphql::{ComplexObject, Context, FieldResult, InputObject, SimpleObject};
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(AsChangeset, InputObject, Clone)]
#[diesel(table_name = users)]
pub struct UserModificationData {}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct UserCreationData {}

// This one needs to match 1:1
#[derive(Queryable, SimpleObject, Debug)]
#[graphql(complex)]
pub struct UserQueryData {}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct UserQueryDataOutput {}
