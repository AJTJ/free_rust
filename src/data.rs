// NOTE: a layer between the database schema and the graphql_schema

use crate::schema::users;
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, InputObject)]
pub struct UserInputData {
    pub username: String,
    pub hashed_password: String,
    pub email: String,
}

#[derive(Queryable, Insertable, SimpleObject)]
#[table_name = "users"]
pub struct UserCreationData {
    pub username: String,
    pub user_id: Uuid,
    pub hashed_password: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, SimpleObject)]
pub struct UserQueryData {
    pub id: i32,
    pub user_id: Uuid,
    pub username: String,
    pub hashed_password: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// LOGIN STUFF

#[derive(InputObject)]
pub struct LoginData {
    pub email: String,
    pub hashed_password: String,
}
