use super::dive_dto::{Dive, DiveFilter, DiveRetrievalData};
use crate::{
    apnea_forms::{
        actions::get_report::get_report,
        dto::report_dto::{Report, ReportRetrievalData},
        helpers::FormRequest,
    },
    apnea_sessions::actions::get_dives,
    graphql_schema::DbPool,
    schema::apnea_sessions,
    utility::{
        errors::{BigError, DieselQuerySnafu},
        gql::query_dto::QueryParams,
    },
};
use actix_web::web;
use async_graphql::{ComplexObject, Context, FieldResult, InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use snafu::ResultExt;
use uuid::Uuid;

#[derive(InputObject)]
pub struct ApneaSessionInput {
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub session_name: Option<String>,
    pub session_report: Option<FormRequest>,
}

#[derive(AsChangeset, InputObject, Clone)]
#[diesel(table_name = apnea_sessions)]
pub struct ApneaSessionUpdate {
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub session_name: Option<String>,

    pub id: Uuid,
    pub is_active: Option<bool>,
}

#[derive(Insertable)]
#[diesel(table_name = apnea_sessions)]
pub struct ApneaSessionCreation {
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub session_name: Option<String>,

    pub user_id: Uuid,

    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
}

// Matches the database object 1:1
#[derive(Queryable, SimpleObject, Clone, Debug)]
#[graphql(complex)]
pub struct ApneaSession {
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub session_name: Option<String>,

    #[graphql(skip)]
    pub user_id: Uuid,

    // default data
    #[graphql(derived(into = "ID"))]
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
    #[graphql(skip)]
    pub archived_at: Option<DateTime<Utc>>,
    #[graphql(skip)]
    pub archived_by: Option<Uuid>,
}

#[ComplexObject]
impl ApneaSession {
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
            get_dives(
                &mut conn,
                DiveRetrievalData::Session(session_id),
                dive_query,
                db_query_dto,
            )
        })
        .await
        .map_err(|e| BigError::ActixBlockingError { source: e })?
        .context(DieselQuerySnafu)?;

        Ok(dives)
    }

    async fn report(&self, ctx: &Context<'_>) -> Result<Option<Report>, BigError> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

        let session_id = self.id;

        let report = web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_report(&mut conn, ReportRetrievalData::SessionId(session_id))
        })
        .await
        .map_err(|e| BigError::ActixBlockingError { source: e })?
        .map_err(|e| BigError::DieselQueryError { source: e });

        report
    }
}

#[derive(InputObject, Clone)]
pub struct ApneaSessionFilter {
    pub session_id: Option<Uuid>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub session_name: Option<String>,
    pub is_active: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub enum ApneaSessionRetrievalData {
    Sessions(Vec<Uuid>),
    User(Uuid),
}
