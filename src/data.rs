// NOTE: a layer between the database schema and the graphql_schema

use crate::schema::users;
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};
use chrono::NaiveDateTime;
use diesel::Expression;
use uuid::Uuid;

#[derive(Debug, Clone, InputObject)]
pub struct UserInputData {
    pub username: String,
    pub hashed_password: String,
    pub email: String,
}

#[derive(Insertable, Queryable, SimpleObject)]
#[table_name = "users"]
pub struct UserCreationData {
    pub username: String,
    pub user_id: Uuid,
    pub hashed_password: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct UserOutputData {}

// impl Todos {
//   pub fn all_users(conn: &PgConnection) ->
// }
