use crate::{
    actions::get_logger_by_id, errors::BigError, graphql_schema::DbPool, schema::logger_entries,
};
use actix_web::web;
use async_graphql::{ComplexObject, Context, SimpleObject};
use chrono::NaiveDateTime;
use uuid::Uuid;

use super::logger_dto::Logger;

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
#[graphql(complex)]
pub struct LoggerEntry {
    pub item_order: Option<i32>,
    pub field_name: String,
    pub category_name: String,
    pub input_type: String,

    #[graphql(skip)]
    pub logger_id: Uuid,
    #[graphql(skip)]
    pub user_id: Uuid,

    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_active: bool,
    pub deleted_at: Option<NaiveDateTime>,
    pub deleted_by: Option<Uuid>,
}

#[ComplexObject]
impl LoggerEntry {
    async fn log(&self, ctx: &Context<'_>) -> Result<Logger, BigError> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

        let logger_id = self.logger_id;
        web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_logger_by_id(&mut conn, logger_id).map(Logger::from)
        })
        .await
        .map_err(|e| BigError::BlockingError { source: e })?
        .map_err(|e| BigError::DieselQueryError { source: e })
    }
}
