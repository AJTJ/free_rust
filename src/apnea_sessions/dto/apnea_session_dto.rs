use super::dive_dto::{Dive, DiveRetrievalData};
use crate::{
    apnea_forms::{
        dto::form_dto::Form,
        form_loader::FormLoader,
        forms_interface::{ReportRequest, ReportResponse},
    },
    apnea_sessions::dive_loader_by_session::DiveLoaderBySession,
    schema::apnea_sessions,
    utility::errors::BigError,
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use serde_json::Value;
use std::sync::Arc;
use uuid::Uuid;

#[derive(InputObject)]
pub struct ApneaSessionInput {
    pub report_data: ReportRequest,

    pub form_id: Uuid,
    pub original_form_id: Option<Uuid>,
    pub previous_session_id: Option<Uuid>,
}

#[derive(Insertable)]
#[diesel(table_name = apnea_sessions)]
pub struct ApneaSessionCreation {
    // TODO: figure out how to make this the type, directly
    pub report_data: Value,

    pub form_id: Uuid,
    pub original_form_id: Option<Uuid>,
    pub previous_session_id: Option<Uuid>,
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
    pub report_data: ReportResponse,

    // eventually I need to be able to filter the report by the jsonb data?

    // relationships data
    #[graphql(skip)]
    pub form_id: Uuid,
    #[graphql(skip)]
    pub original_form_id: Option<Uuid>,
    #[graphql(skip)]
    pub previous_session_id: Option<Uuid>,
    #[graphql(skip)]
    pub user_id: Uuid,

    // default data
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
    async fn form(&self, ctx: &Context<'_>) -> Result<Option<Form>, Arc<BigError>> {
        let form_response = ctx
            .data_unchecked::<DataLoader<FormLoader>>()
            .load_one(self.form_id)
            .await;

        form_response
    }

    // Note: I don't think this requires pagination just now. As there will only ever be so many dives per session.
    async fn dives(&self, ctx: &Context<'_>) -> Result<Option<Vec<Dive>>, Arc<BigError>> {
        ctx.data_unchecked::<DataLoader<DiveLoaderBySession>>()
            .load_one(DiveRetrievalData::Session(self.id))
            .await
    }
}

pub enum ApneaSessionRetrievalData {
    Sessions(Vec<Uuid>),
    User(Uuid),
}
