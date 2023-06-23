use crate::actions::get_user_id_from_token_and_session;
use crate::dive_forms::form_helper::{FormStructure, FormStructureOutput};
use crate::dto::completed_form_dto::{
    CompletedForm, CompletedFormCreation, CompletedFormInput, CompletedFormOutput,
};
use crate::dto::completed_form_field_dto::{CompletedFormField, CompletedFormFieldCreation};
use crate::errors::BigError;
use crate::graphql_schema::DbPool;

use actix_web::web;
use async_graphql::Context;
use chrono::Utc;
use diesel::RunQueryDsl;

pub async fn insert_completed_form(
    ctx: &Context<'_>,
    completed_form_input: CompletedFormInput,
) -> Result<CompletedFormOutput, BigError> {
    let validated_completed_form =
        FormStructure::validate_form(&completed_form_input.form_structure)?;

    let current_stamp = Utc::now().naive_utc();
    let user_id = get_user_id_from_token_and_session(ctx).await?;

    let created_completed_form = CompletedFormCreation {
        completed_form_name: completed_form_input.completed_form_name,
        original_form_id: completed_form_input.original_form_id,
        previous_completed_form_id: completed_form_input.previous_completed_form_id,
        form_id: completed_form_input.form_id,
        session_id: completed_form_input.session_id,
        user_id,
        created_at: current_stamp,
        updated_at: current_stamp,
        is_active: true,
    };

    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

    use crate::schema::completed_forms::dsl::completed_forms;
    let new_created_form_from_db = web::block(move || {
        let mut conn = pool_ctx.get().unwrap();
        let insert_response = diesel::insert_into(completed_forms)
            .values(&created_completed_form)
            .get_result::<CompletedForm>(&mut conn);

        insert_response
    })
    .await
    .map_err(|e| BigError::ActixBlockingError { source: e })?
    .map_err(|e| BigError::DieselInsertError { source: e })?;

    // Another approach... or both?!?
    let new_completed_form_fields: Vec<CompletedFormFieldCreation> = validated_completed_form
        .all_fields
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let le = CompletedFormFieldCreation {
                item_order: Some(i.try_into().unwrap()),

                field_name: c.field_name.to_string(),
                field_value: c.field_value.clone(),
                category_name: c.category_name.to_string(),
                field_value_type: c.field_value_type.to_string(),

                completed_form_id: new_created_form_from_db.id,
                user_id,

                created_at: current_stamp,
                updated_at: current_stamp,
                is_active: true,
            };
            le
        })
        .collect();

    let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

    use crate::schema::completed_form_fields::dsl::completed_form_fields;
    let all_inserted_completed_form_fields = web::block(move || {
        let mut conn = pool_ctx.get().unwrap();
        let insert_response = diesel::insert_into(completed_form_fields)
            .values(&new_completed_form_fields)
            .get_results::<CompletedFormField>(&mut conn);

        insert_response
    })
    .await
    .map_err(|e| BigError::ActixBlockingError { source: e })?
    .map_err(|e| BigError::DieselInsertError { source: e })?;

    Ok(CompletedFormOutput {
        form: new_created_form_from_db,
        fields: all_inserted_completed_form_fields,
        form_structure: FormStructureOutput::from(validated_completed_form),
    })
}
