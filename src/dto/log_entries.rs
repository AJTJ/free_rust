use super::log_dto::LogOutput;
use actix_web::web;
use async_graphql::{ComplexObject, Context, InputObject, SimpleObject, ID};
use chrono::{NaiveDateTime, NaiveTime};
use uuid::Uuid;

use crate::{
    actions::{get_dive_session_by_id, get_log_by_id},
    errors::BigError,
    graphql_schema::DbPool,
    schema::dives,
};

use super::{dive_session_dto::DiveSessionOutput, query_dto::QueryParams};

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

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct LogEntryOutput {
    pub item_order: Option<i32>,
    pub category_type: String,
    pub input_type: String,
    pub input_value: Option<String>,

    #[graphql(skip)]
    pub log_id: Uuid,
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
impl LogEntryOutput {
    async fn log(&self, ctx: &Context<'_>) -> Result<LogOutput, BigError> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

        let log_id = self.log_id;
        web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_log_by_id(&mut conn, log_id).map(LogOutput::from)
        })
        .await
        .map_err(|e| BigError::BlockingError { source: e })?
        .map_err(|e| BigError::DieselQueryError { source: e })
    }
}
