use crate::{actions::get_logger_entries_by_logger, errors::BigError, graphql_schema::DbPool};
use actix_web::web;
use async_graphql::{ComplexObject, Context, Enum, FieldResult, OutputType, SimpleObject};
use chrono::NaiveDateTime;
use uuid::Uuid;

use super::db_query_dto::DBQueryParams;

// This one needs to match 1:1
#[derive(Queryable, SimpleObject, Debug)]
#[graphql(complex)]
pub struct Logger {
    pub logger_name: String,
    pub user_id: Uuid,

    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_active: bool,
    pub deleted_at: Option<NaiveDateTime>,
    pub deleted_by: Option<Uuid>,
}

#[ComplexObject]
impl Logger {
    async fn logger_entries(
        &self,
        ctx: &Context<'_>,
        db_query_dto: Option<DBQueryParams>,
    ) -> Result<Vec<LoggerEntry>, BigError> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

        let logger_id = self.id;
        let user_id = self.user_id;

        web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_logger_entries_by_logger(&mut conn, &logger_id, &user_id, db_query_dto)
        })
        .await
        .map_err(|e| BigError::BlockingError { source: e })
        .unwrap()
        .map_err(|e| BigError::DieselQueryError { source: e })
    }
}

// LOGGER ENTRIES

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
