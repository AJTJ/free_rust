use crate::{actions::get_dives_by_session, graphql_schema::DbPool, schema::dive_sessions};
use actix_web::web;
use async_graphql::{ComplexObject, Context, FieldResult, InputObject, SimpleObject};
use chrono::NaiveDateTime;
use uuid::Uuid;

use super::{
    dive_dto::{Dive, DiveFilter},
    query_dto::QueryParams,
};

#[derive(InputObject)]
pub struct DiveSessionInput {
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub session_name: Option<String>,
}

#[derive(AsChangeset, InputObject, Clone)]
#[diesel(table_name = dive_sessions)]
pub struct DiveSessionUpdate {
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub session_name: Option<String>,

    pub id: Uuid,
    pub is_active: Option<bool>,
}

#[derive(Insertable)]
#[diesel(table_name = dive_sessions)]
pub struct DiveSessionCreation {
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub session_name: Option<String>,

    pub user_id: Uuid,

    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_active: bool,
}

// Matches the database object 1:1
#[derive(Queryable, SimpleObject, Clone)]
// #[graphql(complex)]
pub struct DiveSession {
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub session_name: Option<String>,

    #[graphql(skip)]
    pub user_id: Uuid,

    // default data
    #[graphql(derived(into = "ID"))]
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
impl DiveSession {
    async fn dives(
        &self,
        ctx: &Context<'_>,
        db_query_dto: Option<QueryParams>,
        // this needs to be mut
        dive_query: Option<DiveFilter>,
    ) -> FieldResult<Vec<Dive>> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

        let session_id = self.id;

        let dives = web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_dives_by_session(&mut conn, session_id, dive_query, db_query_dto)
        })
        .await
        .expect("error in dive sessions web::block")
        .expect("error in another loading dive sessions");

        Ok(dives)
    }
}

#[derive(InputObject, Clone)]
pub struct DiveSessionFilter {
    pub session_id: Option<Uuid>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub session_name: Option<String>,
    pub is_active: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
