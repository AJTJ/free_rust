use crate::actions::get_logger_entries_by_logger;
use crate::errors::BigError;
use crate::helpers::form_helper::Form;
use crate::{graphql_schema::DbPool, schema::loggers};
use actix_web::web;
use async_graphql::{ComplexObject, Context, InputObject, SimpleObject};
use chrono::NaiveDateTime;
use uuid::Uuid;

use super::logger_entries_dto::LoggerEntry;
use super::query_dto::QueryParams;

#[derive(InputObject)]
pub struct LoggerInput {
    pub logger_name: String,
    pub form_template: Form,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = loggers)]
pub struct LoggerCreation {
    pub logger_name: String,
    pub user_id: Uuid,
    pub logger_fields: serde_json::Value,
    // pub logger_fields: String,
    // partial default data
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_active: bool,
}

// This one needs to match 1:1
#[derive(Queryable, SimpleObject)]
#[graphql(complex)]
pub struct Logger {
    pub logger_name: String,
    pub logger_fields: serde_json::Value,
    // relationship data
    #[graphql(skip)]
    pub user_id: Uuid,
    pub logger_family_id: Uuid,
    pub version: u32,
    // default data
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_active: bool,
    pub deleted_at: Option<NaiveDateTime>,
    pub deleted_by: Option<Uuid>,
}

#[ComplexObject]
impl Logger {
    pub async fn logger_entries(
        &self,
        ctx: &Context<'_>,
        db_query_dto: Option<QueryParams>,
    ) -> Result<Vec<LoggerEntry>, BigError> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

        let logger_id = self.id;
        let user_id = self.user_id;

        web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_logger_entries_by_logger(&mut conn, &logger_id, &user_id, db_query_dto)
                .map(|v| v.into_iter().map(LoggerEntry::from).collect())
        })
        .await
        .map_err(|e| BigError::BlockingError { source: e })
        .unwrap()
        .map_err(|e| BigError::DieselQueryError { source: e })
    }
}
