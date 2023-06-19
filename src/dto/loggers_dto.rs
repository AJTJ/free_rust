use crate::actions::get_logger_entries_by_logger;
use crate::errors::BigError;
use crate::{actions::get_dive_sessions_by_user, graphql_schema::DbPool, schema::loggers};
use actix_web::web;
use async_graphql::{ComplexObject, Context, FieldResult, InputObject, SimpleObject};
use chrono::NaiveDateTime;
use uuid::Uuid;

use super::{
    db_query_dto::{self, DBQueryParams},
    dive_session_dto::{DiveSession, DiveSessionQueryParams},
};

// #[derive(AsChangeset, InputObject)]
// #[diesel(table_name = users)]
// pub struct FooUpdate {}

// // not sure this one is necessary
// #[derive(SimpleObject)]
// #[graphql(complex)]
// pub struct FooOutput {}

#[derive(InputObject)]

pub struct LoggerInput {
    pub logger_name: String,
}

#[derive(Insertable)]
#[diesel(table_name = loggers)]
pub struct LoggerCreation {
    pub logger_name: String,
    pub user_id: Uuid,
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
    pub user_id: Uuid,
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
