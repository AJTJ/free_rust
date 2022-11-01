// use diesel::pg::PgConnection;

// use super::schema::{dive_sessions, dives, users};

// a layer between the database schema and the graphql_schema

use crate::schema::users;
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};
use chrono::NaiveDateTime;
use diesel::Expression;
// use diesel::sql_types::Integer;
use uuid::Uuid;

#[derive(Debug, Clone, InputObject)]
pub struct UserInputData {
    pub username: String,
    pub hashed_password: String,
    pub email: String,
}

#[derive(Insertable, Queryable)]
#[table_name = "users"]
pub struct UserCreationData {
    pub username: String,
    pub hashed_password: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// impl Todos {
//   pub fn all_users(conn: &PgConnection) ->
// }
