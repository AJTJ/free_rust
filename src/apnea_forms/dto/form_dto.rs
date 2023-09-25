// NOTE: Reports are now simply stored on the Session

use crate::{apnea_forms::forms_interface::FormResponse, schema::forms};

use async_graphql::{InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use serde_json::Value;
use uuid::Uuid;

#[derive(InputObject)]
pub struct FormDetails {
    pub form_name: String,
    pub original_form_id: Option<Uuid>,
    pub previous_form_id: Option<Uuid>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = forms)]
pub struct FormCreation {
    pub form_name: String,
    pub form_data: Value,

    // relationships
    pub user_id: Uuid,
    pub original_form_id: Option<Uuid>,
    pub previous_form_id: Option<Uuid>,

    // partial default data
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
}

// This one needs to match 1:1
#[derive(Queryable, SimpleObject, Clone, Debug)]
// #[graphql(complex)]
pub struct Form {
    pub form_name: String,
    pub form_data: FormResponse,
    // relationship data
    #[graphql(skip)]
    pub user_id: Uuid,
    #[graphql(skip)]
    pub original_form_id: Option<Uuid>,
    #[graphql(skip)]
    pub previous_form_id: Option<Uuid>,

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
