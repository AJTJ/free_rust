// NOTE: a layer between the database schema and the graphql_schema

use crate::schema::users;
use async_graphql::{InputObject, SimpleObject};
use chrono::NaiveDateTime;
use diesel::types::Binary;
use uuid::Uuid;

#[derive(Debug, Clone, InputObject)]
pub struct UserInputData {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Queryable, Insertable, SimpleObject)]
#[table_name = "users"]
pub struct UserCreationData {
    pub username: String,
    pub user_id: Uuid,
    pub hashed_password: String,
    pub password_salt: Binary,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// This one needs to match 1:1
#[derive(Queryable, SimpleObject)]
pub struct UserQueryData {
    pub id: i32,
    pub user_id: Uuid,
    pub username: String,
    pub hashed_password: String,
    pub password_salt: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// AUTH STUFF

#[derive(InputObject)]
pub struct LoginData {
    pub email: String,
    pub hashed_password: String,
}

#[derive(InputObject)]
pub struct LogoutData {
    //TODO: Should be user_id or something else?
    pub email: String,
}
