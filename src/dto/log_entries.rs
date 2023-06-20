use async_graphql::SimpleObject;
use chrono::NaiveDateTime;
use uuid::Uuid;

// This one needs to match 1:1
#[derive(Queryable, SimpleObject, Debug)]
pub struct LogEntry {
    pub item_order: Option<i32>,
    pub category_type: String,
    pub input_type: String,
    pub input_value: Option<String>,
    pub log_id: Uuid,
    pub user_id: Uuid,

    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_active: bool,
    pub deleted_at: Option<NaiveDateTime>,
    pub deleted_by: Option<Uuid>,
}
