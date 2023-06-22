use crate::{
    actions::get_dive_sessions_by_user, errors::BigError, graphql_schema::DbPool, schema::users,
};
use actix_web::web;
use async_graphql::{ComplexObject, Context, FieldResult, InputObject, SimpleObject};
use chrono::NaiveDateTime;
use uuid::Uuid;

use super::{
    dive_session_dto::{DiveSession, DiveSessionFilter},
    query_dto::{self, QueryParams},
};

#[derive(Clone, InputObject)]
pub struct UserInput {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(AsChangeset, InputObject, Clone)]
#[diesel(table_name = users)]
pub struct UserUpdate {
    pub username: Option<String>,
    pub email: Option<String>,
    pub last_login: Option<NaiveDateTime>,
    pub is_active: Option<bool>,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct UserCreation {
    pub username: String,
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
pub struct User {
    pub username: String,
    pub hashed_password: String,
    pub password_salt: Vec<u8>,
    pub email: String,
    pub last_login: NaiveDateTime,

    // default data
    pub id: Uuid,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_active: bool,
    #[graphql(skip)]
    pub archived_at: Option<NaiveDateTime>,
    #[graphql(skip)]
    pub archived_by: Option<Uuid>,
}

#[ComplexObject]
impl User {
    async fn dive_sessions(
        &self,
        ctx: &Context<'_>,
        // this needs to be mut
        dive_session_query: Option<DiveSessionFilter>,
        db_query_dto: Option<QueryParams>,
        // ) -> FieldResult<Vec<DiveSession>> {
    ) -> Result<Vec<DiveSession>, BigError> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

        let user_id = self.id;
        let dive_sessions = web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_dive_sessions_by_user(&mut conn, &user_id, dive_session_query, db_query_dto)
                .map(|dv| dv.into_iter().map(DiveSession::from).collect())
        })
        .await
        .expect("error in dive sessions web::block")
        .expect("error in another loading dive sessions");

        Ok(dive_sessions)
    }
}

// #[derive(SimpleObject)]
// #[graphql(complex)]
// pub struct UserOutput {
//     pub username: String,
//     pub email: String,
//     pub last_login: NaiveDateTime,

//     pub id: ID,
//     pub created_at: NaiveDateTime,
//     pub updated_at: NaiveDateTime,
//     pub is_active: bool,
// }

// impl From<User> for UserOutput {
//     fn from(val: User) -> Self {
//         UserOutput {
//             id: val.id.into(),
//             username: val.username,
//             email: val.email,
//             last_login: val.last_login,
//             created_at: val.created_at,
//             updated_at: val.updated_at,
//             is_active: val.is_active,
//         }
//     }
// }

// #[ComplexObject]
// impl UserOutput {
//     async fn dive_sessions(
//         &self,
//         ctx: &Context<'_>,
//         // this needs to be mut
//         dive_session_query: Option<DiveSessionFilter>,
//         db_query_dto: Option<QueryParams>,
//         // ) -> FieldResult<Vec<DiveSession>> {
//     ) -> Result<Vec<DiveSessionOutput>, BigError> {
//         let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

//         let user_id: Uuid =
//             Uuid::parse_str(&self.id).map_err(|e| BigError::UuidParsingerror { source: e })?;
//         let dive_sessions = web::block(move || {
//             let mut conn = pool_ctx.get().unwrap();
//             get_dive_sessions_by_user(&mut conn, &user_id, dive_session_query, db_query_dto)
//                 .map(|dv| dv.into_iter().map(DiveSessionOutput::from).collect())
//         })
//         .await
//         .expect("error in dive sessions web::block")
//         .expect("error in another loading dive sessions");

//         Ok(dive_sessions)
//     }
// }
