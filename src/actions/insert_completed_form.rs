use crate::actions::{get_user_id_from_token_and_session, get_user_session_data};
use crate::diesel::ExpressionMethods;
use crate::dto::completed_form_dto::{CompletedFormCreation, CompletedFormInput};
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

pub async fn insert_completed_form(
    ctx: &Context<'_>,
    form_input: CompletedFormInput,
) -> Result<FormStructure, BigError> {
    let converted_input_form = FormStructure::from_input(form_input.form_template)?;
    let validated_completed_form = FormStructure::validate_form(&converted_input_form)?;
    let current_stamp = Utc::now().naive_utc();
    let user_id = get_user_id_from_token_and_session(ctx).await?;

    let new_logger = CompletedFormCreation {
        completed_form_name: form_input.completed_form_name,
        original_form_id: form_input.original_form_id,
        previous_completed_form_id: form_input.previous_completed_form_id,
        session_id: todo!(),
        user_id,
        id: todo!(),
        created_at: todo!(),
        updated_at: todo!(),
        is_active: todo!(),
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
        .all_inputs
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
