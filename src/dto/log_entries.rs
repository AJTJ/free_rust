use crate::{actions::get_log_entries_by_log, errors::BigError, graphql_schema::DbPool};
use actix_web::web;
use async_graphql::{ComplexObject, Context, Enum, FieldResult, OutputType, SimpleObject};
use chrono::NaiveDateTime;
use uuid::Uuid;

use super::query_dto::QueryParams;

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
