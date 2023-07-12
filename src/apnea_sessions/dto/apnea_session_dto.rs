use super::dive_dto::{Dive, DiveRetrievalData};
use crate::{
    apnea_forms::{
        dto::report_dto::{Report, ReportsRetrievalData},
        helpers::FormRequest,
        reports_loader::ReportLoader,
    },
    apnea_sessions::actions::get_dives,
    graphql_schema::DbPool,
    schema::apnea_sessions,
    utility::errors::BigError,
};
use actix_web::web;
use async_graphql::{
    dataloader::DataLoader, ComplexObject, Context, FieldResult, InputObject, SimpleObject,
};
use chrono::{DateTime, Utc};
use std::sync::Arc;
use uuid::Uuid;

#[derive(InputObject)]
pub struct ApneaSessionInput {
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub session_name: Option<String>,
    pub session_report: Option<FormRequest>,
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
    async fn report(&self, ctx: &Context<'_>) -> Result<Option<Report>, Arc<BigError>> {
        ctx.data_unchecked::<DataLoader<ReportLoader>>()
            .load_one(ReportsRetrievalData::SessionId(self.id))
            .await
    }

    // Note: I don't think this requires pagination just now. As there will only ever be so many dives per session.
    async fn dives(&self, ctx: &Context<'_>) -> FieldResult<Option<Vec<Dive>>> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

        let session_id = self.id;

        let dives = web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_dives(&mut conn, vec![DiveRetrievalData::Session(session_id)])
        })
        .await
        .map_err(|e| BigError::ActixBlockingError { source: e })??;

        Ok(dives)
    }
}

pub enum ApneaSessionRetrievalData {
    Sessions(Vec<Uuid>),
    User(Uuid),
}
