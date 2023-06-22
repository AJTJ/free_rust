use crate::actions::{get_user_id_from_token_and_session, get_user_session_data};
use crate::diesel::ExpressionMethods;
use crate::dto::dive_session_dto::{DiveSession, DiveSessionCreation, DiveSessionInput};
use crate::dto::form_dto::{Form, FormCreation, FormInput};
use crate::dto::form_field_dto::FormFieldCreation;
use crate::errors::BigError;
use crate::graphql_schema::DbPool;
use crate::helpers::form_helper::FormStructure;
use crate::helpers::token_helpers::get_cookie_from_token;
use actix_web::web;
use async_graphql::Context;
use chrono::Utc;
use diesel::RunQueryDsl;
use serde_json::json;

pub async fn add_logger(
    ctx: &Context<'_>,
    form_input: FormInput,
) -> Result<FormStructure, BigError> {
    // ) -> i32 {
    let new_form = FormStructure::validate_form(&form_input.form_template)?;
    let current_stamp = Utc::now().naive_utc();
    let user_id = get_user_id_from_token_and_session(ctx).await?;

    let created_form = FormCreation {
        form_name: form_input.form_name,
        template_version: new_form.form_template_version,
        user_id,
        original_form_id: form_input.original_form_id,
        previous_form_id: form_input.previous_form_id,
        created_at: current_stamp,
        updated_at: current_stamp,
        is_active: true,
    };

    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

    use crate::schema::forms::dsl::forms;
    let new_form = web::block(move || {
        let mut conn = pool_ctx.get().unwrap();
        let insert_response = diesel::insert_into(forms)
            .values(&created_form)
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

    new_form
}
