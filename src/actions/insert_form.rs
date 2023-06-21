use crate::actions::{get_user_id_from_token_and_session, get_user_session_data};
use crate::diesel::ExpressionMethods;
use crate::dto::dive_session_dto::{DiveSession, DiveSessionCreation, DiveSessionInput};
use crate::dto::form_dto::{Form, FormCreation, FormInput};
use crate::dto::form_field_dto::FormFieldCreation;
use crate::errors::BigError;
use crate::graphql_schema::DbPool;
use crate::helpers::form_helper::{FormStructure, UserFormInput};
use crate::helpers::token_helpers::get_cookie_from_token;
use actix_web::web;
use async_graphql::Context;
use chrono::Utc;
use diesel::RunQueryDsl;
use serde_json::json;

pub async fn add_logger(
    ctx: &Context<'_>,
    logger_data: FormInput,
    user_form_input: UserFormInput,
) -> Result<FormStructure, BigError> {
    // ) -> i32 {
    let new_form = FormStructure::validate_form(user_form_input);
    let current_stamp = Utc::now().naive_utc();
    let user_id = get_user_id_from_token_and_session(ctx).await?;

    /*
    TODO: How should the form be stored?
    What are the implications for logs made, and if the form changes?
    What about data analysis? Analyzing json docs isn't as easy as looking at database values, but it's also not that hard.

    If the user has goals, then we are going to want to query the database for log data
     */

    let new_logger = FormCreation {
        form_name: logger_data.logger_name,
        user_id,
        created_at: current_stamp,
        updated_at: current_stamp,
        is_active: true,
    };

    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

    use crate::schema::loggers::dsl::loggers;
    let new_logger = web::block(move || {
        let mut conn = pool_ctx.get().unwrap();
        let insert_response = diesel::insert_into(loggers)
            .values(&new_logger)
            .get_result::<FormStructure>(&mut conn);

        insert_response
    })
    .await
    .map_err(|e| BigError::BlockingError { source: e })?
    .map_err(|e| BigError::DieselInsertError { source: e });

    // Another approach... or both?!?
    let all_new_entries: Vec<FormFieldCreation> = new_form
        .all_fields
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let le = FormFieldCreation {
                item_order: Some(i.try_into().unwrap()),
                field_name: c.input_name.to_string(),
                category_name: c.category_name.to_string(),
                input_type: c.input_type.to_string(),
                logger_id: user_id,
                user_id,

                created_at: current_stamp,
                updated_at: current_stamp,
                is_active: true,
            };
            le
        })
        .collect();

    // now I could insert all these entries that are created...

    new_logger
}
