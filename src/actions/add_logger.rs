use crate::actions::get_user_id_from_token_and_session;
use crate::dto::loggers_dto::{Logger, LoggerCreation, LoggerInput};
use crate::errors::BigError;
use crate::graphql_schema::DbPool;

use actix_web::web;
use async_graphql::Context;
use chrono::Utc;
use diesel::RunQueryDsl;

pub async fn add_logger(ctx: &Context<'_>, logger_data: LoggerInput) -> Result<Logger, BigError> {
    let current_stamp = Utc::now().naive_utc();
    let user_id = get_user_id_from_token_and_session(ctx).await?;
    let new_logger = LoggerCreation {
        logger_name: logger_data.logger_name,
        user_id,
        created_at: current_stamp,
        updated_at: current_stamp,
        is_active: true,
    };

    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

    use crate::schema::loggers::dsl::loggers;

    web::block(move || {
        let mut conn = pool_ctx.get().unwrap();
        diesel::insert_into(loggers)
            .values(&new_logger)
            .get_result::<Logger>(&mut conn)
    })
    .await
    .map_err(|e| BigError::BlockingError { source: e })?
    .map_err(|e| BigError::DieselInsertError { source: e })
}
