use crate::{
    actions::get_log_entries_by_log, errors::BigError, graphql_schema::DbPool,
    helpers::form_helper::Form,
};
use actix_web::web;
use async_graphql::{ComplexObject, Context, InputObject, SimpleObject, ID};
use chrono::NaiveDateTime;
use uuid::Uuid;

use super::{
    log_entries::{LogEntry, LogEntryOutput},
    query_dto::QueryParams,
};

#[derive(InputObject)]
pub struct LogInput {
    pub log_name: String,
    pub session_id: ID,
    pub logger_used: ID,
    pub completed_form: Form,
}

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

impl From<Log> for LogOutput {
    fn from(x: Log) -> Self {
        LogOutput {
            log_name: x.log_name,
            session_id: x.session_id,
            logger_used: x.logger_used,
            user_id: x.user_id,
            id: x.id,
            created_at: x.created_at,
            updated_at: x.updated_at,
            is_active: x.is_active,
            deleted_at: x.deleted_at,
            deleted_by: x.deleted_by,
        }
    }
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct LogOutput {
    pub log_name: Option<String>,

    #[graphql(skip)]
    pub session_id: Option<Uuid>,
    #[graphql(skip)]
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
