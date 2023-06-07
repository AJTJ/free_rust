// NOTE: a layer between the database schema and the graphql_schema

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

#[derive(Queryable, Insertable, SimpleObject)]
#[table_name = "users"]
pub struct UserCreationData {
    pub username: String,
    pub user_id: Uuid,
    pub hashed_password: String,
    pub password_salt: Vec<u8>,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_active: bool,
}

// This one needs to match 1:1
#[derive(Queryable, SimpleObject, Debug)]
// #[graphql(complex)]
pub struct UserQueryData {
    pub id: i32,
    pub user_id: Uuid,
    pub username: String,
    pub hashed_password: String,
    pub password_salt: Vec<u8>,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_active: bool,
    pub deleted_at: Option<NaiveDateTime>,
    pub deleted_by: Option<Uuid>,
}

#[ComplexObject]
impl UserQueryData {
    async fn dive_sessions(
        &self,
        ctx: &Context<'_>,
        db_query_dto: DBQueryObject,
        mut dive_session_query: DiveSessionQueryInput,
    ) -> FieldResult<Vec<DiveSessionQueryData>> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

        dive_session_query.user_id = self.user_id;

        let dive_sessions = web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_dive_sessions_by_user(&mut conn, dive_session_query, db_query_dto)
        })
        .await
        .expect("error in dive sessions web::block")
        .expect("error in another loading dive sessions");

        Ok(dive_sessions)
    }
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
