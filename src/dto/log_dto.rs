use crate::{actions::get_log_entries_by_log, errors::BigError, graphql_schema::DbPool};
use actix_web::web;
use async_graphql::{ComplexObject, Context, Enum, FieldResult, OutputType, SimpleObject};
use chrono::NaiveDateTime;
use uuid::Uuid;

use super::db_query_dto::DBQueryObject;

// This one needs to match 1:1
#[derive(Queryable, SimpleObject, Debug)]
#[graphql(complex)]
pub struct LogData {
    pub log_name: Option<String>,
    pub session_id: Option<Uuid>,
    pub logger_used: Uuid,
    pub user_id: Uuid,

    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_active: bool,
    pub deleted_at: Option<NaiveDateTime>,
    pub deleted_by: Option<Uuid>,
}

#[ComplexObject]
impl LogData {
    async fn log_entries(
        &self,
        ctx: &Context<'_>,
        db_query_dto: Option<DBQueryObject>,
    ) -> Result<Vec<LogEntryData>, BigError> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

        let log_id = self.id;

        web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_log_entries_by_log(&mut conn, &log_id, db_query_dto)
        })
        .await
        .map_err(|e| BigError::WebBlocking { source: e })
        .unwrap()
        .map_err(|e| BigError::QueryError { source: e })
    }
}

// LOG ENTRIES

// This one needs to match 1:1
#[derive(Queryable, SimpleObject, Debug)]
pub struct LogEntryData {
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
