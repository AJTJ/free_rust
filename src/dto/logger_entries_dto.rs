use crate::schema::logger_entries;
use async_graphql::SimpleObject;
use chrono::NaiveDateTime;
use uuid::Uuid;

// LOGGER ENTRIES

#[derive(Insertable, Debug)]
#[diesel(table_name = logger_entries)]
pub struct LoggerEntryCreation {
    pub item_order: Option<i32>,
    pub field_name: String,
    pub category_name: String,
    pub input_type: String,
    pub logger_id: Uuid,
    pub user_id: Uuid,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_active: bool,
}

// This one needs to match 1:1
#[derive(Queryable, SimpleObject)]
pub struct LoggerEntry {
    pub item_order: Option<i32>,
    pub field_name: String,
    pub category_name: String,
    pub input_type: String,
    pub logger_id: Uuid,
    pub user_id: Uuid,

    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_active: bool,
    pub deleted_at: Option<NaiveDateTime>,
    pub deleted_by: Option<Uuid>,
}
