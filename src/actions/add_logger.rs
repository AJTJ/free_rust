use crate::actions::get_user_id_from_token_and_session;
use crate::dto::loggers_dto::{Logger, LoggerCreation, LoggerInput};
use crate::errors::BigError;
use crate::graphql_schema::DbPool;
use crate::helpers::form_helper::{FormTemplate, UserFormInput};

use actix_web::web;
use async_graphql::Context;
use chrono::Utc;
use diesel::RunQueryDsl;
use serde_json::json;

pub async fn add_logger(
    ctx: &Context<'_>,
    logger_data: LoggerInput,
    user_form_input: UserFormInput,
) -> Result<Logger, BigError> {
    let new_form = FormTemplate::validate_form(user_form_input);
    let current_stamp = Utc::now().naive_utc();
    let user_id = get_user_id_from_token_and_session(ctx).await?;

    // TODO: How should the form be stored?
    // What are the implications for logs made, and if the form changes?
    // What about data analysis? Analyzing json docs isn't as easy as looking at database values, but it's also not that hard.

    let new_logger = LoggerCreation {
        logger_name: logger_data.logger_name,
        user_id,
        logger_fields: json!(new_form),
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
