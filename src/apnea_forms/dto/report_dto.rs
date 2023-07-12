use crate::{apnea_forms::helpers::FormResponse, schema::reports};

use async_graphql::{InputObject, OneofObject, SimpleObject};
use chrono::{DateTime, Utc};
use serde_json::Value;
use uuid::Uuid;

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
