use crate::{actions::get_log_entries_by_log, errors::BigError, graphql_schema::DbPool};
use actix_web::web;
use async_graphql::{ComplexObject, Context, SimpleObject};
use chrono::NaiveDateTime;
use uuid::Uuid;

use super::{
    log_entries::{LogEntry, LogEntryOutput},
    query_dto::QueryParams,
};

// This one needs to match 1:1
#[derive(Queryable, SimpleObject, Debug)]
// #[graphql(complex)]
pub struct Log {
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

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct LogOutput {
    pub log_name: Option<String>,
    pub session_id: Option<Uuid>,
    pub logger_used: Uuid,

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
impl LogOutput {
    async fn log_entries(
        &self,
        ctx: &Context<'_>,
        db_query_dto: Option<QueryParams>,
    ) -> Result<Vec<LogEntryOutput>, BigError> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

        let log_id = self.id;

        web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_log_entries_by_log(&mut conn, &log_id, db_query_dto)
                .map(|v| v.into_iter().map(LogEntryOutput::from).collect())
        })
        .await
        .map_err(|e| BigError::BlockingError { source: e })?
        .map_err(|e| BigError::DieselQueryError { source: e })
    }
}
