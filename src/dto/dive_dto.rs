use actix_web::web;
use async_graphql::{ComplexObject, Context, InputObject, SimpleObject, ID};
use chrono::{NaiveDateTime, NaiveTime};
use uuid::Uuid;

use crate::{
    actions::get_dive_session_by_id, errors::BigError, graphql_schema::DbPool, schema::dives,
};

use super::{dive_session_dto::DiveSessionOutput, query_dto::QueryParams};

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

#[derive(Insertable, InputObject)]
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

    pub session_id: Uuid,
    pub user_id: Uuid,

    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_active: bool,
    pub deleted_at: Option<NaiveDateTime>,
    pub deleted_by: Option<Uuid>,
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct DiveOutput {
    pub discipline_type: Option<String>,
    pub depth: Option<f64>,
    pub distance: Option<f64>,
    pub dive_time: Option<i64>,
    pub dive_name: Option<String>,

    #[graphql(skip)]
    session_id: Uuid,
    #[graphql(skip)]
    user_id: Uuid,

    pub id: ID,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_active: bool,
}

#[ComplexObject]
impl DiveOutput {
    async fn dive_session(
        &self,
        ctx: &Context<'_>,
        query_params: Option<QueryParams>,
    ) -> Result<DiveSessionOutput, BigError> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

        let session_id = self.session_id;
        web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_dive_session_by_id(&mut conn, &session_id, query_params)
                .map(DiveSessionOutput::from)
        })
        .await
        .map_err(|e| BigError::BlockingError { source: e })?
        .map_err(|e| BigError::DieselQueryError { source: e })
    }
}

impl From<Dive> for DiveOutput {
    fn from(d: Dive) -> Self {
        DiveOutput {
            discipline_type: d.discipline_type,
            depth: d.depth,
            distance: d.distance,
            dive_time: d.dive_time,
            dive_name: d.dive_name,

            session_id: d.session_id,
            user_id: d.user_id,

            id: d.id.into(),
            created_at: d.created_at,
            updated_at: d.updated_at,
            is_active: d.is_active,
        }
    }
}

#[derive(InputObject)]
pub struct DiveFilter {
    pub dive_id: Option<Uuid>,
    pub discipline_type: Option<String>,
    pub depth: Option<f64>,
    pub distance: Option<f64>,
    pub dive_time: Option<NaiveTime>,
    pub dive_name: Option<String>,
    pub dive_session: Option<Uuid>,
    pub user_id: Option<Uuid>,

    pub is_active: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
