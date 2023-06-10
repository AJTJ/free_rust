use crate::{actions::get_dives_by_session, graphql_schema::DbPool, schema::dive_sessions};
use actix_web::web;
use async_graphql::{ComplexObject, Context, FieldResult, InputObject, SimpleObject};
use chrono::NaiveDateTime;
use uuid::Uuid;

use super::{
    db_query_dto::DBQueryObject,
    dive_dto::{DiveQueryData, DiveQueryInput},
};

/*
NOTES
- People are often looking for specific things and thus they will be trying to solve specific issues by recording data.
- It is very important to be able to record everything in a shorthand, since people don't necessarily want to input all the data all the time.

- shorthand recording
- max numbers per discipline
- food
    - food time
    - coffee
    - what eaten
- previous day
    - eaten
    - drank
    - etc
- general feeling
- health
    - congestion, headache
- sleep
    - time
    - quality
    - when to when
- dive buddy
    - level of qualification
- last_exertion
    - type of exertion
- environment
    - water environment
    - people
- Data sharing
    - environment data sharing
    - user profile sharing
    - sharing through QR codes
 */

#[derive(InputObject)]
pub struct DiveSessionInputData {
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub session_name: Option<String>,
}

#[derive(AsChangeset, InputObject, Clone)]
#[table_name = "dive_sessions"]
pub struct DiveSessionModificationData {
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub session_name: Option<String>,

    pub session_id: Uuid,
    pub is_active: Option<bool>,
}

#[derive(Insertable, InputObject)]
#[table_name = "dive_sessions"]
pub struct DiveSessionCreationData {
    pub session_id: Uuid,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub session_name: Option<String>,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_active: bool,
}

// Matches the database object 1:1
#[derive(Queryable, SimpleObject)]
#[graphql(complex)]
pub struct DiveSessionQueryData {
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub session_name: Option<String>,

    pub user_id: Uuid,

    pub id: i32,
    pub session_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_active: bool,
    pub deleted_at: Option<NaiveDateTime>,
    pub deleted_by: Option<Uuid>,
}

#[ComplexObject]
impl DiveSessionQueryData {
    async fn dives(
        &self,
        ctx: &Context<'_>,
        db_query_dto: Option<DBQueryObject>,
        // this needs to be mut
        mut dive_query: Option<DiveQueryInput>,
    ) -> FieldResult<Vec<DiveQueryData>> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

        let my_id = self.user_id;

        let dives = web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_dives_by_session(&mut conn, my_id, dive_query, db_query_dto)
        })
        .await
        .expect("error in dive sessions web::block")
        .expect("error in another loading dive sessions");

        Ok(dives)
    }
}

#[derive(InputObject)]
pub struct DiveSessionQueryInput {
    pub session_id: Option<Uuid>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub session_name: Option<String>,
    pub is_active: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
