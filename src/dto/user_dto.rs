use crate::{actions::get_dive_sessions_by_user, graphql_schema::DbPool, schema::users};
use actix_web::web;
use async_graphql::{ComplexObject, Context, FieldResult, InputObject, SimpleObject};
use chrono::NaiveDateTime;
use uuid::Uuid;

use super::{
    db_query_dto::{self, DBQueryObject},
    dive_session_dto::{DiveSessionQueryData, DiveSessionQueryInput},
};

#[derive(Debug, Clone, InputObject)]
pub struct UserInputData {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(AsChangeset, InputObject, Clone)]
#[diesel(table_name = users)]
pub struct UserModificationData {
    pub username: Option<String>,
    pub email: Option<String>,
    pub last_login: Option<NaiveDateTime>,
    pub is_active: Option<bool>,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct UserCreationData {
    pub username: String,
    pub id: Uuid,
    pub hashed_password: String,
    pub password_salt: Vec<u8>,
    pub email: String,
    pub last_login: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_active: bool,
}

// This one needs to match 1:1
#[derive(Queryable, SimpleObject, Debug)]
#[graphql(complex)]
pub struct UserQueryData {
    pub username: String,
    pub hashed_password: String,
    pub password_salt: Vec<u8>,
    pub email: String,
    pub last_login: NaiveDateTime,

    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_active: bool,
    pub deleted_at: Option<NaiveDateTime>,
    pub deleted_by: Option<Uuid>,
}

impl From<UserQueryData> for UserQueryDataOutput {
    fn from(val: UserQueryData) -> Self {
        UserQueryDataOutput {
            id: val.id,
            username: val.username,
            email: val.email,
            last_login: val.last_login,
            created_at: val.created_at,
            updated_at: val.updated_at,
            is_active: val.is_active,
        }
    }
}

#[ComplexObject]
impl UserQueryData {
    async fn dive_sessions(
        &self,
        ctx: &Context<'_>,
        // this needs to be mut
        mut dive_session_query: Option<DiveSessionQueryInput>,
        db_query_dto: Option<DBQueryObject>,
    ) -> FieldResult<Vec<DiveSessionQueryData>> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

        let user_id = self.id;

        let dive_sessions = web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_dive_sessions_by_user(&mut conn, &user_id, dive_session_query, db_query_dto)
        })
        .await
        .expect("error in dive sessions web::block")
        .expect("error in another loading dive sessions");

        Ok(dive_sessions)
    }
}
#[derive(SimpleObject)]
#[graphql(complex)]
pub struct UserQueryDataOutput {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub last_login: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_active: bool,
}

#[ComplexObject]
impl UserQueryDataOutput {
    async fn dive_sessions(
        &self,
        ctx: &Context<'_>,
        // this needs to be mut
        dive_session_query: Option<DiveSessionQueryInput>,
        db_query_dto: Option<DBQueryObject>,
    ) -> FieldResult<Vec<DiveSessionQueryData>> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

        let user_id = self.id;
        let dive_sessions = web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_dive_sessions_by_user(&mut conn, &user_id, dive_session_query, db_query_dto)
        })
        .await
        .expect("error in dive sessions web::block")
        .expect("error in another loading dive sessions");

        Ok(dive_sessions)
    }
}
