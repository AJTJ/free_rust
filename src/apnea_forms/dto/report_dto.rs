use crate::{apnea_forms::helpers::FormOutput, schema::reports};

use async_graphql::{InputObject, OneofObject, SimpleObject};
use chrono::{DateTime, Utc};
use serde_json::Value;
use uuid::Uuid;

#[derive(InputObject)]
pub struct ReportDetailsInput {
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
    pub report_data: FormOutput,
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

#[derive(OneofObject)]
pub enum ReportRetrievalData {
    ReportId(Uuid),
    SessionId(Uuid),
}

#[derive(OneofObject)]
pub enum ReportsRetrievalData {
    UserId(Uuid),
    ReportIds(Vec<Uuid>),
}

// #[derive(InputObject)]
// pub struct ReportOutput {
//     pub form_id: Uuid,
//     pub original_form_id: Option<Uuid>,
//     pub previous_report_id: Option<Uuid>,
//     pub session_id: Uuid,
//     pub user_id: Uuid,
// }

// impl From<ReportDetailsInput> for ReportOutput {
//     fn from(value: ReportDetailsInput) -> Self {
//         ReportOutput {
//             form_id: value.form_id,
//             original_form_id: value.original_form_id,
//             previous_report_id: value.previous_report_id,
//             session_id: value.session_id,
//             user_id: value.user_id,
//         }
//     }
// }
