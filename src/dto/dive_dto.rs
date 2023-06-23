use actix_web::web;
use async_graphql::{ComplexObject, Context, InputObject, SimpleObject};
use chrono::{NaiveDateTime, NaiveTime};
use uuid::Uuid;

use crate::{
    actions::get_dive_session_by_id, errors::BigError, graphql_schema::DbPool, schema::dives,
};

use super::{dive_session_dto::DiveSession, query_dto::QueryParams};

#[derive(InputObject)]
pub struct DiveInput {
    pub discipline_type: Option<String>,
    pub depth: Option<f64>,
    pub distance: Option<f64>,
    pub dive_time: Option<i64>,
    pub dive_name: Option<String>,
}

#[derive(AsChangeset, InputObject, Clone)]
#[diesel(table_name = dives)]
pub struct DiveUpdate {
    pub discipline_type: Option<String>,
    pub depth: Option<f64>,
    pub distance: Option<f64>,
    pub dive_time: Option<i64>,
    pub dive_name: Option<String>,

    pub id: Uuid,
    pub is_active: Option<bool>,
}

#[derive(Insertable)]
#[diesel(table_name = dives)]
pub struct DiveCreation {
    pub discipline_type: Option<String>,
    pub depth: Option<f64>,
    pub distance: Option<f64>,
    pub dive_time: Option<i64>,
    pub dive_name: Option<String>,

    pub session_id: Uuid,
    pub user_id: Uuid,
    // partial default data
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_active: bool,
}

// Matches the database object order 1:1
#[derive(Queryable, SimpleObject)]
pub struct Dive {
    pub discipline_type: Option<String>,
    pub depth: Option<f64>,
    pub distance: Option<f64>,
    pub dive_time: Option<i64>,
    pub dive_name: Option<String>,

    #[graphql(skip)]
    pub session_id: Uuid,
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
impl Dive {
    async fn dive_session(
        &self,
        ctx: &Context<'_>,
        query_params: Option<QueryParams>,
    ) -> Result<DiveSession, BigError> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

        let session_id = self.session_id;
        web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_dive_session_by_id(&mut conn, &session_id, query_params).map(DiveSession::from)
        })
        .await
        .map_err(|e| BigError::ActixBlockingError { source: e })?
        .map_err(|e| BigError::DieselQueryError { source: e })
    }
}

#[derive(InputObject)]
pub struct DiveFilter {
    pub discipline_type: Option<String>,
    pub depth: Option<f64>,
    pub distance: Option<f64>,
    pub dive_time: Option<NaiveTime>,
    pub dive_name: Option<String>,
    pub dive_session: Option<Uuid>,
    pub user_id: Option<Uuid>,

    pub id: Option<Uuid>,
    pub is_active: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
