use std::sync::Arc;

use crate::{
    apnea_forms::{form_loader::FormLoader, helpers::FormResponse},
    schema::reports,
    utility::errors::BigError,
};

use async_graphql::{
    dataloader::DataLoader, ComplexObject, Context, InputObject, OneofObject, SimpleObject,
};
use chrono::{DateTime, Utc};
use serde_json::Value;
use tracing::info;
use uuid::Uuid;

use super::form_dto::Form;

#[derive(InputObject)]
pub struct ReportDetails {
    pub form_id: Uuid,
    pub original_form_id: Option<Uuid>,
    pub previous_report_id: Option<Uuid>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = reports)]
pub struct ReportCreation {
    pub report_data: Value,

    pub form_id: Uuid,
    pub original_form_id: Option<Uuid>,
    pub previous_report_id: Option<Uuid>,
    pub session_id: Uuid,
    pub user_id: Uuid,

    // partial default data
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
}

// This one needs to match 1:1
#[derive(Queryable, SimpleObject, Clone)]
#[graphql(complex)]
pub struct Report {
    pub report_data: FormResponse,

    // relationships
    #[graphql(skip)]
    pub form_id: Uuid,
    #[graphql(skip)]
    pub original_form_id: Option<Uuid>,
    #[graphql(skip)]
    pub previous_report_id: Option<Uuid>,
    #[graphql(skip)]
    pub session_id: Uuid,
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
impl Report {
    async fn form(&self, ctx: &Context<'_>) -> Result<Option<Form>, Arc<BigError>> {
        let form_response = ctx
            .data_unchecked::<DataLoader<FormLoader>>()
            .load_one(self.form_id)
            .await;

        form_response
    }
}

#[derive(OneofObject, Clone, PartialEq, Eq, Hash)]
pub enum ReportRetrievalData {
    SessionId(Uuid),
    ReportId(Uuid),
}

#[derive(OneofObject, Clone, PartialEq, Eq, Hash)]
pub enum ReportsRetrievalData {
    SessionId(Uuid),
    UserId(Uuid),
}
